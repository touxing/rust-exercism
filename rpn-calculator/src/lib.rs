#[derive(Debug, Clone)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    //   unimplemented!(
    // 	"Given the inputs: {:?}, evaluate them as though they were a Reverse Polish notation expression",
    // 	inputs,
    // );
    if inputs.is_empty() {
        return None;
    }

    let mut stack: Vec<i32> = Vec::new();
    for input in inputs {
        match input {
            CalculatorInput::Value(val) => stack.push(*val),
            _ => {
                if stack.len() < 2 {
                    return None;
                }

                let val1 = stack.pop().unwrap();
                let val2 = stack.pop().unwrap();

                match input {
                    CalculatorInput::Add => stack.push(val2 + val1),
                    CalculatorInput::Subtract => stack.push(val2 - val1),
                    CalculatorInput::Multiply => stack.push(val2 * val1),
                    CalculatorInput::Divide => stack.push(val2 / val1),
                    _ => return None,
                }
            }
        }
    }

    if stack.len() != 1 {
        return None;
    }
    stack.pop()
}
