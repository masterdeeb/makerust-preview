use bevy::prelude::*;

#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub enum CalcAction {
    Digit(u8),
    Op(char),
    Clear,
    Equals,
    Decimal,
}

#[derive(Resource)]
pub struct CalculatorData {
    pub display: String,
    pub prev_value: Option<f64>,
    pub current_op: Option<char>,
    pub reset_display_next: bool,
}

impl Default for CalculatorData {
    fn default() -> Self {
        Self {
            display: "0".to_string(),
            prev_value: None,
            current_op: None,
            reset_display_next: false,
        }
    }
}

pub struct CoreLogicPlugin;

impl Plugin for CoreLogicPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CalculatorData>()
            .add_event::<CalcAction>()
            .add_systems(Update, handle_calc_events);
    }
}

fn handle_calc_events(
    mut events: EventReader<CalcAction>,
    mut calc: ResMut<CalculatorData>,
) {
    for event in events.read() {
        match event {
            CalcAction::Digit(d) => {
                if calc.reset_display_next || calc.display == "0" {
                    calc.display = d.to_string();
                    calc.reset_display_next = false;
                } else {
                    // Prevent display from getting too long
                    if calc.display.len() < 12 {
                        calc.display.push_str(&d.to_string());
                    }
                }
            }
            CalcAction::Decimal => {
                if calc.reset_display_next {
                    calc.display = "0.".to_string();
                    calc.reset_display_next = false;
                } else if !calc.display.contains('.') {
                    calc.display.push('.');
                }
            }
            CalcAction::Clear => {
                *calc = CalculatorData::default();
            }
            CalcAction::Op(op) => {
                if let Ok(current_val) = calc.display.parse::<f64>() {
                    if let (Some(prev), Some(old_op)) = (calc.prev_value, calc.current_op) {
                        if !calc.reset_display_next {
                            let res = calculate(prev, current_val, old_op);
                            calc.display = format_result(res);
                            calc.prev_value = Some(res);
                        }
                    } else {
                        calc.prev_value = Some(current_val);
                    }
                }
                calc.current_op = Some(*op);
                calc.reset_display_next = true;
            }
            CalcAction::Equals => {
                if let Ok(current_val) = calc.display.parse::<f64>() {
                    if let (Some(prev), Some(op)) = (calc.prev_value, calc.current_op) {
                        let res = calculate(prev, current_val, op);
                        calc.display = format_result(res);
                        calc.prev_value = None;
                        calc.current_op = None;
                        calc.reset_display_next = true;
                    }
                }
            }
        }
    }
}

fn calculate(a: f64, b: f64, op: char) -> f64 {
    match op {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => if b != 0.0 { a / b } else { 0.0 },
        _ => b,
    }
}

fn format_result(val: f64) -> String {
    let s = format!("{:.8}", val);
    let trimmed = s.trim_end_matches('0').trim_end_matches('.');
    if trimmed.is_empty() {
        "0".to_string()
    } else {
        trimmed.to_string()
    }
}
