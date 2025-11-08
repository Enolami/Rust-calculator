slint::include_modules!();

mod calc;
use calc::CalculatorState;
use slint::{SharedString, VecModel};
use std::{rc::Rc};

fn main() {
    let ui = Calculator::new().unwrap();

    let mut state = CalculatorState::new();

    ui.set_display_text(state.current.clone().into());
    ui.set_operation_text(state.history.clone().into());
    ui.set_memory_is_set(state.is_memory_set());

    let history_model = Rc::new(VecModel::from(Vec::new()));
    ui.set_history_list(history_model.into());

    let ui_handle = ui.as_weak();

    ui.on_button_pressed(move | text| {
        let ui = ui_handle.upgrade().unwrap();

        state.handle_input(text.as_str());

        ui.set_display_text(state.current.clone().into());
        ui.set_operation_text(state.history.clone().into());
        ui.set_memory_is_set(state.is_memory_set());
        
        let history_vec: Vec<SharedString> = state.full_history.iter().map(SharedString::from).collect();
        let history_model = Rc::new(VecModel::from(history_vec));
        ui.set_history_list(history_model.into());
    });

    if let Err(e) = ui.run() {
        eprintln!("Error running the UI: {}", e);
    }
}

