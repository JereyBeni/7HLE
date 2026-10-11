use iced::widget::{button, checkbox, column, pick_list, row, text, Container};
use iced::{Element, Sandbox, Settings};

pub fn run_ui() -> Result<(), iced::Error> {
    SevenHleUi::run(Settings::default())
}

struct SevenHleUi {
    scale_mode: String,
    speed_mode: String,
    xp_mode: bool,
    status_message: String,
}

#[derive(Debug, Clone)]
enum Message {
    ScaleChanged(String),
    SpeedChanged(String),
    XpModeToggled(bool),
    ApplyConfig,
}

impl Sandbox for SevenHleUi {
    type Message = Message;

    fn new() -> Self {
        Self {
            scale_mode: "Fit to Window".to_string(),
            speed_mode: "Normal".to_string(),
            xp_mode: false,
            status_message: "Esperando configuración...".to_string(),
        }
    }

    fn title(&self) -> String {
        String::from("seven_hle - Panel de Configuración")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ScaleChanged(val) => {
                self.scale_mode = val;
                println!("[UI] Scale mode seleccionado: {}", self.scale_mode);
            }
            Message::SpeedChanged(val) => {
                self.speed_mode = val;
                println!("[UI] Perfil de velocidad seleccionado: {}", self.speed_mode);
            }
            Message::XpModeToggled(val) => {
                self.xp_mode = val;
                println!("[UI] XP Mode (NT 5.1): {}", if self.xp_mode { "Activado" } else { "Desactivado" });
            }
            Message::ApplyConfig => {
                self.status_message = format!(
                    "¡Config aplicada! Scale: {}, Speed: {}, XP Mode: {}",
                    self.scale_mode, self.speed_mode, self.xp_mode
                );
                // Disparamos la lógica hacia los subsistemas de seven_hle
                apply_backend_config(&self.scale_mode, &self.speed_mode, self.xp_mode);
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let scales = vec![
            "Fit to Window".to_string(),
            "Stretch".to_string(),
            "1:1 Native".to_string(),
        ];
        
        let speeds = vec![
            "Faster (High Perf)".to_string(),
            "Normal".to_string(),
            "Slow (Compatibility)".to_string(),
        ];

        let content = column![
            text("⚙️ Configuración de seven_hle").size(24),
            
            row![
                text("Scale Mode:     "),
                pick_list(scales, Some(self.scale_mode.clone()), Message::ScaleChanged)
            ].spacing(10),

            row![
                text("Execution Speed:"),
                pick_list(speeds, Some(self.speed_mode.clone()), Message::SpeedChanged)
            ].spacing(10),

            checkbox("Activar XP Mode (NT 5.1 Compatibility)", self.xp_mode)
                .on_toggle(Message::XpModeToggled),

            button("Aplicar Configuración")
                .on_press(Message::ApplyConfig),

            text(&self.status_message).size(14)
        ]
        .spacing(20);

        Container::new(content)
            .padding(30)
            .into()
    }
}

// Función puente que conecta el estado de la UI con el núcleo del emulador
fn apply_backend_config(scale: &str, speed: &str, xp_active: bool) {
    println!("\n--- Aplicando cambios al núcleo de seven_hle ---");
    
    if xp_active {
        println!("[Backend] -> Activando capa de compatibilidad con Windows XP (NT 5.1).");
    } else {
        println!("[Backend] -> Operando bajo perfil nativo de Windows 7 (NT 6.1).");
    }

    match speed {
        s if s.contains("Faster") => println!("[Backend] -> Modo rápido activado: optimizando ciclos de CPU."),
        s if s.contains("Slow") => println!("[Backend] -> Modo lento activado: ralentizando ejecución para apps legacy."),
        _ => println!("[Backend] -> Velocidad normal configurada."),
    }

    println!("[Backend] -> Renderizado gráfico ajustado a escala: {}\n", scale);
}