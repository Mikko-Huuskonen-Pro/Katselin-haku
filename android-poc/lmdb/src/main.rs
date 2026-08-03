//! Katselin Android PoC 4a — heed/LMDB on bionic (POSIX mutex, not posix-sem).
//! Stdlib HTTP only; same heed serde profile as milli.

use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use heed::types::{SerdeJson, Str};
use heed::{Database, Env, EnvOpenOptions};

const ADDR: &str = "127.0.0.1:17702";
const DEFAULT_DB: &str = "/data/local/tmp/poc4a-db";

type KvDb = Database<Str, SerdeJson<String>>;

struct Store {
    env: Env,
    db: KvDb,
}

fn open_store(path: &Path) -> Result<Store, Box<dyn std::error::Error>> {
    std::fs::create_dir_all(path)?;
    // SAFETY: single Env per process path; PoC only.
    let env = unsafe {
        EnvOpenOptions::new()
            .map_size(10 * 1024 * 1024)
            .max_dbs(2)
            .open(path)?
    };
    let mut wtxn = env.write_txn()?;
    let db: KvDb = env.create_database(&mut wtxn, Some("poc4a"))?;
    wtxn.commit()?;
    Ok(Store { env, db })
}

fn put(store: &Store, key: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut wtxn = store.env.write_txn()?;
    store.db.put(&mut wtxn, key, &value.to_string())?;
    wtxn.commit()?;
    Ok(())
}

fn get(store: &Store, key: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let rtxn = store.env.read_txn()?;
    Ok(store.db.get(&rtxn, key)?)
}

/// Write, read, drop env, reopen, read again — proves persistence on device FS.
fn smoke_persistence(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    {
        eprintln!("Katselin PoC4a: opening for write…");
        let store = open_store(path).map_err(|e| format!("open(write): {e}"))?;
        eprintln!("Katselin PoC4a: putting…");
        put(&store, "hello", "android").map_err(|e| format!("put: {e}"))?;
        eprintln!("Katselin PoC4a: getting…");
        let v = get(&store, "hello").map_err(|e| format!("get: {e}"))?;
        if v.as_deref() != Some("android") {
            return Err(format!("first read mismatch: {v:?}").into());
        }
        eprintln!("Katselin PoC4a: write+read OK");
    }
    {
        eprintln!("Katselin PoC4a: reopening…");
        let store = open_store(path).map_err(|e| format!("open(reopen): {e}"))?;
        let v = get(&store, "hello").map_err(|e| format!("get(reopen): {e}"))?;
        if v.as_deref() != Some("android") {
            return Err(format!("reopen read mismatch: {v:?}").into());
        }
        eprintln!("Katselin PoC4a: reopen+read OK");
    }
    Ok(())
}

fn parse_query(path_and_query: &str) -> (String, Vec<(String, String)>) {
    let (path, query) = match path_and_query.split_once('?') {
        Some((p, q)) => (p.to_string(), q),
        None => (path_and_query.to_string(), ""),
    };
    let params = query
        .split('&')
        .filter(|s| !s.is_empty())
        .filter_map(|pair| {
            let (k, v) = pair.split_once('=')?;
            Some((k.to_string(), v.to_string()))
        })
        .collect();
    (path, params)
}

fn param<'a>(params: &'a [(String, String)], name: &str) -> Option<&'a str> {
    params.iter().find(|(k, _)| k == name).map(|(_, v)| v.as_str())
}

fn http_response(status: &str, body: &str) -> Vec<u8> {
    format!(
        "HTTP/1.1 {status}\r\nContent-Type: text/plain; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    )
    .into_bytes()
}

fn handle(store: &Store, request_line: &str) -> Vec<u8> {
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or("");
    let target = parts.next().unwrap_or("/");
    if method != "GET" && method != "POST" {
        return http_response("405 Method Not Allowed", "method");
    }
    let (path, params) = parse_query(target);
    match path.as_str() {
        "/health" => http_response("200 OK", "OK"),
        "/put" => {
            let key = match param(&params, "key") {
                Some(k) if !k.is_empty() => k,
                _ => return http_response("400 Bad Request", "missing key"),
            };
            let value = param(&params, "value").unwrap_or("");
            match put(store, key, value) {
                Ok(()) => http_response("200 OK", "PUT_OK"),
                Err(e) => http_response("500 Internal Server Error", &format!("put err: {e}")),
            }
        },
        "/get" => {
            let key = match param(&params, "key") {
                Some(k) if !k.is_empty() => k,
                _ => return http_response("400 Bad Request", "missing key"),
            };
            match get(store, key) {
                Ok(Some(v)) => http_response("200 OK", &v),
                Ok(None) => http_response("404 Not Found", "missing"),
                Err(e) => http_response("500 Internal Server Error", &format!("get err: {e}")),
            }
        },
        _ => http_response("404 Not Found", "not found"),
    }
}

fn main() {
    let db_path = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(DEFAULT_DB));

    eprintln!("Katselin PoC4a: db={}", db_path.display());
    if let Err(e) = smoke_persistence(&db_path) {
        eprintln!("Katselin PoC4a: smoke FAILED: {e}");
        std::process::exit(1);
    }

    let store = match open_store(&db_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Katselin PoC4a: open FAILED: {e}");
            std::process::exit(1);
        },
    };
    let store = Mutex::new(store);

    eprintln!("Katselin PoC4a: listening on http://{ADDR}");
    let listener = match TcpListener::bind(ADDR) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Katselin PoC4a: bind failed: {e}");
            std::process::exit(1);
        },
    };

    for stream in listener.incoming().flatten() {
        let mut stream = stream;
        let mut buf = [0u8; 2048];
        let n = stream.read(&mut buf).unwrap_or(0);
        let req = String::from_utf8_lossy(&buf[..n]);
        let first = req.lines().next().unwrap_or("");
        let store = store.lock().unwrap();
        let resp = handle(&store, first);
        drop(store);
        let _ = stream.write_all(&resp);
    }
}
