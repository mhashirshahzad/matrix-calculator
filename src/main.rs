mod app;
mod matrix;

use app::App;
use eframe::egui::{self, Ui};
use std::sync::Arc;

fn main() -> eframe::Result<()> {
    let icon = eframe::icon_data::from_png_bytes(include_bytes!("../assets/matrix-logo.png"))
        .expect("The icon data must be valid");

    let mut options = eframe::NativeOptions::default();
    options.viewport = options.viewport.with_app_id("Matrix Calculator");
    options.viewport.icon = Some(Arc::new(icon));

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
            egui::ScrollArea::both()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    ui.heading("Matrix Editor");
                    ui.add_space(12.0);

                    // A | B side-by-side with outputs
                    self.spawn_matrices_ui(ui);

                    ui.add_space(20.0);
                    ui.add_space(12.0);

                    // Operations depend on both → below
                    self.show_arithmetic_operations(ui);
                });
        });
    }
}

impl MatrixGui {
    fn show_arithmetic_operations(&self, ui: &mut egui::Ui) {
        ui.heading("Arithmetic");
        ui.separator();

        let a = self.app.get_matrix(&self.app.matrix_a);
        let b = self.app.get_matrix(&self.app.matrix_b);

        let ops: &[(
            &str,
            fn(
                matrix::types::Matrix,
                matrix::types::Matrix,
            ) -> Result<matrix::types::Matrix, &'static str>,
        )] = &[
            ("A + B", |x, y| x + y),
            ("A - B", |x, y| x - y),
            ("B - A", |x, y| y - x),
            ("A × B", |x, y| x * y),
            ("B × A", |x, y| y * x),
        ];
        if let (Ok(a), Ok(b)) = (a, b) {
            for (label, op) in ops {
                match op(a.clone(), b.clone()) {
                    Ok(m) => {
                        Self::render_matrix(ui, label, &m);
                        ui.add_space(10.0);
                    }
                    Err(e) => {
                        ui.colored_label(egui::Color32::RED, e);
                    }
                }
            }
        }
    }

    pub fn spawn_matrices_ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // ==== Matrix A Editor + Operations ====
            ui.vertical(|ui| {
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Matrix A");
                        if ui.button("+ Row").clicked() {
                            self.app.matrix_a.add_row();
                        }
                        if ui.button("- Row").clicked() {
                            self.app.matrix_a.remove_row();
                        }
                        if ui.button("+ Col").clicked() {
                            self.app.matrix_a.add_col();
                        }
                        if ui.button("- Col").clicked() {
                            self.app.matrix_a.remove_col();
                        }
                    });

                    // Matrix editor
                    for i in 0..self.app.matrix_a.rows {
                        ui.horizontal(|ui| {
                            for j in 0..self.app.matrix_a.cols {
                                let cell = &mut self.app.matrix_a.data[i][j];
                                ui.text_edit_singleline(cell);
                            }
                        });
                    }
                });

                // Determinant
                if let Ok(m) = self.app.get_matrix(&self.app.matrix_a) {
                    ui.group(|ui| {
                        ui.label(format!("Determinant: {:?}", m.determinant()));
                        // Transpose
                        let t = matrix::transpose::transpose(&m);
                        Self::render_matrix(ui, "Transpose", &t);
                    });
                }
            });

            // ==== Matrix B Editor + Operations ====
            ui.vertical(|ui| {
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Matrix B");
                        if ui.button("+ Row").clicked() {
                            self.app.matrix_b.add_row();
                        }
                        if ui.button("- Row").clicked() {
                            self.app.matrix_b.remove_row();
                        }
                        if ui.button("+ Col").clicked() {
                            self.app.matrix_b.add_col();
                        }
                        if ui.button("- Col").clicked() {
                            self.app.matrix_b.remove_col();
                        }
                    });

                    // Matrix editor
                    for i in 0..self.app.matrix_b.rows {
                        ui.horizontal(|ui| {
                            for j in 0..self.app.matrix_b.cols {
                                let cell = &mut self.app.matrix_b.data[i][j];
                                ui.text_edit_singleline(cell);
                            }
                        });
                    }
                });

                // Determinant
                if let Ok(m) = self.app.get_matrix(&self.app.matrix_b) {
                    ui.group(|ui| {
                        ui.label(format!("Determinant: {:?}", m.determinant()));
                        // Transpose
                        let t = matrix::transpose::transpose(&m);
                        Self::render_matrix(ui, "Transpose", &t);
                    });
                }
            });
        });
    }

    fn render_matrix(ui: &mut egui::Ui, title: &str, matrix: &matrix::types::Matrix) {
        use egui::{Frame, RichText};

        Frame::new()
            .fill(ui.visuals().extreme_bg_color)
            .corner_radius(egui::CornerRadius::same(8))
            .inner_margin(egui::Margin::symmetric(12, 10))
            .stroke(egui::Stroke::new(
                1.0,
                ui.visuals().widgets.noninteractive.bg_stroke.color,
            ))
            .show(ui, |ui| {
                ui.label(RichText::new(title).strong().size(16.0));

                ui.add_space(8.0);

                egui::Grid::new(&format!("transpose-{}", title))
                    .spacing([16.0, 10.0])
                    .show(ui, |ui| {
                        for row in &matrix.data {
                            for cell in row {
                                ui.allocate_ui_with_layout(
                                    egui::vec2(48.0, 20.0),
                                    egui::Layout::centered_and_justified(
                                        egui::Direction::LeftToRight,
                                    ),
                                    |ui| {
                                        ui.label(egui::RichText::new(cell).monospace().size(14.0));
                                    },
                                );
                            }
                            ui.end_row();
                        }
                    });
            });
    }
}
