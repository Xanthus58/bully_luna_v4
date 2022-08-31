#![allow(clippy::all)]
use std::time::Duration;

use eframe::egui::{self, Visuals, Window};
use egui_notify::{Anchor, Toast, Toasts};

mod api_handler; //Imports the API handler
use api_handler::*;

pub struct MyApp {
    //Enter global values to be used with your app here
    insult: String,
    toasts: Toasts,
    closable: bool,
    duration: f32,
}

impl Default for MyApp {
    //defaults for your global values
    fn default() -> Self {
        Self {
            //enter global default values here
            insult: "".to_string(),
            toasts: Toasts::default().with_anchor(Anchor::TopRight),
            closable: true,
            duration: 3.5,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |_ui| {
            Window::new("Bully Luna V.4!").show(ctx, |ui| {
                ui.style_mut().visuals = Visuals::dark(); // Makes the buttons dark
                ctx.set_visuals(egui::Visuals::dark()); // Make the ui dark
                egui::warn_if_debug_build(ui);

                let cb = |t: &mut Toast| {
                    //Callback for the toast
                    t.set_closable(self.closable)
                        .set_duration(Some(Duration::from_millis((1000. * self.duration) as u64)));
                };

                self.insult = self.insult.replace("\n", "");

                ui.label("Hello and welcome to version 4 of the bully luna program!");
                ui.label(
                "You can randomly generate an insult or write your own below to be sent to luna!",
            );

                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Enter an insult: ");
                    ui.text_edit_multiline(&mut self.insult);
                });

                let generate_button = ui.button("Generate an insult");
                if generate_button.clicked() {
                    self.insult = get_insult();
                    cb(self.toasts.success("Generation Successful!")); //Sends a success toast
                }

                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("Your insult:");
                    ui.label(&self.insult);
                });

                let send_button = ui.button("Send insult to luna");
                if send_button.clicked() {
                    send_message(&self.insult);

                    cb(self.toasts.success("Message Sent!"));
                }
                self.toasts.show(ctx); // Requests to render toasts
            });
        });
    }
}
