use deno_core::error::AnyError;
use deno_core::Extension;
use deno_core::OpState;
use deno_core::op_sync;
// use deno_core::ResourceId;

use serde::Deserialize;
use serde::Serialize;

pub fn init() -> Extension {
  Extension::builder()
    .ops(vec![
      ("op_foo_hello", op_sync(op_foo_hello))
    ])
    .build()
}

// #[derive(Deserialize)]
// #[derive(Serialize)]
// #[serde(rename_all = "camelCase")]
// pub struct OpFoo {
//   pub message: String,
//   // pub rid: ResourceId,
//   // pub remote_addr: Option<OpAddr>,
//   // pub local_addr: Option<OpAddr>,
// }

// #[derive(Deserialize)]
// struct ListenArgs {
//   transport: String,
//   #[serde(flatten)]
//   transport_args: ArgsEnum,
// }

fn op_foo_hello(
  _state: &mut OpState,
  _str: String,
  _: (),
) -> Result<String, AnyError> {
  Ok("42".to_string())
}
