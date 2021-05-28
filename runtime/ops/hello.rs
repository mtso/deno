use deno_core::error::AnyError;
use deno_core::Extension;
use deno_core::OpState;
use deno_core::op_sync;

use serde::Deserialize;
use serde::Serialize;

pub fn init() -> Extension {
  Extension::builder()
    .ops(vec![
      ("op_hello_greet", op_sync(op_hello_greet))
    ])
    .build()
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GreetArgs {
  pub name: String,
}

#[derive(Serialize)]
pub struct OpGreet {
  pub message: String,
}

fn op_hello_greet(
  _state: &mut OpState,
  args: GreetArgs,
  _: (),
) -> Result<OpGreet, AnyError> {
  Ok(OpGreet {
    message: format!("Hello, {}", args.name)
  })
}
