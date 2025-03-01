use eframe::egui;
use pyo3::prelude::*;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Controlador Spotify",
        options,
        Box::new(|cc|{
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(MyApp::new()))
        }),
    );
}

struct MyApp {
    current_track: String,
    current_artist: String,
    album_cover_url: String,
    is_shuffle: bool,
    is_playing: bool,
    progress: f32,
    py_controller: Py<PyAny>,
    receiver: mpsc::Receiver<SpotifyState>,
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
                    let track = py_controller_clone.call_method0(py, "current_track_name")
                        .and_then(|m| m.extract(py))
                        .unwrap_or_else(|_| "No disponible".to_string());
                    let artist = py_controller_clone.call_method0(py, "artist_name")
                        .and_then(|m| m.extract(py))
                        .unwrap_or_else(|_| "No disponible".to_string());
                    let shuffle = py_controller_clone.call_method0(py, "get_shuffle_state")
                        .and_then(|m| m.extract(py))
                        .unwrap_or(false);
                    let playing = py_controller_clone.call_method0(py, "is_playing")
                        .and_then(|m| m.extract(py))
                        .unwrap_or(false);
                    let album_cover = py_controller_clone.call_method0(py, "album_cover")
                        .and_then(|m| m.extract(py))
                        .unwrap_or_else(|_| "https://picsum.photos/seed/1.759706314/1024".to_string());
                    let progress: f32 = py_controller_clone.call_method0(py, "playback_progress")
                        .and_then(|m| m.extract(py))
                        .unwrap_or(0) as f32 / py_controller_clone.call_method0(py, "current_track_duration")
                        .and_then(|m| m.extract(py))
                        .unwrap_or(1) as f32;

                    SpotifyState { track, artist, album_cover,shuffle, playing, progress }
                });
                sender.send(state).ok();
                thread::sleep(Duration::from_secs(2));
            });

            Self {
                current_track: "No disponible".to_string(),
                current_artist: "No disponible".to_string(),
                album_cover_url: "No disponible".to_string(),
                is_shuffle: false,
                is_playing: false,
                progress: 0.0,
                py_controller,
                receiver,
            }
        })
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update_data();
        ctx.request_repaint();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Controlador Spotify");
            ui.add(egui::Image::new(&self.album_cover_url).max_width(200.0).corner_radius(10));
            //ui.label(&self.album_cover_url);
            ui.label(format!("🎵 {}", self.current_track));
            ui.label(format!("🎤 {}", self.current_artist));
            ui.horizontal(|ui| {
                if ui.button("⏮").clicked() {
                    Python::with_gil(|py| { let _ = self.py_controller.call_method0(py, "prev"); });
                }
                if self.is_playing {
                    if ui.button("⏸").clicked() {
                        Python::with_gil(|py| { let _ = self.py_controller.call_method0(py, "pause"); });
                    }
                } else {
                    if ui.button("▶").clicked() {
                        Python::with_gil(|py| { let _ = self.py_controller.call_method0(py, "play"); });
                    }
                }
                if ui.button("⏭").clicked() {
                    Python::with_gil(|py| { let _ = self.py_controller.call_method0(py, "next"); });
                }
            });
            ui.horizontal(|ui| {
                if ui.checkbox(&mut self.is_shuffle, "Shuffle").clicked() {
                    Python::with_gil(|py| {
                        let _ = self.py_controller.call_method1(py, "set_shuffle", (self.is_shuffle,));
                    });
                }
            });
            ui.add(egui::ProgressBar::new(self.progress).show_percentage());
        });
    }

}

impl MyApp{
    fn update_data(&mut self){
        if let Ok(state) = self.receiver.try_recv() {
            self.current_track = state.track;
            self.current_artist = state.artist;
            self.is_shuffle = state.shuffle;
            self.is_playing = state.playing;
            self.progress = state.progress;
            self.album_cover_url =state.album_cover;
        }
    }
}
