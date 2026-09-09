//! `serve` subcommand: minimal sync HTTP server (tiny_http, no async runtime)
//! exposing the same JSON shapes the one-shot subcommands print under
//! `--json`.
//!
//!   GET /lookup?q=...&limit=...     ranked query (members + class cards)
//!   GET /class?name=4D.File         one class card
//!   GET /member?name=File.exists    one member (resolves inherited names)
//!   GET /members?class=Entity&kind=function
//!   GET /returns?class=4D.FileHandle
//!   GET /health

use crate::model;
use std::collections::HashMap;
use tiny_http::{Header, Method, Response, Server};

pub struct ServeArgs {
    pub port: u16,
}

pub fn parse_args(args: &[String]) -> Result<ServeArgs, String> {
    let mut port: u16 = 8080;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--port" => {
                i += 1;
                let val = args.get(i).ok_or("--port requires a value")?;
                port = val
                    .parse()
                    .map_err(|_| format!("invalid --port value: {val}"))?;
            }
            other => return Err(format!("unexpected argument: {other}")),
        }
        i += 1;
    }
    Ok(ServeArgs { port })
}

pub fn run(args: ServeArgs) -> i32 {
    let addr = format!("0.0.0.0:{}", args.port);
    let server = match Server::http(&addr) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: failed to bind {addr}: {e}");
            return 1;
        }
    };
    eprintln!(
        "listening on http://{addr}  (GET /lookup?q=..., /class?name=..., /member?name=..., /members?class=..., /returns?class=..., /health)"
    );

    for request in server.incoming_requests() {
        handle_request(request);
    }
    0
}

fn handle_request(request: tiny_http::Request) {
    let method = request.method().clone();
    let url = request.url().to_string();

    if method != Method::Get {
        respond_plain(request, 405, "method not allowed");
        return;
    }

    let (path, query) = split_path_query(&url);
    let params = parse_query_string(query);

    match path {
        "/health" => respond_plain(request, 200, "ok"),
        "/lookup" => {
            let Some(q) = required(&params, "q") else {
                respond_plain(request, 400, "missing required query parameter: q");
                return;
            };
            let limit: usize = params
                .get("limit")
                .and_then(|s| s.parse().ok())
                .filter(|n| *n > 0)
                .unwrap_or(5);
            respond_serialized(request, &model::lookup(&q, limit));
        }
        "/class" => {
            let Some(name) = required(&params, "name").or_else(|| required(&params, "class"))
            else {
                respond_plain(request, 400, "missing required query parameter: name");
                return;
            };
            match model::class_card(&name) {
                Some(card) => respond_serialized(request, &card),
                None => respond_plain(request, 404, &format!("no such class: {name}")),
            }
        }
        "/member" => {
            let Some(name) = required(&params, "name") else {
                respond_plain(request, 400, "missing required query parameter: name");
                return;
            };
            match model::member_card(&name) {
                Some(card) => respond_serialized(request, &card),
                None => respond_plain(request, 404, &format!("no such member: {name}")),
            }
        }
        "/members" => {
            let Some(class) = required(&params, "class").or_else(|| required(&params, "name"))
            else {
                respond_plain(request, 400, "missing required query parameter: class");
                return;
            };
            let kind = params.get("kind").map(|k| model::normalize_kind(k));
            if let Some(k) = &kind {
                if !matches!(
                    k.as_str(),
                    "oop_function" | "oop_property" | "oop_constructor"
                ) {
                    respond_plain(
                        request,
                        400,
                        "invalid kind (expected function, property or constructor)",
                    );
                    return;
                }
            }
            match model::members_listing(&class, kind.as_deref()) {
                Some(listing) => respond_serialized(request, &listing),
                None => respond_plain(request, 404, &format!("no such class: {class}")),
            }
        }
        "/returns" => {
            let Some(class) = required(&params, "class").or_else(|| required(&params, "name"))
            else {
                respond_plain(request, 400, "missing required query parameter: class");
                return;
            };
            match model::returns_result(&class) {
                Some(result) => respond_serialized(request, &result),
                None => respond_plain(request, 404, &format!("no such class: {class}")),
            }
        }
        _ => respond_plain(request, 404, "not found"),
    }
}

fn required(params: &HashMap<String, String>, key: &str) -> Option<String> {
    params
        .get(key)
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

fn respond_serialized<T: serde::Serialize>(request: tiny_http::Request, value: &T) {
    match serde_json::to_string(value) {
        Ok(body) => respond_json(request, 200, &body),
        Err(e) => respond_plain(request, 500, &format!("serialization error: {e}")),
    }
}

fn split_path_query(url: &str) -> (&str, &str) {
    match url.split_once('?') {
        Some((p, q)) => (p, q),
        None => (url, ""),
    }
}

/// Minimal `application/x-www-form-urlencoded`-style query string parser:
/// splits on `&`/`=` and percent-decodes each key/value. Deliberately
/// hand-rolled (no `url`/`serde_urlencoded` crate) since it only needs to
/// handle a handful of known parameter names.
fn parse_query_string(query: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    if query.is_empty() {
        return map;
    }
    for pair in query.split('&') {
        let mut parts = pair.splitn(2, '=');
        let key = parts.next().unwrap_or("");
        let value = parts.next().unwrap_or("");
        if key.is_empty() {
            continue;
        }
        map.insert(percent_decode(key), percent_decode(value));
    }
    map
}

fn percent_decode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'+' => {
                out.push(b' ');
                i += 1;
            }
            b'%' if i + 2 < bytes.len() => {
                let hex = std::str::from_utf8(&bytes[i + 1..i + 3]).ok();
                if let Some(byte) = hex.and_then(|h| u8::from_str_radix(h, 16).ok()) {
                    out.push(byte);
                    i += 3;
                } else {
                    out.push(bytes[i]);
                    i += 1;
                }
            }
            b => {
                out.push(b);
                i += 1;
            }
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn respond_plain(request: tiny_http::Request, status: u16, body: &str) {
    let header = Header::from_bytes(&b"Content-Type"[..], &b"text/plain; charset=utf-8"[..])
        .expect("static header is valid");
    let response = Response::from_string(body)
        .with_status_code(status)
        .with_header(header);
    let _ = request.respond(response);
}

fn respond_json(request: tiny_http::Request, status: u16, body: &str) {
    let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json; charset=utf-8"[..])
        .expect("static header is valid");
    let response = Response::from_string(body)
        .with_status_code(status)
        .with_header(header);
    let _ = request.respond(response);
}
