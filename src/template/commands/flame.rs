use std::process::{Command, Stdio};

use crate::template::Day;

pub fn handle(day: Day, debug: bool, dhat: bool) {
    let mut cmd_args = vec![
        "flamegraph".to_string(),
        "--bin".to_string(),
        day.to_string(),
    ];

    if dhat {
        cmd_args.extend([
            "--profile".to_string(),
            "dhat".to_string(),
            "--features".to_string(),
            "dhat-heap".to_string(),
        ]);
    }

    if !debug {
        cmd_args.push("--release".to_string());
    }

    cmd_args.push("--".to_string());
    cmd_args.push("--time".to_string());

    let mut cmd = Command::new("cargo")
        .args(&cmd_args)
        .env("CARGO_PROFILE_RELEASE_DEBUG", "true")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    cmd.wait().unwrap();
}
