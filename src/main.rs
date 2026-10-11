mod pe_loader;
mod kernel32;
mod audio;
mod ui;

fn main() {
    println!("=== Iniciando 7HLE (Windows 7 Compatibility Layer) ===");
    
    // Inicializamos el subsistema de audio
    let _audio_manager = match audio::AudioManager::new() {
        Ok(am) => Some(am),
        Err(e) => {
            eprintln!("[Advertencia] No se pudo inicializar el audio: {}", e);
            None
        }
    };

    if let Err(e) = ui::run_ui() {
        eprintln!("Error crítico en 7HLE: {}", e);
    }
}