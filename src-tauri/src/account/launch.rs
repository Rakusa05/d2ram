use tauri::State;
use std::process::Command;
use crate::{AppState, Region, Account, D2R_PATH};
use crate::handle::{kill_handle};

#[tauri::command]
pub fn launch_account(id: String, region: Region, state: State<AppState>) -> Result<Vec<Account>, String> {
  use std::time::Duration;

  let _ = kill_handle();

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

    let _ = kill_handle();

    Ok(())
  };

  if id.eq_ignore_ascii_case("all") {

    for acc in accounts.iter_mut() {
       std::thread::sleep(Duration::from_millis(600));
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