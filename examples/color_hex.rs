use eframe::NativeOptions;
use egui_typed_input::ValText;

fn main() {
    let mut color = ValText::color_hex();

    eframe::run_simple_native(
        "hex color input",
        NativeOptions::default(),
        move |ctx, _frame| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.text_edit_singleline(&mut color);
                println!("{:?}", color.get_val());
                if let Some(color) = color.get_val() {
                    ui.colored_label(color.clone(), format!("{:?}", color));
                }
            });
        }
    ).unwrap();
}
