use eframe::*;

pub fn gui() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_simple_native("SEO App", options, move |ctx, _| {
        egui::CentralPanel::default().show(ctx, |ui| {
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
            if ui.button("実行").clicked() {
                println!("Run");
            }
        });
    })
}
