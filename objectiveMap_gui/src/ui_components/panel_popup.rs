use eframe::egui::{self, Color32, Context, Response, Ui};
use crate::ui_components::PanelButton;

pub struct PanelPopup {
    pub label: String,
    pub color: Color32,
    show_popup: bool,
    main_button: PanelButton,
}

impl PanelPopup {
    pub fn new(label: &str, color: Color32) -> Self {
        Self {
            label: label.to_string(),
            color,
            show_popup: false,
            main_button: PanelButton::new(label, color),
        }
    }

    pub fn ui(&mut self, ui: &mut Ui, ctx: &Context) {
        self.show_menu(ctx);

        // if ctx.input(|i| i.pointer.any_pressed()) {
        //     if !ctx.screen_rect().contains(ctx.().pos) {
        //         self.show_popup = false;
        //     }
        // }
    }

    fn create_option_buttons(&mut self, ui: &mut Ui) {
        let options = [
            PanelButton::new("Option 1", self.color),
            PanelButton::new("Option 2", self.color),
            PanelButton::new("Option 3", self.color)
        ];
        for option in options {
            option.ui(ui);
        }
    }

    pub fn show_menu(&mut self, ctx: &Context) {
        if self.show_popup {
            let popup_id = egui::Id::new("menu_popup");
            ctx.request_repaint();
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.horizontal(|ui| {
                        ui.group(|ui| {
                            ui.vertical(|ui| {
                                if self.main_button.ui(ui).clicked() {
                                    println!("Guides");
                                }
                            });
                            if self.show_popup {
                                ui.group(|ui| {
                                    self.create_option_buttons(ui);
                                });
                            }
                        });
                    });
                });
            });
        }
    }
}