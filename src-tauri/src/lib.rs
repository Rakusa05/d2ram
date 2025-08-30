// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use serde::{Deserialize, Serialize};
use sysinfo::{System, Pid};
use tauri::State;
use std::{path::PathBuf, process::Command, sync::Mutex};
use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine as _};

const D2R_PATH: &str = r"C:\Program Files (x86)\DIIR\D2R.exe";

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

const SALT: &[u8] = b"D2RAM";

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
  let mut cfgs = serde_json::from_str::<Vec<AccountConfig>>(&txt).map_err(|e| e.to_string())?;

  for c in &mut cfgs {
    if c.password.starts_with("x1:") {
      if let Ok(p) = deobfuscate_password(&c.password) {
        c.password = p;
      }
    }
  }

  Ok(cfgs)
}

fn save_accounts_to_file(accounts: &Vec<Account>) -> Result<(), String> {
  let cfgs: Vec<AccountConfig> = accounts.iter().map(|a| {
    let mut c = a.cfg.clone();

    if !c.password.starts_with("x1:") {
      c.password = obfuscate_password(&c.password);
    }
    c
  }).collect();

  let json = serde_json::to_string_pretty(&cfgs).map_err(|e| e.to_string())?;
  let path = acc_config_path()?;
  std::fs::write(path, json).map_err(|e| e.to_string())
}

fn update_runtime(accounts: &mut [Account]) {
  let mut sys = System::new();
  sys.refresh_processes();

  for a in accounts.iter_mut() {
    if let Some(pid) = a.rt.pid {
      if sys.process(Pid::from_u32(pid)).is_none() {
        a.rt.running = false;
        a.rt.region = None;
        a.rt.pid = None;
      }
    }
  }
}

#[tauri::command]
fn get_accounts_info(state: State<AppState>) -> Result<Vec<Account>, String> {
    let mut guard = state.accounts.lock().unwrap();
    update_runtime(&mut guard);
    Ok(guard.clone())
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

#[tauri::command]
fn launch_account(id: String, region: Region, state: State<AppState>) -> Result<Vec<Account>, String> {
  use std::time::Duration;

  let mut accounts = state.accounts.lock().unwrap();

  let server_addr = match region {
    Region::America => "us.actual.battle.net",
    Region::Europe  => "eu.actual.battle.net",
    Region::Asia    => "kr.actual.battle.net",
  };

  let launch_one = |acc: &mut Account| -> Result<(), String> {
    if acc.rt.running {
      return Ok(());
    }

    let args = [
      "-username", &acc.cfg.accountLogin,
      "-password", &acc.cfg.password,
      "-address",  server_addr,
    ];

    let child = Command::new(D2R_PATH)
      .args(args)
      .spawn()
      .map_err(|e| format!("Failed to launch D2R for id {}: {}", acc.cfg.id, e))?;

    acc.rt.running = true;
    acc.rt.region  = Some(region.clone());
    acc.rt.pid     = Some(child.id());
    Ok(())
  };

  if id.eq_ignore_ascii_case("all") {

    for acc in accounts.iter_mut() {
       std::thread::sleep(Duration::from_millis(500));
      let _ = launch_one(acc);
    }
  } else {
    let id_num: u32 = id.parse().map_err(|_| format!("Invalid id: {}", id))?;
    let acc = accounts.iter_mut()
      .find(|a| a.cfg.id == id_num)
      .ok_or_else(|| format!("Account {} not found", id_num))?;
    launch_one(acc)?;
  }

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
        .invoke_handler(tauri::generate_handler![get_accounts_info, add_account, edit_account, delete_account, launch_account])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
