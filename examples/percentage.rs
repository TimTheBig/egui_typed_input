use eframe::NativeOptions;
use egui_typed_input::ValText;

fn main() {
    let mut percentage_uint: ValText<u32, _> = ValText::percentage_uint();
    let mut percentage_float: ValText<f64, _> = ValText::<f64, _>::percentage();

    eframe::run_simple_native(
        "percentage input",
        NativeOptions::default(),
        move |ctx, _frame| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.label("unsigned int");
                ui.text_edit_singleline(&mut percentage_uint);
                println!("uint: {:?}", percentage_uint.get_val());
                ui.label("float");
                ui.text_edit_singleline(&mut percentage_float);
                println!("float: {:?}", percentage_float.get_val());

                if let Some(Ok(percentage)) = percentage_uint.get_val() {
                    assert!(*percentage <= 100);
                }
                if let Some(Ok(percentage)) = percentage_float.get_val() {
                    assert!(*percentage <= 100.0);
                }
            });
        }
    ).unwrap();
}
