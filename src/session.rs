use librespot_core::{
    authentication::Credentials,
    cache::Cache,
    config::SessionConfig,
    session::Session,
};
use std::path::PathBuf;

pub async fn init_session() -> Result<Session, Box<dyn std::error::Error>> {
    // Cartella cache per credenziali
    let cache = Cache::new(Some(PathBuf::from("./cache")), None, None, None)?;

    // Config sessione
    let session_config = SessionConfig::default();

    // Login via pairing (nessuna credenziale diretta)
    let (session, _credentials) = Session::connect(
        session_config,
        None,       // None -> pairing
        Some(cache),
        false,
    ).await?;

    println!("✅ Spotify session initialized.");
    Ok(session)
}
