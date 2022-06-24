/// Code for the evaluation of infix expressions
/// (by using postfix notation aka Reverse Polish Notation).

// enums for brackets
#[derive(Clone)]
enum Bracket {
    Open,
    Close
}
// enums for single input operators
#[derive(Clone)]
enum OneInOperator {
    SquareRoot,
    Negative
}
impl OneInOperator {
    fn apply(&self, num: f64) -> f64 {
        match self {
            OneInOperator::SquareRoot => return num.sqrt(),
            OneInOperator::Negative => return -1.0 * num
        }
    }
}
// enums for two input operators
#[derive(Clone)]
enum TwoInOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
}
impl TwoInOperator {
    fn apply(&self, left_num: f64, right_num: f64) -> f64 {
        match self {
            TwoInOperator::Add => return left_num + right_num,
            TwoInOperator::Subtract => return left_num - right_num,
            TwoInOperator::Multiply => return left_num * right_num,
            TwoInOperator::Divide => return left_num / right_num,
            TwoInOperator::Power => return left_num.powf(right_num)
        }
    }
}
#[derive(Clone)]
enum Operator {
    OneInOperator(OneInOperator),
    TwoInOperator(TwoInOperator),
    Bracket(Bracket)
}
enum Token {
    Operator(Operator),
    Number(f64)
}

/// Converts a char to an associated Operator.
/// 
/// Returns None if no associated Operator is found.
fn char_to_operator(ch: &char) -> Option<Operator> {
    match ch {
        '+' => return Some(Operator::TwoInOperator(TwoInOperator::Add)),
        '×' => return Some(Operator::TwoInOperator(TwoInOperator::Multiply)),
        '-' => return Some(Operator::TwoInOperator(TwoInOperator::Subtract)),
        '÷' => return Some(Operator::TwoInOperator(TwoInOperator::Divide)),
        '^' => return Some(Operator::TwoInOperator(TwoInOperator::Power)),
        '√' => return Some(Operator::OneInOperator(OneInOperator::SquareRoot)),
        '(' => return Some(Operator::Bracket(Bracket::Open)),
        ')' => return Some(Operator::Bracket(Bracket::Close)),
        _ => return None
    }
}

/// Converts a char to an associated constant value.
/// 
/// Returns None if no associated value is found.
fn char_to_value(ch: &char) -> Option<f64> {
    match ch {
        'e' => return Some(std::f64::consts::E),
        'π' => return Some(std::f64::consts::PI),
        _ => return None
    }
}

/// Class for managing a stack of Operators
struct OperatorStack {
    stack: Vec<Operator>,
    highest_priority: i32
}
impl OperatorStack {
    pub fn new() -> Self {
        return Self {
            stack: Vec::new(),
            highest_priority: -1
        };
    }

    /// Compute priority level of an Operator. (BEDMAS precedence)
    fn get_priority(op: &Operator) -> i32 {
        match op {
            Operator::TwoInOperator(inside) => {
                match inside {
                    TwoInOperator::Add|TwoInOperator::Subtract => return 1,
                    TwoInOperator::Multiply|TwoInOperator::Divide => return 2,
                    TwoInOperator::Power => return 3,
                }
            }
            Operator::OneInOperator(_) => return 4,
            _ => return 0
        }
    }
    
    /// Attempt to push an Operator onto the stack, returns any
    /// Operators that were popped off in order to push. 
    /// 
    /// Pop details: element [0] of output would be the first 
    ///              Operator popped off when pushing
    fn push(&mut self, op: Operator) -> Vec<Operator> {
        let mut output: Vec<Operator> = Vec::new();
        let curr_priority = Self::get_priority(&op);

        match &op {
            // Upon brackets
            Operator::Bracket(direction) => {
                match direction {
                    // For open bracket, reset precedence
                    Bracket::Open => {
                        self.highest_priority = 0;
                        self.stack.push(op);
                    }
                    // For close bracket, pop operators off until
                    // open bracket is met
                    Bracket::Close => { 
                        loop {
                            let stack_top = self.stack.pop();
                            match stack_top {
                                None|Some(Operator::Bracket(Bracket::Open)) => break,
                                Some(_) => output.push(stack_top.unwrap())
                            }
                        }
                        // recalculate highest priority
                        if self.stack.last().is_some() {
                            self.highest_priority = OperatorStack::get_priority(self.stack.last().unwrap());
                        } else {
                            self.highest_priority = -1;
                        }
                    }
                }
            }

            // Upon two input operators, worry about precedence and pop off lower precedence operators
            Operator::TwoInOperator(_) => {
                // pop any higher/equal precedence operators off the stack
                // and put them onto the output
                while self.highest_priority >= curr_priority {
                    let stack_top = self.stack.pop();
                    output.push(stack_top.unwrap());
    
                    if self.stack.last().is_some() {          
                        self.highest_priority = Self::get_priority(self.stack.last().unwrap());
                    } else {
                        self.highest_priority = -1;
                    }
                }
    
                // put new operator onto stack, update priority
                self.highest_priority = curr_priority;
                self.stack.push(op);
            }

            // Upon single input operators, don't worry about precedence and push straight to stack
            Operator::OneInOperator(_) => {
                self.stack.push(op);
                self.highest_priority = curr_priority;
            }
        }
        return output;
    }
}

// Datatype to represent a postfix expression
type Postfix = Vec<Token>;

/// Converts an infix string expression to Postfix
/// Infix expression assumed to contain no spaces.
fn infix_to_postfix(expr: &String) -> Postfix {
    let mut output: Postfix = Vec::new();
    let mut operator_stack: OperatorStack = OperatorStack::new();
    let mut numerics_buffer: String = String::from("");
    // track if previous token was an operator.
    // helps to distinguish if "-" means subtract or negative
    let mut prev_token_is_op = true;

    // go through each char in postfix string
    for ch in expr.chars() {
        // if char is digit or decimal then append to numerics buffer
        if ch.is_digit(10) || ch == '.'{ 
            numerics_buffer.push(ch);
            prev_token_is_op = false;
        }
        else { // char now must be operator or symbol constant
            // convert numerics_buffer into f64 and place onto output
            if !numerics_buffer.is_empty() {
                output.push(Token::Number(numerics_buffer.parse::<f64>().unwrap()));
                numerics_buffer.clear();
            }

            let mut potential_op = char_to_operator(&ch);
            if potential_op.is_some() { // check if char is a valid operator char
                
                // upon a subtract Operator, decide if it should be interpreted
                // as a negative Operator
                if prev_token_is_op && matches!(potential_op, Some(Operator::TwoInOperator(TwoInOperator::Subtract))) {
                    potential_op = Some(Operator::OneInOperator(OneInOperator::Negative));
                }
                
                // place associated Operator enum onto operator stack
                let pop_offs: Vec<Operator> = operator_stack.push(potential_op.unwrap());
                if !pop_offs.is_empty() {
                    // append any Operators popped off onto RPN output
                    for op in pop_offs {
                        output.push(Token::Operator(op));
                    }
                }
                prev_token_is_op = true;
            } else { // char now must be a symbol constant
                output.push(Token::Number(char_to_value(&ch).unwrap()));
                prev_token_is_op = false;
            }
        }
    }

    // convert any remaining numeric buffer and dump onto output
    if !numerics_buffer.is_empty() {
        output.push(Token::Number(numerics_buffer.parse::<f64>().unwrap()));
    }
    // append remaining operators in operator_stack onto output
    for op in operator_stack.stack.iter().rev() {
        output.push(Token::Operator(op.clone()));
    }
    return output;
}

/// Reads a Postfix expression and evaluates the final answer.
fn evaluate_postfix(expr: &Postfix) -> Option<f64> {
    let mut working_stack: Vec<f64> = Vec::new();
    for token in expr.iter() {
        match token {
            // upon a number, push it to working_stack
            Token::Number(num) => {
                working_stack.push(*num);
            }
            // upon an operator, apply it to working_stack
            Token::Operator(op) => {
                // grab top of working_stack
                let mut pop_off = working_stack.pop();
                if pop_off.is_none() {
                    return None;
                }
                let right_number = pop_off.unwrap();
        
                match op { 
                    // check 1 input operators
                    Operator::OneInOperator(inside) => working_stack.push(inside.apply(right_number)),

                    // check 2 input operators
                    Operator::TwoInOperator(inside) => {
                        // grab top of working_stack again
                        pop_off = working_stack.pop();
                        if pop_off.is_none() {
                            return None;
                        }
                        let left_number = pop_off.unwrap();
                        working_stack.push(inside.apply(left_number, right_number));
                    },
                    // this should be impossible to trigger?
                    _ => return None
                }
            }
        }
    }
        
    // multiple numbers could still be left on the stack due
    // to postfix expressions having implied multiplication
    // e.g ab = a*b
    while working_stack.len() > 1 {
        let right_number: f64 = working_stack.pop().unwrap();
        let left_number: f64 = working_stack.pop().unwrap();
        working_stack.push(left_number*right_number);
    }
    // return answer
    return working_stack.last().copied();
}

// Evaluates the answer to an infix string expression
pub fn evaluate_infix_expr(expr: &String) -> Option<f64> {
    return evaluate_postfix(&infix_to_postfix(expr));
}


#[cfg(test)]
mod tests {
    // Note: On windows, the  #![windows_subsystem = "windows"]
    //       in main.rs will block any print outputs to the terminal,
    //       including output from tests (not sure about other OSs).
    //       Comment out  #![..  to read output if needed.
    use super::*;

    #[test]
    fn basic_expressions() {
        let expr_and_ans = [
            ("363.2+5.6", 368.8),
            ("486.4-154.2", 332.2),
            ("1272.5×4", 5090.0),
            ("32÷10", 3.2),
            ("√16", 4.0),
            ("4^3.5", 128.0)
        ];
        for item in expr_and_ans {
            assert_eq!(evaluate_infix_expr(&String::from(item.0)).unwrap(), item.1);
        }
    }
    #[test]
    fn complex_order_of_operations() {
        let expr_and_ans = [
            ("3.6+(23.2-6×3^2÷3+5)×1.5", 18.9)
        ];
        for item in expr_and_ans {
            assert_eq!(evaluate_infix_expr(&String::from(item.0)).unwrap(), item.1);
        }
    }
    #[test]
    fn negative_numbers() { 
        let expr_and_ans = [
            ("36+-4.5", 31.5),
            ("-12×-2", 24.0),
            ("48.2--15.8", 64.0),
            ("-20+6×-(5÷2)", -35.0)
        ];
        for item in expr_and_ans {
            assert_eq!(evaluate_infix_expr(&String::from(item.0)).unwrap(), item.1);
        }
    }
    #[test]
    fn one_input_operator_chaining() {
        // a chain of one input operators should be applied in the reverse at which
        // they appear LTR (i.e the very inside is applied first)
        assert_eq!(evaluate_infix_expr(&String::from("-√25")).unwrap(), -5.0);
        // complex number should result in f64::NAN
        assert!(evaluate_infix_expr(&String::from("√-36")).unwrap().is_nan())
    }
}

