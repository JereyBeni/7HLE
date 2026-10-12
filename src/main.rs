// ============================================================================
// 7HLE - Windows 7 & XP Compatibility Layer / Emulator
// ============================================================================

mod pe_loader;
mod kernel32;
mod audio;
mod memory;
mod graphics;
mod dx_wrapper;
mod xp_compat;
mod cpu;
mod ui;

fn main() {
    println!("==================================================");
    println!("  Starting seven_hle v1.1.0 (Win7 / WinXP Layer)  ");
    println!("==================================================");

    // 1. Inicialización y simulación detallada del CPU (Core 2 Duo / Intel VT activado)
    let cpu_manager = cpu::EmulatedCpuManager::new(true);
    cpu_manager.print_detailed_specs();
    
    // Probando una consulta de CPUID emulada (Leaf 1: Características)
    let (eax, ebx, ecx, edx) = cpu_manager.emulate_cpuid(1);
    println!("[Main CPUID] EAX: 0x{:08X}, EBX: 0x{:08X}, ECX: 0x{:08X}, EDX: 0x{:08X}", eax, ebx, ecx, edx);

    // 2. Inicialización de la Memoria Virtual (1 GB Heap)
    let mut vram = memory::VirtualMemoryManager::new(1024);
    let _sample_ptr = vram.virtual_alloc(
        0,
        4096,
        memory::MemoryAllocationType::MemCommit,
        memory::MemoryProtection::PageReadWrite,
    );
    vram.print_memory_stats();

    // 3. Capa de compatibilidad retro con Windows XP (NT 5.1)
    let xp_layer = xp_compat::XpCompatLayer::new();
    let (major, minor, build) = xp_layer.hook_get_version_ex();
    println!("[Main] Sistema emulado reportando versión: {}.{}.{}", major, minor, build);
    xp_layer.redirect_legacy_apis();

    // 4. Inicialización del subsistema gráfico y audio
    let _audio = audio::AudioManager::new().ok();
    let gfx_d3d = graphics::GraphicsFramework::new(1280, 720, "d3d9");
    gfx_d3d.direct3d_create_device();

    // 5. Lanzamiento de la Interfaz Gráfica (Iced UI)
    println!("[Main] Abriendo panel de configuración de seven_hle...");
    if let Err(e) = ui::run_ui() {
        eprintln!("[Main Error] Error en el loop de UI: {}", e);
    }
}