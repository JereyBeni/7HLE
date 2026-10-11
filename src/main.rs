mod pe_loader;
mod kernel32;
mod ui;

fn main() {
    println!("=== Iniciando 7HLE (Windows 7 Compatibility Layer) ===");
    
    // Lanzamos la interfaz gráfica nativa en Rust
    if let Err(e) = ui::run_ui() {
        eprintln!("Error crítico en 7HLE: {}", e);
    }
}