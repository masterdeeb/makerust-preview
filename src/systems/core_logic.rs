use bevy::prelude::*;
use crate::ui::theme::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Component, Clone, Copy, Debug)]
pub enum ButtonAction {
    Digit(u8),
    Op(Operation),
    Clear,
    Calculate,
}

#[derive(Component)]
pub struct CalculatorDisplay;

#[derive(Resource)]
pub struct CalculatorState {
    pub display_text: String,
    pub previous_value: Option<f64>,
    pub current_op: Option<Operation>,
    pub new_input: bool,
}

impl Default for CalculatorState {
    fn default() -> Self {
        Self {
            display_text: "0".to_string(),
            previous_value: None,
            current_op: None,
            new_input: true,
        }
    }
}

pub struct CoreLogicPlugin;

impl Plugin for CoreLogicPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CalculatorState>()
            .add_systems(Update, (button_interaction_system, update_display_system));
    }
}

fn button_interaction_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut state: ResMut<CalculatorState>,
) {
    for (interaction, mut color, action) in &mut interaction_query {
        let is_op = matches!(action, ButtonAction::Op(_) | ButtonAction::Calculate | ButtonAction::Clear);
        
        match *interaction {
            Interaction::Pressed => {
                *color = if is_op { BUTTON_OP_PRESSED.into() } else { BUTTON_PRESSED.into() };
                
                match action {
                    ButtonAction::Digit(d) => {
                        if state.new_input {
                            state.display_text = d.to_string();
                            state.new_input = false;
                        } else {
                            if state.display_text == "0" {
                                state.display_text = d.to_string();
                            } else {
                                state.display_text.push_str(&d.to_string());
                            }
                        }
                    }
                    ButtonAction::Clear => {
                        state.display_text = "0".to_string();
                        state.previous_value = None;
                        state.current_op = None;
                        state.new_input = true;
                    }
                    ButtonAction::Op(op) => {
                        if let Ok(current_val) = state.display_text.parse::<f64>() {
                            if let (Some(prev), Some(current_op)) = (state.previous_value, state.current_op) {
                                if !state.new_input {
                                    let result = calculate(prev, current_val, current_op);
                                    state.display_text = result.to_string();
                                    state.previous_value = Some(result);
                                }
                            } else {
                                state.previous_value = Some(current_val);
                            }
                        }
                        state.current_op = Some(*op);
                        state.new_input = true;
                    }
                    ButtonAction::Calculate => {
                        if let Ok(current_val) = state.display_text.parse::<f64>() {
                            if let (Some(prev), Some(op)) = (state.previous_value, state.current_op) {
                                let result = calculate(prev, current_val, op);
                                state.display_text = result.to_string();
                                state.previous_value = None;
                                state.current_op = None;
                                state.new_input = true;
                            }
                        }
                    }
                }
            }
            Interaction::Hovered => {
                *color = if is_op { BUTTON_OP_HOVERED.into() } else { BUTTON_HOVERED.into() };
            }
            Interaction::None => {
                *color = if is_op { BUTTON_OP_NORMAL.into() } else { BUTTON_NORMAL.into() };
            }
        }
    }
}

fn calculate(a: f64, b: f64, op: Operation) -> f64 {
    match op {
        Operation::Add => a + b,
        Operation::Subtract => a - b,
        Operation::Multiply => a * b,
        Operation::Divide => if b != 0.0 { a / b } else { 0.0 },
    }
}

fn update_display_system(
    state: Res<CalculatorState>,
    mut query: Query<&mut Text, With<CalculatorDisplay>>,
) {
    if state.is_changed() {
        for mut text in &mut query {
            text.sections[0].value = state.display_text.clone();
        }
    }
}
