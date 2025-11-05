#[derive(Clone, Debug, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub struct CalculatorState {
    pub current: String,
    pub history: String,
    stored: f64,
    op: Option<Op>,
    last_op: Option<Op>,
    last_operand: Option<f64>,
    error: bool,
    evaluated: bool,
}

impl CalculatorState {
    pub fn new() -> Self {
        CalculatorState {
            current: "0".to_string(),
            history: "".to_string(),
            stored: 0.0,
            op: None,
            last_operand: None,
            error: false,
            evaluated: false,
            last_op: None,
        }
    }

    pub fn handle_input(&mut self, input: &str) {
        if self.error && input != "C" && input != "CE" {
            return;
        }

        match input {
            "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
                self.input_digit(input.chars().next().unwrap());
            }
            "." => self.input_decimal(),
            "+" | "−" | "x" | "÷" => self.set_operation(input),
            "=" => self.calculate(),
            "C" => self.clear_all(),
            "CE" => self.clear_entry(),
            "←" => self.backspace(),
            "±" => self.toggle_sign(),
            _ => {}
        }
    }

    fn clear_all(&mut self) {
        *self = CalculatorState::new();
    }

    fn clear_entry(&mut self) {
        self.current = "0".to_string();
        self.error = false;
    }

    fn clear_if_evaluated(&mut self) {
        if self.evaluated {
            self.history = "".to_string();
            self.current = "0".to_string();
            self.evaluated = false;
        }
    }

    fn input_digit(&mut self, digit: char) {
        self.clear_if_evaluated();
        if self.current == "0" {
            self.current = digit.to_string();
        } else {
            self.current.push(digit);
        }
    }

    fn input_decimal(&mut self) {
        self.clear_if_evaluated();
        if !self.current.contains('.') {
            self.current.push('.');
        }
    }

    fn backspace(&mut self) {
        self.clear_if_evaluated();
        self.current.pop();
        if self.current.is_empty() {
            self.current = "0".to_string();
        }
    }

    fn toggle_sign(&mut self) {
        self.clear_if_evaluated();
        if self.current == "0" {
            return;
        }
        if let Ok(num) = self.current.parse::<f64>() {
            self.current = (-num).to_string();
        }
    }

    fn set_operation(&mut self, op_str: &str) {
        if let Some(op) = self.op.clone() {
            self.calculate_internal(&op);
            if self.error {
                return;
            }
        }

        self.evaluated = false;
        self.stored = self.current.parse().unwrap_or(0.0);
        let new_op = Self::parse_op(op_str);
        self.op = Some(new_op.clone());
        self.last_op = Some(new_op.clone());
        self.current = "0".to_string();
        self.last_operand = None;
        self.history = format!("{} {}", self.stored, op_str);
    }

    fn calculate(&mut self) {
        let (b, op_to_use) = if let Some(op) = self.op.clone() {
            let b_val = self.current.parse().unwrap_or(0.0);
            self.last_operand = Some(b_val);
            self.last_op = Some(op.clone());
            self.history = format!("{} {} =",self.history, b_val);
            (b_val, op)
        } else if let Some(last_op) = self.last_op.clone() {
            self.stored = self.current.parse().unwrap_or(0.0);
            let b_val = self.last_operand.unwrap_or(0.0);
            self.history = format!("{} {} {}=",self.stored,match last_op {
                Op::Add => "+",
                Op::Sub => "−",
                Op::Mul => "x",
                Op::Div => "÷",
            },b_val);
            (b_val, last_op)
        } else {
            return;
        };

        if self.op.is_none() {
            self.current = b.to_string();
        }

        self.calculate_internal(&op_to_use);

        self.op = None;
        self.evaluated = true;
    }

    fn calculate_internal(&mut self, op_to_use: &Op) {
        let a = self.stored;
        let b = self.current.parse().unwrap_or(0.0);

        let result = match op_to_use {
            Op::Add => a + b,
            Op::Sub => a - b,
            Op::Mul => a * b,
            Op::Div => {
                if b == 0.0 {
                    self.error = true;
                    self.current = "Error".to_string();
                    self.history = "".to_string();
                    return;
                }
                a / b
            }
        };

        self.current = result.to_string();
        self.stored = result;
    }

    fn parse_op(op_str: &str) -> Op {
        match op_str {
            "+" => Op::Add,
            "−" => Op::Sub,
            "x" => Op::Mul,
            "÷" => Op::Div,
            _ => panic!("Unknown operator"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1_add() {
        // 12 + 3 = -> 15
        let mut state = CalculatorState::new();
        state.handle_input("1");
        state.handle_input("2");
        state.handle_input("+");
        state.handle_input("3");
        state.handle_input("=");
        assert_eq!(state.current, "15");
    }

    #[test]
    fn test_case_2_multiply_by_zero() {
        // 5 × 0 = -> 0
        let mut state = CalculatorState::new();
        state.handle_input("5");
        state.handle_input("x");
        state.handle_input("0");
        state.handle_input("=");
        assert_eq!(state.current, "0");
    }

    #[test]
    fn test_case_3_div_by_zero() {
        // 9 ÷ 0 = -> Error
        let mut state = CalculatorState::new();
        state.handle_input("9");
        state.handle_input("÷");
        state.handle_input("0");
        state.handle_input("=");
        assert_eq!(state.current, "Error");
        assert!(state.error);
        // lock
        state.handle_input("+");
        assert_eq!(state.current, "Error");
        // unlock
        state.handle_input("C");
        assert_eq!(state.current, "0");
        assert!(!state.error);
        state.handle_input("CE");
        assert_eq!(state.current, "0");
        assert!(!state.error);
    }

    #[test]
    fn test_case_4_invalid_decimal() {
        // 1 . . 5 + 2 = -> 3.5
        let mut state = CalculatorState::new();
        state.handle_input("1");
        state.handle_input(".");
        state.handle_input("."); // Ignored
        state.handle_input("5");
        state.handle_input("+");
        state.handle_input("2");
        state.handle_input("=");
        assert_eq!(state.current, "3.5");
    }

    #[test]
    fn test_case_5_repeat_equals() {
        // 5 + 2 = = -> 9
        let mut state = CalculatorState::new();
        state.handle_input("5");
        state.handle_input("+");
        state.handle_input("2");
        state.handle_input("="); // 7
        assert_eq!(state.current, "7");
        state.handle_input("="); // 9
        assert_eq!(state.current, "9"); 
    }

    #[test]
    fn test_case_6_backspace() {
        // 10 ← ← -> 1
        let mut state = CalculatorState::new();
        state.handle_input("1");
        state.handle_input("0");
        state.handle_input("←");
        assert_eq!(state.current, "1");
        state.handle_input("←");
        assert_eq!(state.current, "0"); 
    }
}