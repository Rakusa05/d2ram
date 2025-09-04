use tauri::State;
use std::process::Command;
use crate::{AppState, Region, Account, ConnType, Language};

#[tauri::command]
pub fn launch_account(id: String, region: Region, state: State<AppState>) -> Result<Vec<Account>, String> {
    use std::time::Duration;

    let _ = crate::handle::kill_handle();

    let (game_path, game_lang) = {
        let s = state.settings.lock().unwrap();
        (s.game_path.clone(), s.game_language.clone())
    };

    let mut accounts = state.accounts.lock().unwrap();

    let server_addr = match region {
        Region::America => "us.actual.battle.net",
        Region::Europe  => "eu.actual.battle.net",
        Region::Asia    => "kr.actual.battle.net",
    };

    let region_code = match region {
        Region::America => "US",
        Region::Europe  => "EU",
        Region::Asia    => "KR",
    };

    let locale_opt: Option<&'static str> = match game_lang {
        Language::Default => None,
        Language::English => Some("enUS"),
        Language::French  => Some("frFR"),
    };

    let launch_one = |acc: &mut Account| -> Result<(), String> {
        if acc.rt.running {
            return Ok(());
        }

        let args: Vec<String> = match acc.cfg.connectionType {
            ConnType::Login => {
                let username = acc.cfg.accountLogin.clone();
                let password = acc.cfg.password.clone();

                vec![
                    "-username".into(), username,
                    "-password".into(), password,
                    "-address".into(),  server_addr.into(),
                ]
            }

            ConnType::Token => {
                let token = acc.cfg.token.clone();
                if token.trim().is_empty() {
                    return Err(format!("Account {} has an empty token", acc.cfg.id));
                }

                write_bnet_launch_options(&token, region_code)?;

                let mut v = vec!["-launch".into(), "-uid".into(), "OSI".into()];
                if let Some(loc) = locale_opt {
                    v.push("-locale".into());
                    v.push(loc.into());
                }
                v
            }
        };

        let child = Command::new(&game_path)
            .args(&args)
            .spawn()
            .map_err(|e| format!("Failed to launch D2R for id {}: {}", acc.cfg.id, e))?;

        acc.rt.running = true;
        acc.rt.region  = Some(region.clone());
        acc.rt.pid     = Some(child.id());

        let _ = crate::handle::kill_handle();
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

#[cfg(target_os = "windows")]
fn write_bnet_launch_options(token: &str, region_code: &str) -> Result<(), String> {
    use std::process::Command;

    // Same DPAPI call you tested; writes REG_BINARY (WEB_TOKEN) + REG_SZ (REGION).
    const PS_SCRIPT: &str = r#"
$token  = $env:__D2R_TOKEN
$region = $env:__D2R_REGION

[void][System.Reflection.Assembly]::LoadWithPartialName("System.Security")

$entropy = [byte[]](0xc8,0x76,0xf4,0xae,0x4c,0x95,0x2e,0xfe,0xf2,0xfa,0x0f,0x54,0x19,0xc0,0x9c,0x43)

$bytes = [Text.Encoding]::UTF8.GetBytes($token)
$enc   = [Security.Cryptography.ProtectedData]::Protect(
            $bytes, $entropy,
            [Security.Cryptography.DataProtectionScope]::CurrentUser
         )

$path = 'HKCU:\SOFTWARE\Blizzard Entertainment\Battle.net\Launch Options\OSI'
if (-not (Test-Path $path)) { New-Item -Path $path -Force | Out-Null }

Set-ItemProperty -Path $path -Name 'WEB_TOKEN' -Value $enc    -Type Binary
Set-ItemProperty -Path $path -Name 'REGION'    -Value $region -Type String
"#;

    let out = Command::new("powershell.exe")
        .arg("-NoProfile")
        .arg("-NonInteractive")
        .arg("-Command")
        .arg(PS_SCRIPT)
        .env("__D2R_TOKEN", token)
        .env("__D2R_REGION", region_code)
        .output()
        .map_err(|e| format!("Failed to start PowerShell: {e}"))?;

    if !out.status.success() {
        // include stderr to help debug
        let stderr = String::from_utf8_lossy(&out.stderr);
        return Err(format!("PowerShell registry write failed: {}", stderr.trim()));
    }
    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn write_bnet_launch_options(_token: &str, _region_code: &str) -> Result<(), String> {
    Err("Token mode is only supported on Windows.".into())
}
