use chrono::Datelike;
use eframe::egui;
use eframe::egui::{Color32, Frame, RichText};
use pyo3::prelude::*;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Controlador Spotify",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(MyApp::new()))
        }),
    );
}

struct Transition {
    from: usize,
    to: usize,
    progress: f32,  // Progreso de 0.0 a 1.0
    direction: f32, // 1.0 si el swipe es hacia la derecha, -1.0 si es hacia la izquierda
}

struct MyApp {
    screens: Vec<SpotifyScreen>,
    active_screen: usize,
    py_controller: Py<PyAny>,
    receiver: mpsc::Receiver<SpotifyState>,
    drag_start: Option<egui::Pos2>,
    swipe_processed: bool,
    transition: Option<Transition>,
    last_frame: Instant,

    calendar_month: u32,
    calendar_year: i32,
}

#[derive(Default)]
struct SpotifyScreen {
    current_track: String,
    current_artist: String,
    album_cover_url: String,
    is_shuffle: bool,
    is_playing: bool,
    progress: f32,
}

struct SpotifyState {
    track: String,
    artist: String,
    album_cover: String,
    shuffle: bool,
    playing: bool,
    progress: f32,
}

impl MyApp {
    fn new() -> Self {
        Python::with_gil(|py| {
            py.run("import sys; sys.path.append('.')", None, None).ok();
            let module = PyModule::import(py, "ejemplo").unwrap();
            let controller_class = module.getattr("SpotifyController").unwrap();
            let py_controller: Py<PyAny> = controller_class.call0().unwrap().into();
            let py_controller_clone = py_controller.clone_ref(py);
            let (sender, receiver) = mpsc::channel();

            thread::spawn(move || {
                loop {
                    let state = Python::with_gil(|py| {
                        let track = py_controller_clone
                            .call_method0(py, "current_track_name")
                            .and_then(|m| m.extract(py))
                            .unwrap_or_else(|_| "No disponible".to_string());

                        let artist = py_controller_clone
                            .call_method0(py, "artist_name")
                            .and_then(|m| m.extract(py))
                            .unwrap_or_else(|_| "No disponible".to_string());
                        let shuffle = py_controller_clone
                            .call_method0(py, "get_shuffle_state")
                            .and_then(|m| m.extract(py))
                            .unwrap_or(false);
                        let playing = py_controller_clone
                            .call_method0(py, "is_playing")
                            .and_then(|m| m.extract(py))
                            .unwrap_or(false);
                        let album_cover = py_controller_clone
                            .call_method0(py, "album_cover")
                            .and_then(|m| m.extract(py))
                            .unwrap_or_else(|_| {
                                "https://picsum.photos/seed/1.759706314/1024".to_string()
                            });
                        let progress: f32 = py_controller_clone
                            .call_method0(py, "playback_progress")
                            .and_then(|m| m.extract(py))
                            .unwrap_or(0) as f32
                            / py_controller_clone
                                .call_method0(py, "current_track_duration")
                                .and_then(|m| m.extract(py))
                                .unwrap_or(1) as f32;

                        SpotifyState {
                            track,
                            artist,
                            album_cover,
                            shuffle,
                            playing,
                            progress,
                        }
                    });
                    sender.send(state).ok();
                    thread::sleep(Duration::from_secs(2));
                }
            });

            Self {
                screens: vec![SpotifyScreen::default(), SpotifyScreen::default()],
                active_screen: 0,
                py_controller,
                receiver,
                drag_start: None,
                swipe_processed: false,
                transition: None,
                last_frame: Instant::now(),
                calendar_month: chrono::Local::now().month(),
                calendar_year: chrono::Local::now().year(),
            }
        })
    }

    fn update_data(&mut self) {
        while let Ok(state) = self.receiver.try_recv() {
            for screen in &mut self.screens {
                screen.current_track = state.track.clone();
                screen.current_artist = state.artist.clone();
                screen.album_cover_url = state.album_cover.clone();
                screen.is_shuffle = state.shuffle;
                screen.is_playing = state.playing;
                screen.progress = state.progress;
            }
        }
    }

    fn handle_swipe(&mut self, ctx: &egui::Context) {
        ctx.input(|i| {
            let pointer = &i.pointer;

            if pointer.any_pressed() && self.drag_start.is_none() {
                self.drag_start = pointer.press_origin();
            }

            if let Some(start_pos) = self.drag_start {
                if let Some(current_pos) = pointer.latest_pos() {
                    let delta = current_pos - start_pos;
                    if delta.x.abs() > 50.0 && !self.swipe_processed && self.transition.is_none() {
                        let new_screen = if delta.x > 0.0 {
                            (self.active_screen + self.screens.len() - 1) % self.screens.len()
                        } else {
                            (self.active_screen + 1) % self.screens.len()
                        };
                        let direction = if delta.x > 0.0 { 1.0 } else { -1.0 };
                        self.transition = Some(Transition {
                            from: self.active_screen,
                            to: new_screen,
                            progress: 0.0,
                            direction,
                        });
                        self.swipe_processed = true;
                    }
                }
            }

            if pointer.any_released() {
                self.drag_start = None;
                self.swipe_processed = false;
            }
        });
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update_data();
        self.handle_swipe(ctx);

        let now = Instant::now();
        if let Some(transition) = &mut self.transition {
            let dt = now.duration_since(self.last_frame).as_secs_f32();
            self.last_frame = now;
            let duration = 0.5; // Duración de la transición en segundos
            transition.progress += dt / duration;
            if transition.progress >= 1.0 {
                self.active_screen = transition.to;
                self.transition = None;
            }
        } else {
            self.last_frame = now;
        }

        ctx.request_repaint();

        egui::CentralPanel::default().show(ctx, |ui| {
            //ui.horizontal(|ui| {
            //    ui.label("Pantalla actual:");
            //    ui.radio_value(&mut self.active_screen, 0, "1");
            //    ui.radio_value(&mut self.active_screen, 1, "2");
            //});

            let available_rect = ui.available_rect_before_wrap();
            if let Some(transition) = &self.transition {
                let t = transition.progress.clamp(0.0, 1.0);
                let width = available_rect.width();
                let old_offset = egui::vec2(t * width * transition.direction, 0.0);
                let new_offset = egui::vec2(old_offset.x - width * transition.direction, 0.0);

                {
                    let rect = available_rect.translate(old_offset);
                    let mut child = ui.child_ui(rect, egui::Layout::default(), None);
                    if transition.from == 0 {
                        calendario(
                            &mut child,
                            &mut self.calendar_month,
                            &mut self.calendar_year,
                        );
                    } else {
                        spotify_gui(
                            &mut child,
                            &self.screens[transition.from],
                            &self.py_controller,
                        );
                    }
                }
                {
                    // Renderizar la pantalla de destino con un desplazamiento
                    let rect = available_rect.translate(new_offset);
                    let mut child = ui.child_ui(rect, egui::Layout::default(), None);
                    if transition.to == 0 {
                        calendario(
                            &mut child,
                            &mut self.calendar_month,
                            &mut self.calendar_year,
                        );
                    } else {
                        spotify_gui(
                            &mut child,
                            &self.screens[transition.to],
                            &self.py_controller,
                        );
                    }
                }
            } else {
                // Renderizar la pantalla actual sin transición
                let mut child = ui.child_ui(available_rect, egui::Layout::default(), None);
                if self.active_screen == 0 {
                    calendario(
                        &mut child,
                        &mut self.calendar_month,
                        &mut self.calendar_year,
                    );
                } else {
                    spotify_gui(
                        &mut child,
                        &self.screens[self.active_screen],
                        &self.py_controller,
                    );
                }
            }

            //let mut is_shuffle = self.screens[self.active_screen].is_shuffle;
            //if ui.checkbox(&mut is_shuffle, "Shuffle").changed() {
            //    self.screens[self.active_screen].is_shuffle = is_shuffle;
            //    Python::with_gil(|py| {
            //        let _ = self.py_controller.call_method1(py, "toggle_shuffle", (is_shuffle,));
            //    });
            //}
        });
    }
}

struct Pendiente {
    titulo: String,
    fecha: String,
    descripion: Option<String>,
}

fn calendario(ui: &mut egui::Ui, calendar_month: &mut u32, calendar_year: &mut i32) {
    ui.columns(2, |cols| {
        cols[0].vertical(|ui| {
            cal(ui, calendar_month, calendar_year);
        });
        cols[1].vertical(|ui| {
            let time = chrono::offset::Local::now().format("%H:%M").to_string();
            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new(time)
                        .strong()
                        .font(egui::FontId::new(160.0, egui::FontFamily::Proportional)),
                );
            });
            ui.set_min_width(200.0);
            ui.label("Columna de texto:");
            ui.separator();
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.spacing_mut().item_spacing = egui::Vec2::new(100.0, 50.0); // 10px horizontal, 20px vertical
                ui.separator();
                for _ in 0..30 {
                    let frame = Frame::new()
                        .fill(Color32::from_rgb(45, 45, 45)) // Color del rectángulo
                        .corner_radius(5.0)
                        .inner_margin(egui::Margin::same(10));
                    ui.vertical_centered(|ui| {
                        frame.show(ui, |ui| {
                            ui.spacing_mut().item_spacing = egui::Vec2::new(0.0, 1.0); // 10px horizontal, 20px vertical
                            ui.label(
                                egui::RichText::new("Tarea")
                                    .strong()
                                    .font(egui::FontId::new(40.0, egui::FontFamily::Proportional)),
                            );
                            ui.label(
                                RichText::new(
                                    r"
sadsjhasdsghdghdasghdsaahkdsadsjhasdsghdghdasghdsaahkd
sadsjhasdsghdghdasghdsaahkdsadsjhasdsghdghdasghdsaahkd
Texto dentro del rectángulo",
                                )
                                .color(Color32::WHITE),
                            );
                        });
                    });
                }
            });
        });
    });
}

fn cal(ui: &mut egui::Ui, calendar_month: &mut u32, calendar_year: &mut i32) {
    let mut current_date = chrono::Local::now().date_naive();
    let mut current_month = *calendar_month;
    let mut current_year = *calendar_year;

    let total_height = ui.available_height();

    let frame_ui = |ui: &mut egui::Ui| {
        ui.horizontal(|ui| {
            if ui.button("◀").clicked() {
                if current_month == 1 {
                    current_month = 12;
                    current_year -= 1;
                } else {
                    current_month -= 1;
                }
                *calendar_month = current_month; // Actualizar el mes
                *calendar_year = current_year; // Actualizar el año
            }
            ui.label(format!("{} {}", month_name(current_month), current_year));
            if ui.button("▶").clicked() {
                if current_month == 12 {
                    current_month = 1;
                    current_year += 1;
                } else {
                    current_month += 1;
                }
                *calendar_month = current_month; // Actualizar el mes
                *calendar_year = current_year; // Actualizar el año
            }
        });

        let days_in_month = days_in_month(current_month, current_year);
        let first_day_of_month = chrono::NaiveDate::from_ymd_opt(current_year, current_month, 1)
            .unwrap()
            .weekday()
            .num_days_from_monday(); // 0 = Lunes, 6 = Domingo

        let cell_size = egui::vec2(40.0, total_height * 0.08); // Ancho y alto de cada celda

        ui.columns(7, |columns| {
            for (i, column) in columns.iter_mut().enumerate() {
                column.label(weekday_name(i as u32)); // Mostrar los nombres de los días de la semana
            }
        });

        let mut day_counter = 1;
        for _ in 0..6 {
            // Máximo de 6 filas para cubrir todos los días del mes
            ui.columns(7, |columns| {
                for (i, column) in columns.iter_mut().enumerate() {
                    if (i as u32) < first_day_of_month && day_counter == 1 {
                        // Espacios vacíos antes del primer día del mes
                        column.label("");
                    } else if day_counter <= days_in_month {
                        // Mostrar los días del mes
                        if column
                            .add_sized(cell_size, egui::Button::new(format!("{}", day_counter)))
                            .clicked()
                        {
                            println!("Día seleccionado: {}", day_counter);
                        }
                        day_counter += 1;
                    } else {
                        // Espacios vacíos después del último día del mes
                        column.label("");
                    }
                }
            });
        }
    };
    ui.add_space(total_height * 0.2);
    Frame::new()
        .fill(Color32::from_rgb(32, 32, 32))
        .corner_radius(10)
        .shadow(egui::Shadow {
            offset: [0, 0],
            blur: 20,
            spread: 0,
            color: Color32::from_rgba_unmultiplied(0, 0, 0, 120),
        })
        .inner_margin(10)
        .outer_margin(50)
        .show(ui, frame_ui);
}

// Función para obtener el nombre del mes
fn month_name(month: u32) -> &'static str {
    match month {
        1 => "Enero",
        2 => "Febrero",
        3 => "Marzo",
        4 => "Abril",
        5 => "Mayo",
        6 => "Junio",
        7 => "Julio",
        8 => "Agosto",
        9 => "Septiembre",
        10 => "Octubre",
        11 => "Noviembre",
        12 => "Diciembre",
        _ => "Desconocido",
    }
}

// Función para obtener el nombre del día de la semana
fn weekday_name(weekday: u32) -> &'static str {
    match weekday {
        0 => "Lun",
        1 => "Mar",
        2 => "Mié",
        3 => "Jue",
        4 => "Vie",
        5 => "Sáb",
        6 => "Dom",
        _ => "Desconocido",
    }
}

// Función para obtener el número de días en un mes
fn days_in_month(month: u32, year: i32) -> u32 {
    match month {
        4 | 6 | 9 | 11 => 30, // Abril, Junio, Septiembre, Noviembre
        2 => {
            // Febrero (considera años bisiestos)
            if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
                29
            } else {
                28
            }
        }
        _ => 31, // Resto de los meses
    }
}
fn spotify_gui(ui: &mut egui::Ui, screen: &SpotifyScreen, py_controller: &Py<PyAny>) {
    ui.add_space(100.0);
    ui.centered_and_justified(|ui| {
        ui.vertical_centered(|ui| {
            let img_size = ui.available_height() * 0.7;
            ui.add(
                egui::Image::new(&screen.album_cover_url)
                    .max_size(egui::vec2(img_size, img_size))
                    .maintain_aspect_ratio(true)
                    .corner_radius(21.0),
            );
            ui.label(
                egui::RichText::new(format!("🎵 {}", screen.current_track))
                    .heading()
                    .font(egui::FontId::new(24.0, egui::FontFamily::Proportional)),
            );

            ui.label(
                egui::RichText::new(format!("🎤 {}", screen.current_artist))
                    .strong()
                    .font(egui::FontId::new(20.0, egui::FontFamily::Proportional)),
            );

            ui.add_space(20.);

            ui.horizontal(|ui| {
                let available_width = ui.available_width();
                let button_size = egui::vec2(60.0, 60.0);
                let total_buttons_width = button_size.x * 4.5 + 2.0 * 20.0; // 3 botones y espacio entre ellos

                let left_margin = (available_width - total_buttons_width) / 2.0;

                ui.add_space(left_margin);
                ui.spacing_mut().item_spacing = egui::vec2(70.0, 10.0);
                if ui.add_sized(button_size, egui::Button::new("⏮")).clicked() {
                    Python::with_gil(|py| {
                        let _ = py_controller.call_method0(py, "prev");
                    });
                }
                if screen.is_playing {
                    if ui.add_sized(button_size, egui::Button::new("⏸")).clicked() {
                        Python::with_gil(|py| {
                            let _ = py_controller.call_method0(py, "pause");
                        });
                    }
                } else if ui.add_sized(button_size, egui::Button::new("▶")).clicked() {
                    Python::with_gil(|py| {
                        let _ = py_controller.call_method0(py, "play");
                    });
                }
                if ui.add_sized(button_size, egui::Button::new("⏭")).clicked() {
                    Python::with_gil(|py| {
                        let _ = py_controller.call_method0(py, "next");
                    });
                }
            });
            ui.add_space(30.0);
            ui.add(
                egui::ProgressBar::new(screen.progress).desired_width(ui.available_width() * 0.8),
            );
        });
    });
}
