use rune::termcolor::{ColorChoice, StandardStream};
use rune::{Context, Diagnostics, Source, Sources, Vm};
use std::path::Path;
use std::sync::Arc;

fn create_vm(paths: Vec<impl AsRef<Path>>) -> rune::support::Result<Vm> {
    let context = Context::with_default_modules()?;
    let ctx = Arc::new(Context::runtime(&context)?);

    let mut sources = Sources::new();
    for p in paths {
        let _ = sources.insert(Source::from_path(p)?);
    }

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

pub fn run() -> rune::support::Result<()> {
    let mut vm = create_vm(vec!["scripts/add.rn"])?;
    let output = vm.call(["add"], (10i64, 23i64))?;
    let output: i64 = rune::from_value(output)?;

    println!("{}", output);

    Ok(())
}
