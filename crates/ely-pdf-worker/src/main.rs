//! The PDF worker process: `serve` on stdin and stdout.

fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::new().filter_or("ELY_LOG", "info")).init();
    ely_pdf_worker::serve(std::io::stdin().lock(), std::io::stdout().lock())
}
