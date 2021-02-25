
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
    Ok(())
}

fn get_alg_type(alg_type: &str) -> AlgType {
    match alg_type {
        "TwoStacks" => AlgType::TwoStacks,
        "Polish" => AlgType::Polish,
        _ => AlgType::TwoStacks,
    }
}

mod tests {
    #[test]
    fn test_config_empty_vec() {

    }
}
