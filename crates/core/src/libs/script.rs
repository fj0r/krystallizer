use rune::termcolor::{ColorChoice, StandardStream};
use rune::{Context, Diagnostics, Source, Sources, Vm};
use std::sync::Arc;

fn create_vm() -> rune::support::Result<Vm> {
    let context = Context::with_default_modules()?;
    let ctx = Arc::new(Context::runtime(&context)?);

    let mut sources = Sources::new();
    let _ = sources.insert(Source::memory("pub fn add(a, b) { a + b }")?);

    let mut diagnostics = Diagnostics::new();

    let result = rune::prepare(&mut sources)
        .with_context(&context)
        .with_diagnostics(&mut diagnostics)
        .build();

    if !diagnostics.is_empty() {
        let mut writer = StandardStream::stderr(ColorChoice::Always);
        diagnostics.emit(&mut writer, &sources)?;
    }

    let unit = result?;
    let vm = Vm::new(ctx, unit.into());
    Ok(vm)
}

pub fn run_script() -> rune::support::Result<()> {
    let mut vm = create_vm()?;
    let output = vm.call(["add"], (10i64, 23i64))?;
    let output: i64 = rune::from_value(output)?;

    println!("{}", output);

    Ok(())
}
