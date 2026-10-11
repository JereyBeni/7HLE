mod pe_loader;
mod kernel32;
mod audio;
mod memory;
mod graphics;
mod ui;

fn main() {
    println!("=== Iniciando 7HLE (Windows 7 Compatibility Layer) ===");
    
    // Asignamos 1 GB de RAM virtual para el entorno (podes cambiar a 512 si querés)
    let _emulated_ram = memory::VirtualMemory::new(1024); 

    // Inicializamos subsistemas
    let _audio = audio::AudioManager::new().ok();
    let _gfx_d3d = graphics::GraphicsFramework::new(800, 600, "d3d9");
    let _gfx_gl = graphics::GraphicsFramework::new(800, 600, "opengl");

    _gfx_d3d.direct3d_create_device();
    _gfx_gl.opengl_wgl_create_context();

    if let Err(e) = ui::run_ui() {
        eprintln!("Error crítico en 7HLE: {}", e);
    }
}