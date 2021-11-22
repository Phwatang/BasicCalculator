struct OperatorStack {
    stack: Vec<char>,
    highest_priority: i32
}

impl OperatorStack {
    pub fn new() -> Self {
        return Self {
            stack: Vec::new(),
            highest_priority: -1
        };
    }
    /// Checks if char is a supported operator that can be placed to the stack
    fn check_if_operator(token: char) -> bool {
        match token {
            '+'|'-'|'*'|'/'|'^'|'√' => return true,
            _ => false
        }
    }

    /// Compute priority level of an operator. (BEDMAS precedence)
    fn get_priority(token: Option<&char>) -> i32 {
        match token {
            None => return -1,
            Some('+')|Some('-') => return 1,
            Some('*')|Some('/') => return 2,
            Some('^')|Some('√') => return 3,
            // Rest of the cases should be digits or symbol constants
            _ => return 0
        }
    }

    /// Attempt to push char to stack, returns any chars
    /// that were popped off in order to push. i.e char at [0]
    /// would be the first char popped off.
    pub fn push(&mut self, token: char) -> Option<String> {
        let mut output_buffer = String::from("");
        let current_priority = Self::get_priority(Some(&token));

        if token == '(' { // reset precedence upon open bracket
            self.highest_priority = 0;
            self.stack.push(token);
            return None;
        }
        else if token == ')' { // pop tokens off from stack upon close bracket
            let mut buffer_top = self.stack.pop();
            loop {
                match buffer_top {
                    None|Some('(') => break,
                    Some(_) => output_buffer.push(buffer_top.unwrap())
                }
                buffer_top = self.stack.pop();
            }
            // recalculate highest priority
            self.highest_priority = OperatorStack::get_priority(self.stack.last());
            // check output_buffer incase of situtations like "()".
            if output_buffer.is_empty() {
                return None
            }
            else {
                return Some(output_buffer);
            }
        }

        // pop any higher/equal precedence operators off the buffer
        // and put them onto the output
        while self.highest_priority >= current_priority {
            let buffer_top = self.stack.pop();
            output_buffer.push(buffer_top.unwrap());            
            self.highest_priority = Self::get_priority(self.stack.last());
        }
        // put new operator onto buffer, update priority
        self.highest_priority = current_priority;
        self.stack.push(token);
        // check nothing was added to output_buffer
        if output_buffer.is_empty() {
            return None
        }
        else {
            return Some(output_buffer);
        }
    }
}

/// Computes the RPN expression of a string containing an infix expression.
/// Infix expression assumed to contain no spaces.
pub fn postfix_to_RPN(expr: &String) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    let mut operator_stack: OperatorStack = OperatorStack::new();
    let mut numerics_buffer: String = String::from("");

    // go through each char in postfix string
    for token in expr.chars() {
        // if token is digit or decimal then append to numerics buffer
        if token.is_digit(10) || token == '.'{ 
            numerics_buffer.push(token);
        }
        else { // token now must be operator or symbol constant
            // dump numerics_buffer into a single element onto output
            if !numerics_buffer.is_empty() {
                output.push(numerics_buffer.clone());
                numerics_buffer.clear();
            }
            // check if token is operator
            if OperatorStack::check_if_operator(token) == true {
                // push operator onto operator_stack
                let pop_offs: Option<String> = operator_stack.push(token);
                if pop_offs.is_some() {
                    // place any operators popped off onto output
                    output.push(pop_offs.unwrap());
                }
            } else { // token now must be a symbol constant
                output.push(String::from(token));
            }
        }
    }
    // dump rest of numeric buffer onto output
    if !numerics_buffer.is_empty() {
        output.push(numerics_buffer);
    }
    // append rest of operators in operator_stack onto the output
    for token in operator_stack.stack.iter().rev() {
        output.push(String::from(token.clone()));
    }
    return output;
}

/// Consumes a Vec<String> containing a RPN expression and
/// evaluates the final answer.
pub fn evaluate_RPN(expr: &Vec<String>) -> Option<f64> {
    let mut working_stack: Vec<f64> = Vec::new();
    for item in expr.iter() {
        if item.parse::<f64>().is_ok() { // if item is number
            working_stack.push(item.parse::<f64>().unwrap());
        }
        else { // otherwise must be operator or symbol constant
            let token = item.as_str();
            match token { // check constants
                "π" => working_stack.push(std::f64::consts::PI),
                "e" => working_stack.push(std::f64::consts::E),
                _ => { // must be 1 or 2 input operators now
                    // grab top of working_stack
                    let mut pop_off = working_stack.pop();
                    if pop_off.is_none() {
                        return None;
                    }
                    let right_number = pop_off.unwrap();
                    match token { // check 1 input operators
                        "√" => working_stack.push(right_number.sqrt()),
                        _ => { // must be 2 input operators now
                            // grab top of working_stack again
                            pop_off = working_stack.pop();
                            if pop_off.is_none() {
                                return None;
                            }
                            let left_number = pop_off.unwrap();
                            match token {
                                "+" => working_stack.push(left_number+right_number),
                                "-" => working_stack.push(left_number-right_number),
                                "/" => working_stack.push(left_number/right_number),
                                "*" => working_stack.push(left_number*right_number),
                                "^" => working_stack.push(left_number.powf(right_number)),
                                _ => break
                            }
                        }
                    }
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