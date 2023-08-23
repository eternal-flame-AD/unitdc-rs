mod utils;

use js_sys::Function;
use unitdc::interpreter::{Interpreter, Output};
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

static mut INTERPRETER: Option<Interpreter> = None;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: String);
}

/// Runs a string in the interpreter.
#[wasm_bindgen]
pub fn unitdc_input(input: String) -> Result<(), JsValue> {
    unsafe {
        if let Some(ref mut interpreter) = INTERPRETER {
            interpreter.run_str(&input).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

/// Initializes the interpreter, taking a callback function to send output to.
#[wasm_bindgen]
pub fn unitdc_init(js_output: Function) {
    unsafe {
        INTERPRETER = Some(Interpreter::new(Box::new(move |output| match output {
            Output::Quantity(q) => {
                js_output
                    .call2(
                        &JsValue::NULL,
                        &JsValue::from("quantity"),
                        &serde_wasm_bindgen::to_value(&q).unwrap(),
                    )
                    .unwrap();
            }
            Output::QuantityList(q) => {
                js_output
                    .call2(
                        &JsValue::NULL,
                        &JsValue::from("quantity_list"),
                        &serde_wasm_bindgen::to_value(&q).unwrap(),
                    )
                    .unwrap();
            }
            Output::Message(e) => {
                js_output
                    .call2(&JsValue::NULL, &JsValue::from("message"), &JsValue::from(e))
                    .unwrap();
            }
        })));
        INTERPRETER
            .as_mut()
            .unwrap()
            .run_str(include_str!("../../../unitdc.rc"))
            .unwrap();
    }
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    #[cfg(feature = "console_log")]
    console_log::init().expect("could not initialize logger");
    set_panic_hook();

    Ok(())
}
