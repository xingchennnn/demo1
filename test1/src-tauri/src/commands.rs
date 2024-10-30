#[tauri::command]
pub fn close(close : String )-> String {
  format!("close window {}" , close);
  println!("close window {}" , close);
  println!("I was invoked from JavaScript! ---------------{}" , close);

  // 调用系统命令,执行睡眠
  #[cfg(target_os = "windows")]
  let output = std::process::Command::new("rundll32.exe")
      .arg("powrprof.dll,SetSuspendState")
      .output()
      .expect("failed to execute process");
  
  #[cfg(target_os = "linux")]
  let output = std::process::Command::new("systemctl")
      .arg("suspend")
      .output()
      .expect("failed to execute process");
  
  println!("Output: {}", std::str::from_utf8(&output.stdout).unwrap());
  


  format!("close window {}" , close)
}