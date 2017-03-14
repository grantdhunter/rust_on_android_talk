#!/usr/bin/env run-cargo-script

use std::process::Command;
use std::path::Path;


fn main() {

    Command::new("cargo").arg("apk").status().unwrap();
    Command::new("adb")
        .args(&["install", "-r", "target/android-artifacts/build/bin/android_rust_talk-debug.apk"])
        .status()
        .unwrap();
}
