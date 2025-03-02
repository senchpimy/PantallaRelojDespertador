use eframe::egui;
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

            thread::spawn(move || loop {
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
                        .unwrap_or_else(|_| "https://picsum.photos/seed/1.759706314/1024".to_string());
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
            let duration = 0.5;
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
            ui.horizontal(|ui| {
                ui.label("Pantalla actual:");
                ui.radio_value(&mut self.active_screen, 0, "1");
                ui.radio_value(&mut self.active_screen, 1, "2");
            });

            let available_rect = ui.available_rect_before_wrap();
            if let Some(transition) = &self.transition {
                let t = transition.progress.clamp(0.0, 1.0);
                let width = available_rect.width();
                let old_offset = egui::vec2(t * width * transition.direction, 0.0);
                let new_offset = egui::vec2(old_offset.x - width * transition.direction, 0.0);

                {
                    let rect = available_rect.translate(old_offset);
                    let mut child = ui.child_ui(rect, egui::Layout::default(), None);
                    spotify_gui(&mut child, &self.screens[transition.from], &self.py_controller);
                }
                {
                    let rect = available_rect.translate(new_offset);
                    let mut child = ui.child_ui(rect, egui::Layout::default(), None);
                    spotify_gui(&mut child, &self.screens[transition.to], &self.py_controller);
                }
            } else {
                let mut child = ui.child_ui(available_rect, egui::Layout::default(), None);
                spotify_gui(&mut child, &self.screens[self.active_screen], &self.py_controller);
            }

            let mut is_shuffle = self.screens[self.active_screen].is_shuffle;
            if ui.checkbox(&mut is_shuffle, "Shuffle").changed() {
                self.screens[self.active_screen].is_shuffle = is_shuffle;
                Python::with_gil(|py| {
                    let _ = self.py_controller.call_method1(py, "toggle_shuffle", (is_shuffle,));
                });
            }
        });
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
            ui.label(egui::RichText::new(format!("🎵 {}", screen.current_track)).heading());
            ui.label(egui::RichText::new(format!("🎤 {}", screen.current_artist)).strong());
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
            ui.add(egui::ProgressBar::new(screen.progress).desired_width(ui.available_width() * 0.8));
        });
    });
}
