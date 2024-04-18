use calculatorium_backend::calculator::{CalculationError, Calculator};

fn main() {
    let calculator = Calculator::default();
    let mut input = String::default();

    loop {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        let now = std::time::SystemTime::now();
        println!("Start calculating the approximation of {}", input);
        
        match calculator.approximate(&input) {
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
