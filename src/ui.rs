// ============================================================================
// 7HLE - Subsistema de Ventana Gráfica Nativa (Winit + Softbuffer)
// ============================================================================

use std::num::NonZeroU32;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

pub fn run_ui() -> Result<(), String> {
    println!("[7HLE UI] Inicializando subsistema de ventana gráfica nativa...");

    // 1. Inicializar el loop de eventos de la ventana
    let event_loop = EventLoop::new().map_err(|e| e.to_string())?;
    let window = WindowBuilder::new()
        .with_title("seven_hle v1.1.0 - Windows 7 / XP Compatibility Layer")
        .with_inner_size(winit::dpi::LogicalSize::new(800, 600))
        .build(&event_loop)
        .map_err(|e| e.to_string())?;

    // 2. Configurar el contexto de gráficos por software (softbuffer)
    let context = softbuffer::Context::new(&window).map_err(|e| e.to_string())?;
    let mut surface = softbuffer::Surface::new(&context, &window).map_err(|e| e.to_string())?;

    println!("[7HLE UI] ¡Ventana gráfica creada con éxito! Ejecutando loop de renderizado...");

    // 3. Loop principal de eventos de la ventana
    let result = event_loop.run(move |event, elwt| {
        elwt.set_control_flow(ControlFlow::Poll);

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => {
                println!("[7HLE UI] Cerrando ventana de la aplicación de forma segura...");
                elwt.exit();
            }
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                window_id,
            } if window_id == window.id() => {
                let (width, height) = {
                    let size = window.inner_size();
                    (size.width, size.height)
                };

                surface
                    .resize(
                        NonZeroU32::new(width).unwrap_or(NonZeroU32::new(1).unwrap()),
                        NonZeroU32::new(height).unwrap_or(NonZeroU32::new(1).unwrap()),
                    )
                    .unwrap();

                let mut buffer = surface.buffer_mut().unwrap();
                
                // Pintamos un fondo clásico estilo escritorio de Windows / retro (azul oscuro / grisáceo)
                for pixel in buffer.iter_mut() {
                    *pixel = 0x001A334E; // Color azulado retro
                }

                buffer.present().unwrap();
            }
            _ => {}
        }

        // Forzar refresco constante para mantener la ventana fluida
        window.request_redraw();
    });

    result.map_err(|e| e.to_string())
}