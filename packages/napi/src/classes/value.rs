use napi_derive::napi;
use napi::bindgen_prelude::*;
use yad_core::core::Value as CoreValue;
use yad_core::constants::types;

#[napi(js_name = "Value")]
pub struct JsValue {
  pub r#type: u8,
  pub byte_length: u8,
  pub bytes: Vec<u8>
}

#[napi]
impl JsValue {
  #[napi]
  pub fn from_u8(num: u8) -> Self {
    Self {
      r#type:
    }
  }
}