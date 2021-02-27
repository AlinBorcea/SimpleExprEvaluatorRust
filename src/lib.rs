
pub struct Config<'a> {
    expr: &'a str,
    alg: &'a str,
}

impl<'a> Config<'a> {
    pub fn new(args: &mut Vec<String>) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Two CLI arguments are needed: Expression AlgType");
        }
        if args[1].len() < 3 {
            return Err("The expression must have at least 3 members!");
        }

        args[1].insert(0, '#');
        args[1].push('#');

        let expr = &args[1];
        let alg: &str;

        if args[2] == "TwoStacks" {
            alg = &args[2];
        
        } else if args[2] == "Polish" {
            alg = &args[2];
        
        } else {
            alg = "TwoStacks";
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
    use super::Config;
    use super::utils;

    pub fn run(config: Config) -> Result<(), &str> {
        let mut operand_stack: Vec<String> = Vec::new();
        let mut operator_stack: Vec<(char, i16)> = Vec::new();
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
                5 => {},
                _ => return Err("Invalid character"),
            }

            println!("{:?}\t\t\t{:?}", operator_stack, operand_stack);

            if offset < 0 {
                return Err("Parentheses do not match!");
            }

        }

        while operator_stack.len() > 0 {
            evaluate(&mut operand_stack, &mut operator_stack);
            println!("{:?}\t\t\t{:?}", operator_stack, operand_stack);
        }

        if offset > 0 {
            return Err("Parentheses do not match!");
        }

        if operand_stack.len() != 1 {
            return Err("Expression lacks an operator!");
        }

        if operator_stack.len() > 2 {
            return Err("Expression lacks variables");
        }

        Ok(())
    }

    fn push_operator<'a>(operand_stack: &mut Vec<String>, operator_stack: &mut Vec<(char, i16)>,
                        it: char, offset: i16) -> Result<(), &'a str> {
        
        let it_priority = utils::operator_priority(it, offset);
        if operator_stack.is_empty() {
            operator_stack.push((it, it_priority));
            return Ok(());
        }

        let (_, last_pr) = operator_stack.last().unwrap();
        if *last_pr < it_priority {
            operator_stack.push((it, it_priority));
            println!("c1");
            return Ok(());
        
        } else if !operand_stack.is_empty() && !operator_stack.is_empty() {
            
            evaluate(operand_stack, operator_stack);
            return push_operator(operand_stack, operator_stack, it, offset);
        
        } else {
            Err("Something")
        }
    }

    fn evaluate(operand_stack: &mut Vec<String>, operator_stack: &mut Vec<(char, i16)>) {
        let last_var = operand_stack.pop().unwrap();
        let mut pre_last_var = operand_stack.pop().unwrap();

        let (last_op, _) = operator_stack.pop().unwrap();
        let op_str = String::from(last_op);

        pre_last_var += &op_str.to_owned();
        pre_last_var += &last_var.to_owned();
        operand_stack.push(pre_last_var);
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
    fn char_types() {
        let c1 = 'Ă';
        let c2 = '1';
        let c3 = 'a';
        let c4 = 't';
        let c5 = 'A';
        let c6 = 'G';
        let c7 = '+';
        let c8 = '-';
        let c9 = '/';
        let c10 = '*';
        let c11 = '(';
        let c12 = ')';

        assert_eq!(utils::char_type(c1), 0);
        assert_eq!(utils::char_type(c2), 0);
        assert_eq!(utils::char_type(c3), 1);
        assert_eq!(utils::char_type(c4), 1);
        assert_eq!(utils::char_type(c5), 1);
        assert_eq!(utils::char_type(c6), 1);
        assert_eq!(utils::char_type(c7), 2);
        assert_eq!(utils::char_type(c8), 2);
        assert_eq!(utils::char_type(c9), 2);
        assert_eq!(utils::char_type(c10), 2);
        assert_eq!(utils::char_type(c11), 3);
        assert_eq!(utils::char_type(c12), 4);
    }

}
