//! # **DMNTK** | Decision Model and Notation Toolkit

mod actions;
mod examples;

/// Main entrypoint of **DMNTK**.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
  actions::do_action().await
}
