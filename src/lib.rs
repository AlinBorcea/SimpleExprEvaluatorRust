
pub enum AlgType {
    TwoStacks,
    Polish,
}

pub struct Config<'a> {
    expression: &'a str,
    alg_type: AlgType,
}

impl<'a> Config<'a> {
    pub fn new(args: &Vec<String>) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Two CLI arguments are needed: Expression AlgType");
        }
        let expression = &args[1];
        let alg_type = get_alg_type(&args[2]);

        if let Err(e) = validate_expression(&expression) {
            return Err(e);
        }

        Ok(Config { expression, alg_type })
    }

    pub fn expression(&self) -> &str {
        self.expression
    }

    pub fn alg_type(&self) -> &str {
        match self.alg_type {
            AlgType::TwoStacks => "TwoStacks",
            AlgType::Polish => "Polish",
        }
    }

}

fn validate_expression(expr: &str) -> Result<(), &str> {
    if expr.len() < 3 {
        return Err("The expression must have at least 3 members!");
    }

    let mut it = expr.chars();
    let mut curent = it.next();
    let mut next = it.next();

    let mut c1: char;
    let mut c2: char;

    while next != None {
        c1 = curent.unwrap_or('`');
        c2 = next.unwrap_or('`');

        if c1 == '`' || c2 == '`' {
            return Err("Iteration error!");
        }

        if !good_neighbors(c1, c2) {
            return Err("There are bad neighbors in your expression!");
        }

        curent = next;
        next = it.next();
    }

    Ok(())
}

fn good_neighbors(c1: char, c2: char) -> bool {
    let t1 = char_type(c1);
    let t2 = char_type(c2);
    return match t1 {
        1 => match t2 {
            2 => true,
            4 => true,
            _ => false,
        }
        2 => match t2 {
            1 => true,
            3 => true,
            _ => false,
        }
        3 => match t2 {
            1 => true,
            3 => true,
            _ => false,
        }
        4 => match t2 {
            2 => true,
            _ => false,
        }
        _ => false,
    }
}

fn char_type(c: char) -> u8 {
    if (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') {
        return 1;
    }
    if c == '+' || c == '-' || c == '*' || c == '/' {
        return 2;
    }
    if c == '(' {
        return 3;
    }
    if c == ')' {
        return 4;
    }

    0
}

fn get_alg_type(alg_type: &str) -> AlgType {
    match alg_type {
        "TwoStacks" => AlgType::TwoStacks,
        "Polish" => AlgType::Polish,
        _ => AlgType::TwoStacks,
    }
}

mod tests {
    use super::*;

    #[test]
    fn short_expression() {
        let expr = String::from("a+");
        assert_ne!(Ok(()), validate_expression(&expr));
    }

    #[test]
    fn empty_expression() {
        let expr = String::from("");
        assert_ne!(Ok(()), validate_expression(&expr));
    }

    #[test]
    fn incorect_expression1() {
        let expr = String::from("a+b/c+");
        assert_eq!(Ok(()), validate_expression(&expr));
    }

    #[test]
    fn incorect_expression2() {
        let expr = String::from("a+b/c)");
        assert_eq!(Ok(()), validate_expression(&expr));
    }

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

        assert_eq!(char_type(c1), 0);
        assert_eq!(char_type(c2), 0);
        assert_eq!(char_type(c3), 1);
        assert_eq!(char_type(c4), 1);
        assert_eq!(char_type(c5), 1);
        assert_eq!(char_type(c6), 1);
        assert_eq!(char_type(c7), 2);
        assert_eq!(char_type(c8), 2);
        assert_eq!(char_type(c9), 2);
        assert_eq!(char_type(c10), 2);
        assert_eq!(char_type(c11), 3);
        assert_eq!(char_type(c12), 4);
    }

}
