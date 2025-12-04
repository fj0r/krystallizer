use anyhow::Result;
use koto::{prelude::*, runtime};
use std::{convert::AsRef, fs::read_to_string, path::Path};

pub fn run(p: impl AsRef<Path>) -> Result<()> {
    let script = read_to_string(p)?;
    let mut koto = Koto::default();
    let prelude = koto.prelude();

    // Standalone functions can be inserted directly
    prelude.insert("say_hello", say_hello);

    // The add_fn helper avoids the need for type annotations
    prelude.add_fn("plus", |ctx| match ctx.args() {
        [KValue::Number(a), KValue::Number(b)] => Ok((a + b).into()),
        unexpected => unexpected_args("|Number, Number|", unexpected),
    });

    let _ = koto.compile_and_run(&script)?;

    let result = koto.call_exported_function("my_fn", &[1.into(), 2.into()])?;
    println!("koto:: {}", koto.value_to_string(result)?);
    Ok(())
}

fn say_hello(ctx: &mut CallContext) -> runtime::Result<KValue> {
    match ctx.args() {
        [] => println!("koto:: Hello?"),
        [KValue::Str(name)] => println!("koto:: Hello {name}"),
        unexpected => return unexpected_args("||, or |String|", unexpected),
    }

    Ok(KValue::Null)
}
