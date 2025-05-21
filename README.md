# egui-typed-input

Input checked and type safe parsed egui text inputs, allowing you to have typed inputs and focus on your ux not parsing.

## Usage
```rust
use egui_typed_input::ValText;

fn main() {
    let mut alphabetical_order: ValText<Vec<char>, ()> = ValText::new(
        // parser
        (|str| Ok(str.chars().collect::<Vec<_>>())),
        // input validator
        (|current_text, input, index| {
            if input.chars().all(|c| c.is_ascii_alphabetic()) {
                input.chars().all(|c| {
                    c.to_ascii_lowercase() >= current_text.chars().skip(index.saturating_sub(1)).take(1).last().unwrap_or('a')
                })
            } else { false }
        }),
    );

    eframe::run_simple_native(
    "typed input",
    eframe::NativeOptions::default(),
    move |ctx, _frame| {
            egui::CentralPanel::default().show(ctx, |ui| {
            ui.text_edit_singleline(&mut alphabetical_order);
            println!("alphabetical_order: {:?}", alphabetical_order.get_val());
        });
    }
    ).unwrap();
}
```