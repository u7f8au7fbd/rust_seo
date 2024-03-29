use crate::seo;
use eframe::egui::ComboBox;

pub fn gui() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([320.0, 240.0])
            .with_resizable(false),
        ..Default::default()
    };

    let mut selected = String::from("https://www.google.com");
    let mut search_text = "".to_owned();
    let mut serialize_type = "toml".to_owned();
    let mut amount = 18;

    eframe::run_simple_native("Rusty SEO App", options, move |main, _| {
        egui::CentralPanel::default().show(main, |ui| {
            ui.spacing_mut().item_spacing = egui::Vec2::new(10.0, 10.0);

            ui.heading("SEO App");

            ui.horizontal(|ui| {
                let name_label = ui.label("Text:");
                ui.text_edit_singleline(&mut search_text)
                    .labelled_by(name_label.id);
            });
            ComboBox::from_label("Serch Engine")
                .selected_text(&selected)
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut selected,
                        String::from("https://www.google.com"),
                        "Google",
                    );
                    ui.selectable_value(
                        &mut selected,
                        String::from("https://www.yahoo.co.jp"),
                        "Yahoo",
                    );
                    ui.selectable_value(
                        &mut selected,
                        String::from("https://www.bing.com"),
                        "Bing",
                    );
                });

            ui.add(egui::Slider::new(&mut amount, 18..=180).text("Amount of results"));

            if ui.button("Run").clicked() {
                println!("Running search for: {}", search_text);
                tokio::task::spawn(async move {
                    seo::test().await.unwrap();
                });
            }

            ComboBox::from_label("Export type:")
                .selected_text(&serialize_type)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut serialize_type, String::from("toml"), "toml");
                    ui.selectable_value(&mut serialize_type, String::from("ron"), "ron");
                    ui.selectable_value(&mut serialize_type, String::from("csv"), "csv");
                });

            if ui.button("Export").clicked() {
                println!("Exporting to: {}", serialize_type);
            }
        });
    })
}
