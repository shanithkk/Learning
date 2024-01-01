use std::path::PathBuf;

use rhai::{Engine, EvalAltResult};

pub fn main() -> Result<(), Box<EvalAltResult>>
{
    // Create an 'Engine'
    let engine = Engine::new();
    let s = engine.eval_file::<i64>(PathBuf::from("./file.rhai"));
    println!("{:?}",s);

    let res = engine.eval_file::<Test>(PathBuf::from("./struct.rhai"));
    println!("{:?}",res);
    Ok(())
}

#[derive(Debug, Clone)]
struct Test{
    name: String,
    id : i32
}