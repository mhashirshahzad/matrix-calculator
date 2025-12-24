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
                    self.app.matrix_a.rows += 1;
                    self.app
                        .matrix_a
                        .data
                        .push(vec!["0".into(); self.app.matrix_a.cols]);
                }
                if ui.button("+ Col").clicked() {
                    self.app.matrix_a.cols += 1;
                    for row in &mut self.app.matrix_a.data {
                        row.push("0".into());
                    }
                }
            });
            ui.separator();
            ui.horizontal(|ui| {
                if ui.button("- Row").clicked() {
                    if self.app.matrix_a.rows > 1 {
                        self.app.matrix_a.rows -= 1;
                        self.app.matrix_a.data.pop(); // remove the last row
                    }
                }

                if ui.button("- Col").clicked() {
                    if self.app.matrix_a.cols > 1 {
                        self.app.matrix_a.cols -= 1;
                        for row in &mut self.app.matrix_a.data {
                            row.pop(); // remove the last column in each row
                        }
                    }
                }
            });

            ui.separator();
            ui.label("Matrix A :");

            for i in 0..self.app.matrix_a.rows {
                ui.horizontal(|ui| {
                    for j in 0..self.app.matrix_a.cols {
                        ui.text_edit_singleline(&mut self.app.matrix_a.data[i][j]);
                    }
                });
            }
            ui.separator();

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
