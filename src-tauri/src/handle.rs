use std::{process::Command, thread, time::Duration};
use regex::Regex;

pub fn kill_handle() -> Result<(), String> {
    let mut handle_path = dirs_next::config_dir().ok_or("No Config Dir")?;
    handle_path.push("D2RAM");
    handle_path.push("handle64.exe");

    let run_search = || -> Result<String, String> {
        let out = Command::new(&handle_path)
            .args([
        "-accepteula", "-a", "-p", "D2R.exe", "Check For Other Instances", "-nobanner",
    ])
            .output()
            .map_err(|e| e.to_string())?;
        let mut s = String::from_utf8_lossy(&out.stdout).to_string();
        if !out.stderr.is_empty() {
            s.push_str(&String::from_utf8_lossy(&out.stderr));
        }
        Ok(s)
    };

    let mut output = run_search()?;
    for _ in 0..3 {
        if output.is_empty() || output.to_ascii_lowercase().contains("no matching handles found") {
            thread::sleep(Duration::from_millis(150));
            output = run_search()?;
        } else {
            break;
        }
    }

    let re = Regex::new(
        r"(?mi)pid:\s+(\d+)\s+type:\s+Event\s+([0-9A-Fa-f]+):.*Check For Other Instances"
    ).unwrap();

    if let Some(caps) = re.captures(&output) {
        let pid = caps.get(1).unwrap().as_str().trim();
        let handle = caps.get(2).unwrap().as_str().trim();

        // requires elevation
        let _ = Command::new(&handle_path)
            .args(["-c", handle, "-p", pid, "-y"])
            .status()
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}
