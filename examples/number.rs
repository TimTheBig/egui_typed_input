use eframe::NativeOptions;
use egui_typed_input::ValText;

fn main() {
    let mut int: ValText<i32, _> = ValText::number_int();
    let mut uint: ValText<u32, _> = ValText::number_uint();
    let mut float: ValText<f32, _> = ValText::number();

    eframe::run_simple_native(
        "number input",
        NativeOptions::default(),
        move |ctx, _frame| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.label("int");
                ui.text_edit_singleline(&mut int);
                println!("int: {:?}", int.get_val());
                ui.label("unsigned int");
                ui.text_edit_singleline(&mut uint);
                println!("uint: {:?}", uint.get_val());
                ui.label("float");
                ui.text_edit_singleline(&mut float);
                println!("float: {:?}", float.get_val());
            });
        },
    ).unwrap();
}
