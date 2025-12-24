mod app;
mod matrix;

use app::App;
use eframe::egui::{self, Ui};

#[allow(unused)]
const ICON: &[u8; 1676] = include_bytes!("../assets/matrix-logo.svg");

fn main() -> eframe::Result<()> {
    let mut options = eframe::NativeOptions::default();
    options.viewport = options.viewport.with_app_id("Matrix Calculator");
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
                    ui.separator();
                    self.show_arithmetic_operations(ui);
                });
        });
    }
}

impl MatrixGui {
    fn show_transposes(&self, ui: &mut egui::Ui) {
        ui.label("Transpose of matrix A:");

        match self.app.get_matrix(&self.app.matrix_a) {
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
        ui.label("Transpose of matrix B:");

        match self.app.get_matrix(&self.app.matrix_b) {
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
    }

    fn show_arithmetic_operations(&self, ui: &mut egui::Ui) {
        ui.label("A + B:");

        if let (Ok(m_a), Ok(m_b)) = (
            self.app.get_matrix(&self.app.matrix_a),
            self.app.get_matrix(&self.app.matrix_b),
        ) {
            match m_a + m_b {
                Ok(m_r) => {
                    for row in &m_r.data {
                        ui.horizontal(|ui| {
                            for v in row {
                                ui.label(v);
                            }
                        });
                    }
                }
                Err(e) => {
                    ui.label(e);
                }
            }
        }
        ui.label("A - B:");

        if let (Ok(m_a), Ok(m_b)) = (
            self.app.get_matrix(&self.app.matrix_a),
            self.app.get_matrix(&self.app.matrix_b),
        ) {
            match m_a - m_b {
                Ok(m_r) => {
                    for row in &m_r.data {
                        ui.horizontal(|ui| {
                            for v in row {
                                ui.label(v);
                            }
                        });
                    }
                }
                Err(e) => {
                    ui.label(e);
                }
            }
        }
        ui.label("B - A:");

        if let (Ok(m_a), Ok(m_b)) = (
            self.app.get_matrix(&self.app.matrix_a),
            self.app.get_matrix(&self.app.matrix_b),
        ) {
            match m_b - m_a {
                Ok(m_r) => {
                    for row in &m_r.data {
                        ui.horizontal(|ui| {
                            for v in row {
                                ui.label(v);
                            }
                        });
                    }
                }
                Err(e) => {
                    ui.label(e);
                }
            }
        }
        ui.label("A x B:");

        if let (Ok(m_a), Ok(m_b)) = (
            self.app.get_matrix(&self.app.matrix_a),
            self.app.get_matrix(&self.app.matrix_b),
        ) {
            match m_a * m_b {
                Ok(m_r) => {
                    for row in &m_r.data {
                        ui.horizontal(|ui| {
                            for v in row {
                                ui.label(v);
                            }
                        });
                    }
                }
                Err(e) => {
                    ui.label(e);
                }
            }
        }
        ui.label("B x A:");

        if let (Ok(m_a), Ok(m_b)) = (
            self.app.get_matrix(&self.app.matrix_a),
            self.app.get_matrix(&self.app.matrix_b),
        ) {
            match m_b * m_a {
                Ok(m_r) => {
                    for row in &m_r.data {
                        ui.horizontal(|ui| {
                            for v in row {
                                ui.label(v);
                            }
                        });
                    }
                }
                Err(e) => {
                    ui.label(e);
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
        // Scoped borrows to satisfy Rust
        {
            let matrix_a = &mut self.app.matrix_a;
            Self::edit_matrix_ui(ui, "Matrix A:", matrix_a);
        }

        ui.separator();

        {
            let matrix_b = &mut self.app.matrix_b;
            Self::edit_matrix_ui(ui, "Matrix B:", matrix_b);
        }
    }
}
