use iced::widget::{button, column, container, row, text, Space};
use iced::{Element, Length, Sandbox, Settings, Color};
use crate::pe_loader::WinProcess;

pub struct Win7Desktop {
    status_message: String,
    running_app: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    SelectExeClicked,
    StartMenuClicked,
}

impl Sandbox for Win7Desktop {
    type Message = Message;

    fn new() -> Self {
        Self {
            status_message: "7HLE Listo. Seleccioná un .exe para correrlo.".into(),
            running_app: None,
        }
    }

    fn title(&self) -> String {
        "7HLE - Windows 7 HLE Environment".into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::SelectExeClicked => {
                if let Some(path) = rfd::FileDialog::new().add_filter("Executable", &["exe"]).pick_file() {
                    let path_str = path.to_string_lossy();
                    match WinProcess::load(&path_str) {
                        Ok(proc) => {
                            self.status_message = format!("Éxito: {}", proc.info_summary);
                            self.running_app = Some(path.file_name().unwrap().to_string_lossy().to_string());
                        }
                        Err(e) => {
                            self.status_message = format!("Error al parsear PE: {}", e);
                        }
                    }
                }
            }
            Message::StartMenuClicked => {
                self.status_message = "Menú Inicio de 7HLE desplegado, boludo.".into();
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let top_bar = row![
            button("Iniciar").on_press(Message::StartMenuClicked),
            text("  7HLE (Windows 7 Compatibility Layer)").size(16),
        ]
        .padding(10);

        let desktop_content = column![
            text("Escritorio 7HLE").size(24),
            Space::with_height(20),
            text(&self.status_message),
            Space::with_height(10),
            if let Some(app) = &self.running_app {
                text(format!("Proceso Activo: {}", app)).size(14)
            } else {
                text("Ningún programa cargado en el HLE.").size(14)
            },
            Space::with_height(30),
            button("Examinar y cargar .exe...").on_press(Message::SelectExeClicked),
        ]
        .padding(20);

        let taskbar = row![
            text("  [Inicio]  |  7HLE Terminal  |  Sistema").size(12)
        ]
        .padding(8);

        let layout = column![
            top_bar,
            container(desktop_content)
                .width(Length::Fill)
                .height(Length::Fill),
            taskbar
        ];

        container(layout)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_| container::Appearance {
                background: Some(Color::from_rgb(0.08, 0.25, 0.45).into()), // Azul Aero clásico de Win 7
                text_color: Some(Color::WHITE),
                ..Default::default()
            })
            .into()
    }
}

pub fn run_ui() -> iced::Result {
    Win7Desktop::run(Settings::default())
}