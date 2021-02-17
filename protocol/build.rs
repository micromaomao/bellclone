use std::process::{Command, Stdio};

fn generate_fbs(paths: &[&str]) -> Result<(), String> {
  for path in paths {
    println!("cargo:rerun-if-changed={}", path);
  }
  let mut res = Command::new("flatc")
    .arg("--rust")
    .arg("-o")
    .arg("src/")
    .args(paths)
    .stderr(Stdio::inherit())
    .spawn()
    .map_err(|e| format!("Unable to exec flatc: {}", e))?;
  let res = res
    .wait()
    .map_err(|e| format!("Unable to \"wait\": {}", e))?;
  if !res.success() {
    Err(format!("flatc exited with code {}", res))
  } else {
    Ok(())
  }
}

fn main() -> Result<(), String> {
  generate_fbs(&["src/base.fbs", "src/servermsg.fbs", "src/clientmsg.fbs"])?;
  Ok(())
}
