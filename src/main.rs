mod db;
mod qr;
mod totp;

struct RsOTP {
    accounts: Vec<db::Account>,
    conn: rusqlite::Connection,
    show_add: bool,
    add_label: String,
    add_issuer: String,
    add_secret: String,
    copied_feedback: String,
    feedback_timer: f32,
    status_msg: String,
    status_timer: f32,
}

impl Default for RsOTP {
    fn default() -> Self {
        let conn = db::init_db();
        let accounts = db::list_accounts(&conn);
        RsOTP {
            accounts,
            conn,
            show_add: false,
            add_label: String::new(),
            add_issuer: String::new(),
            add_secret: String::new(),
            copied_feedback: String::new(),
            feedback_timer: 0.0,
            status_msg: String::new(),
            status_timer: 0.0,
        }
    }
}

impl eframe::App for RsOTP {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint_after(std::time::Duration::from_millis(500));

        let dt = ctx.input(|i| i.unstable_dt);
        self.feedback_timer -= dt;
        if self.feedback_timer <= 0.0 {
            self.copied_feedback.clear();
        }
        self.status_timer -= dt;
        if self.status_timer <= 0.0 {
            self.status_msg.clear();
        }

        let dropped = ctx.input(|i| i.raw.dropped_files.clone());
        for file in &dropped {
            if let Some(path) = &file.path {
                let p = path.to_string_lossy().to_string();
                if qr::is_image_file(&p) {
                    match qr::parse_qr(&p) {
                        Some(auth) => {
                            self.show_add = true;
                            self.add_label = auth.label;
                            self.add_issuer = auth.issuer;
                            self.add_secret = auth.secret;
                            self.status_msg = "QR code detected!".to_string();
                            self.status_timer = 3.0;
                        }
                        None => {
                            self.status_msg = "No QR code found in image".to_string();
                            self.status_timer = 3.0;
                        }
                    }
                }
            }
        }

        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("RsOTP Codes");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("+").clicked() {
                        self.show_add = !self.show_add;
                        if self.show_add {
                            self.add_label.clear();
                            self.add_issuer.clear();
                            self.add_secret.clear();
                        }
                    }
                });
            });
            if !self.status_msg.is_empty() {
                ui.label(&self.status_msg);
            }
        });

        if self.show_add {
            egui::CentralPanel::default()
                .frame(egui::Frame::none().fill(ui_dark_bg(ctx)))
                .show(ctx, |ui| {
                    egui::Frame::group(ui.style())
                        .fill(ui_dark_bg(ctx))
                        .show(ui, |ui| {
                            ui.heading("Add Account");
                            ui.separator();

                            ui.horizontal(|ui| {
                                ui.label("Label:");
                                ui.text_edit_singleline(&mut self.add_label);
                            });
                            ui.horizontal(|ui| {
                                ui.label("Issuer:");
                                ui.text_edit_singleline(&mut self.add_issuer);
                            });
                            ui.horizontal(|ui| {
                                ui.label("Secret (Base32):");
                                ui.text_edit_singleline(&mut self.add_secret);
                            });

                            ui.separator();
                            ui.label("Or drop a QR code image (PNG/JPG)");
                            ui.separator();

                            ui.horizontal(|ui| {
                                if ui
                                    .add_enabled(!self.add_secret.is_empty(), egui::Button::new("Save"))
                                    .clicked()
                                {
                                    let label = if self.add_label.is_empty() { "Unnamed" } else { &self.add_label };
                                    let issuer = if self.add_issuer.is_empty() { "" } else { &self.add_issuer };
                                    let secret = self.add_secret.trim().replace(' ', "").to_uppercase();
                                    db::add_account(&self.conn, label, issuer, &secret);
                                    self.accounts = db::list_accounts(&self.conn);
                                    self.show_add = false;
                                }
                                if ui.button("Cancel").clicked() {
                                    self.show_add = false;
                                }
                            });
                        });
                });
            return;
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(ui_dark_bg(ctx)))
            .show(ctx, |ui| {
                if self.accounts.is_empty() {
                    ui.vertical_centered(|ui| {
                        ui.add_space(100.0);
                        ui.label("No accounts");
                        ui.label("Click + to add one");
                        ui.add_space(10.0);
                        ui.label("or drop a QR code image");
                    });
                    return;
                }

                let mut delete_id: Option<i64> = None;

                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        for acc in &self.accounts {
                            let code = totp::generate_totp(&acc.secret, acc.period, acc.digits);
                            let remaining = totp::remaining_seconds(acc.period);
                            let progress = remaining as f32 / acc.period as f32;

                            egui::Frame::group(ui.style())
                                .fill(ui_card_bg(ctx))
                                .rounding(8.0)
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.vertical(|ui| {
                                            let display = if acc.issuer.is_empty() {
                                                acc.label.clone()
                                            } else {
                                                format!("{} ({})", acc.label, acc.issuer)
                                            };
                                            ui.label(egui::RichText::new(&display).size(12.0).color(egui::Color32::GRAY));
                                            let code_color = if remaining < 5 {
                                                egui::Color32::from_rgb(255, 80, 80)
                                            } else {
                                                egui::Color32::WHITE
                                            };
                                            let resp = ui.label(
                                                egui::RichText::new(&code)
                                                    .size(28.0)
                                                    .monospace()
                                                    .color(code_color)
                                                    .strong(),
                                            );
                                            if resp.clicked() {
                                                if let Ok(mut clipboard) = arboard::Clipboard::new() {
                                                    let _ = clipboard.set_text(code.clone());
                                                }
                                                self.copied_feedback = format!("Copied: {}", code);
                                                self.feedback_timer = 2.0;
                                            }
                                        });
                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                            if ui.button("x").clicked() {
                                                delete_id = Some(acc.id);
                                            }
                                        });
                                    });
                                    let bar = egui::ProgressBar::new(progress)
                                        .desired_width(ui.available_width())
                                        .fill(if remaining < 5 {
                                            egui::Color32::from_rgb(255, 80, 80)
                                        } else {
                                            egui::Color32::from_rgb(0, 180, 80)
                                        });
                                    ui.add(bar);
                                    if !self.copied_feedback.is_empty() {
                                        ui.label(&self.copied_feedback);
                                    }
                                });
                            ui.add_space(6.0);
                        }
                    });

                if let Some(id) = delete_id {
                    db::delete_account(&self.conn, id);
                    self.accounts = db::list_accounts(&self.conn);
                }
            });
    }
}

fn ui_dark_bg(ctx: &egui::Context) -> egui::Color32 {
    ctx.style().visuals.window_fill()
}

fn ui_card_bg(ctx: &egui::Context) -> egui::Color32 {
    let base = ctx.style().visuals.window_fill();
    egui::Color32::from_rgb(
        base.r().saturating_add(10),
        base.g().saturating_add(10),
        base.b().saturating_add(15),
    )
}

fn main() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([380.0, 600.0])
            .with_min_inner_size([300.0, 400.0])
            .with_title("RsOTP Codes"),
        ..Default::default()
    };

    eframe::run_native(
        "RsOTP Codes",
        native_options,
        Box::new(|_cc| Box::new(RsOTP::default())),
    )
}
