use steel::SteelVal;
use steel::steel_vm::engine::Engine;

pub fn run() {
    let mut steel_engine = Engine::new();
    let answer = steel_engine.run(
        "(+ 1 2 3 4)
         (+ 5 6 7 8)",
    );
    assert_eq!(answer, Ok(vec![SteelVal::IntV(10), SteelVal::IntV(26)]));
}
