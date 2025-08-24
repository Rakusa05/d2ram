// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use serde::{Deserialize, Serialize};
use tauri::State;
use std::{path::PathBuf, sync::Mutex};
use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine as _};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
enum Region {
    America,
    Europe,
    Asia,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
struct AccountConfig {
    id: u32,
    displayName: String,
    accountLogin: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct AccountRuntime {
    region: Option<Region>,
    running: bool,
    pid: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Account {
  #[serde(flatten)]
  cfg: AccountConfig,
  #[serde(flatten)]
  rt: AccountRuntime
}

#[allow(non_snake_case)]
#[derive(Serialize,Deserialize, Debug)]
struct AccountInput {
  displayName: String,
  accountLogin: String,
  password: String,
}

struct AppState {
    accounts: Mutex<Vec<Account>>,
}

const SALT: &[u8] = "D2RAM";

fn obfuscate_password(pw: &str) -> String {
    let xored: Vec<u8> = pw
        .as_bytes()
        .iter()
        .enumerate()
        .map(|(i, b)| b ^ SALT[i % SALT.len()])
        .collect();
    format!("x1:{}", STANDARD_NO_PAD.encode(xored))
}

fn deobfuscate_password(pw: &str) -> Result<String, String> {
    let enc = pw.strip_prefix("x1:").ok_or("missing x1: prefix")?;
    let data = STANDARD_NO_PAD.decode(enc).map_err(|e| e.to_string())?;
    let plain: Vec<u8> = data
        .into_iter()
        .enumerate()
        .map(|(i, b)| b ^ SALT[i % SALT.len()])
        .collect();
    String::from_utf8(plain).map_err(|e| e.to_string())
}



fn acc_config_path() -> Result<PathBuf, String> {
  let mut conf = dirs_next::config_dir().ok_or("No Config Dir")?;
  conf.push("D2RAM");
  std::fs::create_dir_all(&conf).map_err(|e| e.to_string())?;
  conf.push("accounts_config.json");
  Ok(conf)
}

fn read_acc_config() -> Result<Vec<AccountConfig>, String> {
  let path = acc_config_path()?;
  let txt = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
  let config = serde_json::from_str::<Vec<AccountConfig>>(&txt).map_err(|e| e.to_string())?;

  Ok(config)
}

fn save_accounts_to_file(accounts: &Vec<Account>) -> Result<(), String> {
  let cfgs: Vec<AccountConfig> = accounts.iter().map(|a| a.cfg.clone()).collect();
  let json = serde_json::to_string_pretty(&cfgs).map_err(|e| e.to_string())?;
  let path = acc_config_path()?;
  std::fs::write(path, json).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_accounts_info() -> Result<Vec<Account>, String> {
  let config = read_acc_config()?;
  let accounts: Vec<Account> = config.into_iter().map(|cfg| Account { cfg, rt: AccountRuntime::default() }).collect();

  Ok(accounts)
}

#[tauri::command]
fn add_account(new_account: AccountInput, state: State<AppState>) -> Result<Vec<Account>, String> {
  let mut accounts = state.accounts.lock().unwrap();

  let new_id = accounts.iter().map(|a| a.cfg.id).max().unwrap_or(0) + 1;

  
  let account = Account {
    cfg: AccountConfig {

      id: new_id,
      displayName: new_account.displayName,
      accountLogin: new_account.accountLogin,
      password: new_account.password,
    },
    rt: AccountRuntime {

      region: None,
      running: false,
      pid: None,
    },
  };

  accounts.push(account);
  save_accounts_to_file(&accounts)?;
  Ok(accounts.clone())
}

#[tauri::command]
fn edit_account(updated_account: AccountInput, id: u32, state: State<AppState>) -> Result<Vec<Account>, String> {
  let mut accounts = state.accounts.lock().unwrap();

  if let Some(account) = accounts.iter_mut().find(|a| a.cfg.id == id) {
    account.cfg.displayName = updated_account.displayName;
    account.cfg.accountLogin = updated_account.accountLogin;
    account.cfg.password = updated_account.password;
  }

  save_accounts_to_file(&accounts)?;
  Ok(accounts.clone())
}

#[tauri::command]
fn delete_account(id: u32, state: State<AppState>) -> Result<Vec<Account>, String> {
  let mut accounts = state.accounts.lock().unwrap();

  if let Some(pos) = accounts.iter().position(|a| a.cfg.id == id) {
    accounts.remove(pos);
  }

  for (i, a) in accounts.iter_mut().enumerate() {
    a.cfg.id = (i as u32) + 1;
  }

  save_accounts_to_file(&accounts)?;
  Ok(accounts.clone())
}



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _platform: &'static str = tauri_plugin_os::platform();

    let initial: Vec<Account> = read_acc_config().unwrap_or_default().into_iter().map(|cfg| Account { cfg, rt: AccountRuntime::default() }).collect();

    tauri::Builder::default()
        .manage(AppState { accounts: Mutex::new(initial)})
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_accounts_info, add_account, edit_account, delete_account])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
