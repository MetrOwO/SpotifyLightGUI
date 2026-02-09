use std::sync::atomic::{AtomicBool, Ordering};
use tao::event_loop::{ControlFlow, EventLoop};
use tray_icon::{
    menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem},
    TrayIconBuilder,
};

static IS_PLAYING: AtomicBool = AtomicBool::new(false);

fn main() {
    let event_loop = EventLoop::new();

    // Creazione dei bottoni
    let prev_item = MenuItem::new("󰒮 Indietro", true, None);
    let play_pause_item = MenuItem::new("󰐊 Play", true, None);
    let next_item = MenuItem::new("󰒭 Avanti", true, None);
    let quit_item = MenuItem::new("Esci", true, None);

    let tray_menu = Menu::new();
    let _ = tray_menu.append_items(&[
        &prev_item,
        &play_pause_item,
        &next_item,
        &PredefinedMenuItem::separator(),
        &quit_item,
    ]);

    // Caricamento icona
    let icon = load_icon(std::path::Path::new("assets/music.png"));

    let _tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_icon(icon)
        .build()
        .unwrap();

    event_loop.run(move |_event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Ok(event) = MenuEvent::receiver().try_recv() {
            if event.id == prev_item.id() {
                println!("Tasto indietro > indietro");
            } 
            else if event.id == next_item.id() {
                println!("Tasto avanti > avanti");
            } 
            else if event.id == play_pause_item.id() {
                let currently_playing = IS_PLAYING.load(Ordering::SeqCst);
                if currently_playing {
                    println!("Tasto pausa > pausa");
                    play_pause_item.set_text("󰐊 Play");
                    IS_PLAYING.store(false, Ordering::SeqCst);
                } else {
                    println!("Tasto riproduci > play");
                    play_pause_item.set_text("󰏤 Pausa");
                    IS_PLAYING.store(true, Ordering::SeqCst);
                }
            } 
            else if event.id == quit_item.id() {
                *control_flow = ControlFlow::Exit;
            }
        }
    });
}

fn load_icon(path: &std::path::Path) -> tray_icon::Icon {
    let image = image::open(path)
        .expect("Assicurati che l'icona sia in assets/music.png")
        .into_rgba8();
    let (width, height) = image.dimensions();
    tray_icon::Icon::from_rgba(image.into_raw(), width, height).unwrap()
}