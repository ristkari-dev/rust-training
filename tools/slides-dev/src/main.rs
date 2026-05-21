use anyhow::{Result, anyhow, bail};
use clap::Parser;
use slides_dev::{Resolved, mime_for, resolve};
use std::path::PathBuf;

#[derive(Parser)]
#[command(about = "Serve a lesson's reveal.js slides on localhost")]
struct Cli {
    /// Lesson name, e.g. "01-hello-rust"
    #[arg(long)]
    lesson: String,

    /// Repository root (default: current dir)
    #[arg(long, default_value = ".")]
    root: PathBuf,

    /// Port (default: 8000)
    #[arg(long, default_value_t = 8000)]
    port: u16,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let slides_dir = cli.root.join("lessons").join(&cli.lesson).join("slides");
    if !slides_dir.is_dir() {
        bail!("not a directory: {}", slides_dir.display());
    }
    let shared_dir = cli.root.join("shared");

    let addr = format!("0.0.0.0:{}", cli.port);
    let server =
        tiny_http::Server::http(&addr).map_err(|e| anyhow!("failed to bind {addr}: {e}"))?;
    println!(
        "serving {} on http://localhost:{}",
        slides_dir.display(),
        cli.port
    );

    for request in server.incoming_requests() {
        let url = request.url().to_string();
        match resolve(&url, &slides_dir, &shared_dir) {
            Resolved::Forbidden => {
                // Discard: if responding fails the client is already gone.
                let _ = request
                    .respond(tiny_http::Response::from_string("forbidden").with_status_code(403));
            }
            Resolved::File(path) => match std::fs::read(&path) {
                Ok(bytes) => {
                    let mime = mime_for(&path);
                    let header = tiny_http::Header::from_bytes(b"Content-Type" as &[u8], mime)
                        .map_err(|()| anyhow!("invalid header"))?;
                    // Discard: if responding fails the client is already gone.
                    let _ =
                        request.respond(tiny_http::Response::from_data(bytes).with_header(header));
                }
                Err(_) => {
                    // Discard: if responding fails the client is already gone.
                    let _ = request.respond(
                        tiny_http::Response::from_string("not found").with_status_code(404),
                    );
                }
            },
        }
    }
    Ok(())
}
