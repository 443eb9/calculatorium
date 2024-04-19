use calculatorium_backend::{
    calculator::{CalculationError, Calculator},
    math::{symbol::Number, IntoRawExpr},
};

fn main() {
    let mut calculator = Calculator::default();
    let mut input = String::default();

    loop {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if handle_cmd(&mut calculator, input) {
            continue;
        }

        calculator.set_expr(input);
        let now = std::time::SystemTime::now();
        println!("Start calculating the approximation of {}", input);

        match calculator.approximate() {
            Ok(ok) => println!(
                "Done (after {}s)! Expression ≈ \n{}",
                now.elapsed().unwrap().as_secs_f32(),
                ok
            ),
            Err(err) => {
                let hint = match err {
                    CalculationError::Parsing(e) => e.expand(&input),
                };
                println!("Calculation failed: \n{}", hint);
            }
        };

        println!();
    }
}

fn handle_cmd(calc: &mut Calculator, input: &str) -> bool {
    let cmd = input.split(' ').collect::<Vec<_>>();
    if cmd.is_empty() {
        return false;
    }

    match cmd[0] {
        "set" => {
            if cmd.len() != 3 {
                return false;
            }
            if let Some(n) = Number::parse_raw(cmd[2]) {
                calc.set_variable(cmd[1].to_string(), n);
                println!("Set variable {} to {}", cmd[1], cmd[2]);
            }
            true
        }
        "get" => {
            if cmd.len() != 2 {
                return false;
            }
            match calc.get_variable(&cmd[1]) {
                Some(n) => println!("Found variable {} with value {}", cmd[1], n.assemble()),
                None => println!("Unknown variable {}", cmd[1]),
            }
            true
        }
        "clearvar" => {
            calc.variables_mut().clear();
            println!("Successfully cleared all variables");
            return true;
        }
        _ => false,
    }
}
