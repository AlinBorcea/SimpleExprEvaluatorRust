
pub struct Config<'a> {
    expr: &'a str,
    alg: &'a str,
}

impl<'a> Config<'a> {
    /// Config::new(&Vec<String>)creates a new Config with the CLI arguments.
    /// 
    /// One argument is required, the expression.
    /// 
    /// The second argument is the algorithm to be used and it has a default value of "TwoStacks".
    /// 
    /// Config::new(&Vec<String>) returns a result: a Config struct or an error message.
    pub fn new(args: &Vec<String>) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("One CLI argument is required: Expression");
        }
        if args[1].len() < 3 {
            return Err("The expression must have at least 3 members!");
        }

        let expr = &args[1];
        let alg: &str;

        if args.len() < 3 {
            alg = "TwoStacks";
        
        } else {
            if args[2] == "Polish" {
                alg = &args[2];
        
            } else {
                alg = "TwoStacks";
            }
        }

        Ok(Config { expr, alg })
    }

    pub fn expression(&self) -> &str {
        self.expr
    }

    pub fn alg_type(&self) -> &str {
        self.alg
    }

}

pub mod two_stacks {
    use std::process;

    use super::Config;
    use super::utils;

    struct Operator {
        symbol: char,
        priority: i16,
    }

    impl Operator {
        fn print_operator(&self) {
            print!("{}{}, ", self.symbol, self.priority);
        }
    }

    fn print_operator_stack(stack: &Vec<Operator>) {
        for it in stack.iter() {
            it.print_operator();
        }
    }

    pub fn run<'a>(config: &Config) -> Result<(), &'a str> {
        let mut operand_stack: Vec<String> = Vec::new();
        let mut operator_stack: Vec<Operator> = Vec::new();
        let mut offset: i16 = 0;

        for it in config.expression().chars() {
            match utils::char_type(it) {
                1 => operand_stack.push(String::from(it)),
                2 => {
                    if let Err(e) = push_operator(&mut operand_stack, &mut operator_stack, it, offset) {
                        return Err(e);
                    }
                },
                3 => offset += 10,
                4 => offset -= 10,
                _ => return Err("Invalid character"),
            }

            print_operator_stack(&operator_stack);
            println!("\t\t{:?}", operand_stack);

            if offset < 0 {
                return Err("Parentheses do not match!");
            }

        }

        while operator_stack.len() > 0 {
            if let Err(e) = evaluate(&mut operand_stack, &mut operator_stack) {
                return Err(e);
            }
            print_operator_stack(&operator_stack);
            println!("\t\t{:?}", operand_stack);
        }

        if offset > 0 {
            return Err("Parentheses do not match!");
        }

        if operand_stack.len() != 1 {
            return Err("Expression lacks an operator");
        }

        if operator_stack.len() > 0 {
            return Err("Expression lacks variables");
        }

        if  operand_stack[0] != config.expr {
            return Err("The expression is unordered");
        }

        Ok(())
    }

    fn push_operator<'a>(operand_stack: &mut Vec<String>, operator_stack: &mut Vec<Operator>,
                        it: char, offset: i16) -> Result<(), &'a str> {
        
        let operator = Operator { symbol: it, priority: utils::operator_priority(it, offset) };

        if operator_stack.is_empty() {
            operator_stack.push(operator);
            return Ok(());
        }

        let last_operator = operator_stack.last().unwrap();

        if last_operator.priority < operator.priority {
            operator_stack.push(operator);
            return Ok(());
        
        } else if !operand_stack.is_empty() && !operator_stack.is_empty() {
            
            if let Err(e) = evaluate(operand_stack, operator_stack) {
                return Err(e);
            }
            return push_operator(operand_stack, operator_stack, it, offset);
        
        } else {
            Err("Error pushing operator")
        }
    }

    fn evaluate<'a>(operand_stack: &mut Vec<String>, operator_stack: &mut Vec<Operator>) -> Result<(), &'a str> {
        let empty_operand_clo = || { 
            println!("Expression lacks an operator");
            process::exit(3);
        };

        let empty_operator_clo = || { 
            println!("Expression lacks a variable");
            process::exit(3);
        };
        
        let last_var = operand_stack.pop().unwrap_or_else(empty_operand_clo);
        let mut pre_last_var = operand_stack.pop().unwrap_or_else(empty_operand_clo);

        let operator = operator_stack.pop().unwrap_or_else(empty_operator_clo);
        let op_str = String::from(operator.symbol);

        pre_last_var += &op_str.to_owned();
        pre_last_var += &last_var.to_owned();

        if operator_stack.len() > 0 {
            let last_op = operator_stack.last().unwrap();
            if operator.priority - last_op.priority > 1 {
                pre_last_var.insert(0, '(');
                pre_last_var.push(')');
            }
        }

        operand_stack.push(pre_last_var);
        Ok(())
    }

}

mod utils {
    pub fn char_type(ch: char) -> u8 {
        if (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z') {
            return 1;
        }
        if ch == '+' || ch == '-' || ch == '*' || ch == '/' {
            return 2;
        }
        if ch == '(' {
            return 3;
        }
        if ch == ')' {
            return 4;
        }
        if ch == '#' {
            return 5
        }
    
        0
    }

    pub fn operator_priority(op: char, offset: i16) -> i16 {
        return match op {
            '+' => 1 + offset,
            '-' => 1 + offset,
            '*' => 2 + offset,
            '/' => 2 + offset,
            _ => 0,
        };
    }

}

mod tests {
    use super::*;

    #[test]
    fn two_stacks1() {
        let expr = String::from("aaa");
        let args = vec![String::from(""), expr];

        let cfg = Config::new(&args).unwrap();

        assert_ne!(Ok(()), two_stacks::run(&cfg));
    }

    #[test]
    fn two_stacks2() {
        let expr = String::from("++*");
        let args = vec![String::from(""), expr];

        let cfg = Config::new(&args).unwrap();

        assert_ne!(Ok(()), two_stacks::run(&cfg));
    }

    #[test]
    fn two_stacks3() {
        let expr = String::from("--a");
        let args = vec![String::from(""), expr];

        let cfg = Config::new(&args).unwrap();

        assert_ne!(Ok(()), two_stacks::run(&cfg));
    }

    #[test]
    fn two_stacks4() {
        let expr = String::from("ab*");
        let args = vec![String::from(""), expr];

        let cfg = Config::new(&args).unwrap();

        assert_ne!(Ok(()), two_stacks::run(&cfg));
    }

    #[test]
    fn two_stacks5() {
        let expr = String::from("a(b");
        let args = vec![String::from(""), expr];

        let cfg = Config::new(&args).unwrap();

        assert_ne!(Ok(()), two_stacks::run(&cfg));
    }

    #[test]
    fn two_stacks6() {
        let expr = String::from("a(b)");
        let args = vec![String::from(""), expr];

        let cfg = Config::new(&args).unwrap();

        assert_ne!(Ok(()), two_stacks::run(&cfg));
    }

    #[test]
    fn two_stacks7() {
        let expr = String::from("a+(b)");
        let args = vec![String::from(""), expr];

        let cfg = Config::new(&args).unwrap();

        assert_ne!(Ok(()), two_stacks::run(&cfg));
    }

    #[test]
    fn two_stacks8() {
        let expr = String::from("a*(b)");
        let args = vec![String::from(""), expr];

        let cfg = Config::new(&args).unwrap();

        assert_ne!(Ok(()), two_stacks::run(&cfg));
    }

    #[test]
    fn two_stacks9() {
        let expr = String::from("a+(b*c-d/e)");
        let args = vec![String::from(""), expr];

        let cfg = Config::new(&args).unwrap();

        assert_eq!(Ok(()), two_stacks::run(&cfg));
    }

    #[test]
    fn two_stacks10() {
        let expr = String::from("a+(b*c-d/e)");
        let args = vec![String::from(""), expr];

        let cfg = Config::new(&args).unwrap();

        assert_ne!(Ok(()), two_stacks::run(&cfg));
    }

}
