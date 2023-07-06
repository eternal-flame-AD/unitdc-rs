use std::sync::Mutex;

use num_traits::ToPrimitive;
use unitdc::{
    interpreter::Interpreter,
    quantity::units::{BaseUnit, UnitCombo, UnitExponent},
};

#[test]
fn test_interpreter() {
    let outputs = Mutex::new(Vec::new());
    let output_fn = |output| outputs.lock().unwrap().push(output);
    let mut interpreter = Interpreter::new(Box::new(output_fn));
    interpreter
        .run_str("@base(m)")
        .expect("command should succeed");
    interpreter
        .run_str("0 (m) 1e3 @derived(km)")
        .expect("command should succeed");
    interpreter
        .run_str("2 (km) p")
        .expect("command should succeed");

    let output = outputs.lock().unwrap().pop().expect("output should exist");
    match output {
        unitdc::interpreter::Output::Quantity(q) => {
            assert_eq!(q.number.to_f64().unwrap(), 2000.0);
            assert_eq!(
                q.unit,
                UnitCombo(vec![UnitExponent {
                    unit: BaseUnit {
                        symbol: "m".to_string(),
                    },
                    exponent: 1,
                },])
            );
        }
        _ => panic!("output should be a quantity"),
    }

    interpreter
        .run_str("2 (km) 1 (m) + p")
        .expect("command should succeed");
    let output = outputs.lock().unwrap().pop().expect("output should exist");
    match output {
        unitdc::interpreter::Output::Quantity(q) => {
            assert_eq!(q.number.to_f64().unwrap(), 2001.0);
            assert_eq!(
                q.unit,
                UnitCombo(vec![UnitExponent {
                    unit: BaseUnit {
                        symbol: "m".to_string(),
                    },
                    exponent: 1,
                },])
            );
        }
        _ => panic!("output should be a quantity"),
    }
}
