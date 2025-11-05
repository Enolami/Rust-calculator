slint::include_modules!();

mod calc;
use calc::CalculatorState;

fn main() {
    let ui = Calculator::new().unwrap();

    let mut state = CalculatorState::new();

    ui.set_display_text(state.current.clone().into());
    ui.set_operation_text(state.history.clone().into());

    let ui_handle = ui.as_weak();

    ui.on_button_pressed(move | text| {
        let ui = ui_handle.upgrade().unwrap();

        state.handle_input(text.as_str());

        ui.set_display_text(state.current.clone().into());
        ui.set_operation_text(state.history.clone().into());
    });

    if let Err(e) = ui.run() {
        eprintln!("Error running the UI: {}", e);
    }
}

