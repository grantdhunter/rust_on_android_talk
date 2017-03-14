#!/usr/bin/env run-cargo-script

use std::process::{Command, Stdio, exit};


fn main() {

    let st = Command::new("cargo").arg("apk").status().unwrap();

    if !st.success() {
        exit(1);
    }

    let st = Command::new("adb")
        .arg("install")
        .arg("-r")
        .arg("target/android-artifacts/build/bin/android_rust_talk-debug.apk")
        .status()
        .unwrap();

    if !st.success() {
        exit(1);
    }

    let mut cmd = Command::new("adb")
        .args(&["logcat", "RustAndroidGlueStdouterr:D", "*:S"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    let _ = cmd.wait();
}
