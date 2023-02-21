#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use noir_wallet_lib::{btc, cosmos, eth, evmos};
use noir_wallet_lib::bip39::Mnemonic;
use noir_wallet_lib::ecdsa::KeyPair;
use noir_wallet_lib::error::NoirError;
use serde::Serialize;

#[derive(Serialize)]
pub struct Response<T> where T: Serialize {
  pub success: bool,
  pub result: Option<T>,
  pub message: String,
}

impl<T> Response<T> where T: Serialize {
  pub fn ok(result: T) -> Response<T> {
    Response {
      success: true,
      result: Some(result),
      message: "".to_string(),
    }
  }

  pub fn err(message: String) -> Response<T> {
    Response {
      success: false,
      result: None,
      message,
    }
  }
}

#[tauri::command]
fn generate() -> String {
  Mnemonic::generate().0
}

#[tauri::command]
fn derive(mnemonic: String, path: String, hrp: String) -> Response<(KeyPair, String, String, String, String)> {
  let inner_derive = |mnemonic: &String, path: &String| -> Result<(KeyPair, String, String, String, String), NoirError> {
    let master = KeyPair::from_mnemonic(mnemonic)?;
    let derived = master.derive(path)?;
    let btc = btc::Address::from_public(&derived.public.as_bytes()).to_string();
    let eth = eth::Address::from_public(&derived.public.as_bytes())?.to_string();
    let cosmos = cosmos::Address::from_public(&derived.public.as_bytes())?.to_string(&hrp)?;
    let evmos = evmos::Address::from_public(&derived.public.as_bytes())?.to_string(&hrp)?;
    Ok((derived, btc, eth, cosmos, evmos))
  };

  match inner_derive(&mnemonic, &path) {
    Ok(result) => Response::ok(result),
    Err(err) => Response::err(err.to_string()),
  }
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![generate, derive])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
