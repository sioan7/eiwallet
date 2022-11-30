use layer_1::did_key_from_pk;
use model::JWK;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct EiwalletApp {
    data: PersistentData,
    #[serde(skip)]
    session: SessionData,
}

#[derive(Default, Deserialize, Serialize)]
struct PersistentData {
    pub ed25519_key: Option<JWK>,
    pub did: Option<String>,
}

#[derive(Default)]
struct SessionData {
    pub allowed_to_close: bool,
    pub show_confirmation_dialog: bool,
}

impl EiwalletApp {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        let visuals = egui::Visuals::light();
        cc.egui_ctx.set_visuals(visuals);
        cc.egui_ctx.set_fonts(egui::FontDefinitions::default());

        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for EiwalletApp {
    fn on_close_event(&mut self) -> bool {
        self.session.show_confirmation_dialog = true;
        self.session.allowed_to_close
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("eiwallet");
            ctx.settings_ui(ui);

            if let Some(key) = self.data.ed25519_key.as_ref() {
                ui.horizontal_top(|ui| {
                    ui.label("ed25519 key");
                    let mut code = serde_json::to_string_pretty(&key).unwrap();
                    ui.code_editor(&mut code);
                });

                if let Some(did) = self.data.did.as_mut() {
                    ui.horizontal_centered(|ui| {
                        ui.label("DID");

                        ui.code_editor(did);
                    });
                } else if ui.button("Generate DID").clicked() {
                    self.data.did = Some(did_key_from_pk(key));
                }
            } else if ui.button("Generate ed25519 key").clicked() {
                self.data.ed25519_key = JWK::generate_ed25519().ok();
            }
        });

        if self.session.show_confirmation_dialog {
            egui::Window::new("Do you want to quit?")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("No").clicked() {
                            self.session.show_confirmation_dialog = false;
                        }

                        if ui.button("Yes").clicked() {
                            self.session.allowed_to_close = true;
                            frame.close();
                        }
                    });
                });
        }
    }
}
