use librespot_playback::{player::Player, config::PlayerConfig};
use librespot_core::session::Session;

pub async fn init_player(session: Session) -> Result<Player, Box<dyn std::error::Error>> {
    let config = PlayerConfig {
        name: "SpotifyLightGUI".to_string(),
        ..Default::default()
    };

    let player = Player::new(config, session, None)?;
    Ok(player)
}

pub async fn play_track(player: &mut Player, track_uri: &str) -> Result<(), Box<dyn std::error::Error>> {
    player.load(track_uri, true, 0)?;
    Ok(())
}
