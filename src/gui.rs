use eframe::*;

use crate::{comparison, connect};

pub fn gui() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder {
            maximize_button: Some(false),
            resizable: Some(false),
            inner_size: Some(egui::vec2(320., 320.)),
            ..Default::default()
        },
        ..Default::default()
    };
    eframe::run_simple_native("SEO App", options, move |ctx, _| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.set_pixels_per_point(4.);
            let mut fonts = egui::FontDefinitions::default();
            //フォントデータの設定
            fonts.font_data.insert(
                "Moralerspace".to_owned(),
                egui::FontData::from_static(include_bytes!(
                    ".././assets/fonts/MoralerspaceNeonHWNF.ttf"
                )),
            );
            //フォントファミリーの設定
            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, "Moralerspace".to_owned());

            ctx.set_fonts(fonts);
            //以下よりUIの作成
            if ui.button("比較").clicked() {
                comparison::test();
            }
            if ui.button("探索").clicked() {
                tokio::task::spawn(async move {
                    connect::get_google(
                        "Rust言語".to_string(),
                        "AIzaSyC2lo3mkteyh87zvQy2rfy48b6hgbPd6Vg".to_string(),
                        "570e13bebba514d9d".to_string(),
                    )
                    .await
                    .unwrap();
                });
            }
        });
    })
}
