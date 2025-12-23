mod app;
mod matrix;

use app::App;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Matrix Transpose",
        options,
        Box::new(|_| Ok(Box::new(MatrixGui::default()))),
    )
}

struct MatrixGui {
    app: App,
}

impl Default for MatrixGui {
    fn default() -> Self {
        Self { app: App::new() }
    }
}

impl eframe::App for MatrixGui {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Matrix Editor");

            ui.horizontal(|ui| {
                if ui.button("+ Row").clicked() {
                    self.app.rows += 1;
                    self.app.cells.push(vec!["0".into(); self.app.cols]);
                }
                if ui.button("+ Col").clicked() {
                    self.app.cols += 1;
                    for row in &mut self.app.cells {
                        row.push("0".into());
                    }
                }
            });

            ui.separator();
            ui.label("Matrix:");

            for i in 0..self.app.rows {
                ui.horizontal(|ui| {
                    for j in 0..self.app.cols {
                        ui.text_edit_singleline(&mut self.app.cells[i][j]);
                    }
                });
            }

            ui.separator();
            ui.label("Transpose:");

            match self.app.to_matrix() {
                Ok(m) => {
                    let t = matrix::transpose::transpose(&m);
                    for row in t.data {
                        ui.horizontal(|ui| {
                            for v in row {
                                ui.label(format!("{v}"));
                            }
                        });
                    }
                }
                Err(e) => {
                    ui.colored_label(egui::Color32::RED, e);
                }
            }
        });
    }
}
