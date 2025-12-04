use koto::prelude::*;

pub fn run() {
    let mut koto = Koto::default();
    match koto.compile("1 + 2") {
        Ok(chunk) => match koto.run(chunk) {
            Ok(result) => match result {
                KValue::Number(n) => println!("{n}"), // 3.0
                other => panic!("Unexpected result type: {}", other.type_as_string()),
            },
            Err(runtime_error) => {
                panic!("Runtime error: {runtime_error}");
            }
        },
        Err(compiler_error) => {
            panic!("Compiler error: {compiler_error}");
        }
    }
}
