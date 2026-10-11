use iced::widget::{button, column, container, row, text, Space, scrollable};
use iced::{Element, Length, Sandbox, Settings, Color};
use crate::pe_loader::WinProcess;

pub struct Win7Desktop {
    status_message: String,
    running_app: Option<String>,
    imports_list: Vec<String>,
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
            status_message: "7HLE Listo. Seleccioná un .exe para inspeccionar sus imports.".into(),
            running_app: None,
            imports_list: Vec::new(),
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
                            self.imports_list = proc.imported_functions;
                        }
                        Err(e) => {
                            self.status_message = format!("Error al parsear PE: {}", e);
                            self.imports_list.clear();
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

        let mut imports_col = column![text("Funciones Importadas (IAT):").size(16)].spacing(5);
        if self.imports_list.is_empty() {
            imports_col = imports_col.push(text("Ningún ejecutable cargado todavía.").size(13));
        } else {
            for imp in self.imports_list.iter().take(10) { // Mostramos los primeros 10 para no saturar
                imports_col = imports_col.push(text(format!(" • {}", imp)).size(12));
            }
            if self.imports_list.len() > 10 {
                imports_col = imports_col.push(text(format!(" ... y {} más.", self.imports_list.len() - 10)).size(12));
            }
        }

        let desktop_content = column![
            text("Escritorio 7HLE").size(24),
            Space::with_height(10),
            text(&self.status_message),
            Space::with_height(10),
            if let Some(app) = &self.running_app {
                text(format!("Proceso Activo: {}", app)).size(14)
            } else {
                text("Ningún programa cargado en el HLE.").size(14)
            },
            Space::with_height(15),
            scrollable(imports_col).height(120),
            Space::with_height(15),
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
                background: Some(Color::from_rgb(0.08, 0.25, 0.45).into()),
                text_color: Some(Color::WHITE),
                ..Default::default()
            })
            .into()
    }
}

pub fn run_ui() -> iced::Result {
    Win7Desktop::run(Settings::default())
}