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
                    self.spawn_matrices_ui(ui);
                    ui.separator();

                    self.show_transposes(ui);
                    self.show_arithmetic_operations(ui);
                    self.show_determinants(ui);
                });
        });
    }
}

impl MatrixGui {
    fn show_transposes(&self, ui: &mut egui::Ui) {
        ui.heading("Transposes");
        ui.separator();

        match self.app.get_matrix(&self.app.matrix_a) {
            Ok(m) => {
                let t = matrix::transpose::transpose(&m);
                Self::render_matrix(ui, "Transpose of Matrix A", &t);
            }
            Err(e) => {
                ui.colored_label(egui::Color32::RED, e);
            }
        }

        ui.add_space(12.0);

        match self.app.get_matrix(&self.app.matrix_b) {
            Ok(m) => {
                let t = matrix::transpose::transpose(&m);
                Self::render_matrix(ui, "Transpose of Matrix B", &t);
            }
            Err(e) => {
                ui.colored_label(egui::Color32::RED, e);
            }
        }
    }

    fn show_determinants(&self, ui: &mut egui::Ui) {
        use egui::RichText;

        ui.heading("Determinants");
        ui.separator();

        for (label, matrix) in [
            ("Matrix A", &self.app.matrix_a),
            ("Matrix B", &self.app.matrix_b),
        ] {
            match self.app.get_matrix(matrix) {
                Ok(m) => match m.determinant() {
                    Ok(det) => {
                        ui.label(
                            RichText::new(format!("{label}: {det}"))
                                .monospace()
                                .size(15.0),
                        );
                    }
                    Err(e) => {
                        ui.colored_label(egui::Color32::RED, e);
                    }
                },
                Err(e) => {
                    ui.colored_label(egui::Color32::RED, e);
                }
            }
        }
    }

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

    fn edit_matrix_ui(ui: &mut Ui, label: &str, matrix: &mut matrix::types::Matrix) {
        ui.label(label);

        ui.horizontal(|ui| {
            if ui.button("+ Row").clicked() {
                matrix.rows += 1;
                matrix.data.push(vec!["0".into(); matrix.cols]);
            }
            if ui.button("+ Col").clicked() {
                matrix.cols += 1;
                for row in &mut matrix.data {
                    row.push("0".into());
                }
            }
            if ui.button("- Row").clicked() {
                if matrix.rows > 1 {
                    matrix.rows -= 1;
                    matrix.data.pop();
                }
            }
            if ui.button("- Col").clicked() {
                if matrix.cols > 1 {
                    matrix.cols -= 1;
                    for row in &mut matrix.data {
                        row.pop();
                    }
                }
            }
        });

        for i in 0..matrix.rows {
            ui.horizontal(|ui| {
                for j in 0..matrix.cols {
                    ui.text_edit_singleline(&mut matrix.data[i][j]);
                }
            });
        }
    }

    pub fn spawn_matrices_ui(&mut self, ui: &mut Ui) {
        egui::Frame::group(ui.style())
            .inner_margin(10.0)
            .show(ui, |ui| {
                ui.heading("Matrix A");
                Self::edit_matrix_ui(ui, "", &mut self.app.matrix_a);
            });

        ui.add_space(16.0);

        egui::Frame::group(ui.style())
            .inner_margin(10.0)
            .show(ui, |ui| {
                ui.heading("Matrix B");
                Self::edit_matrix_ui(ui, "", &mut self.app.matrix_b);
            });
    }

    fn render_matrix(ui: &mut egui::Ui, title: &str, matrix: &matrix::types::Matrix) {
        use egui::{Frame, RichText};

        Frame::group(ui.style())
            .inner_margin(egui::Margin::same(8))
            .show(ui, |ui| {
                ui.label(RichText::new(title).strong().size(16.0));

                ui.add_space(6.0);

                egui::Grid::new(title)
                    .spacing([12.0, 8.0])
                    .striped(true)
                    .show(ui, |ui| {
                        for row in &matrix.data {
                            for cell in row {
                                ui.label(cell);
                            }
                            ui.end_row();
                        }
                    });
            });
    }
}
