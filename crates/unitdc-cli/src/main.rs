use unitdc::interpreter::{Interpreter, Output};

fn main() {
    let mut interpreter = Interpreter::new(Box::new(|output| match output {
        Output::Message(e) => eprintln!("message: {}", e),
        Output::Quantity(q) => println!("[0]: {}", q),
        Output::QuantityList(mut q) => {
            q.reverse();
            for (i, q) in q.iter().enumerate() {
                println!("[{}]: {}", i, q)
            }
        }
    }));

    interpreter
        .run_str(include_str!("../../../unitdc.rc"))
        .expect("unitdc.rc should run");

    for line in std::io::stdin().lines() {
        let line = line.expect("line should exist");
        if let Err(e) = interpreter.run_str(&line) {
            eprintln!("{}", e)
        }
    }
}
