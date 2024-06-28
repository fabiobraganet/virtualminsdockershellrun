use std::fs::OpenOptions;
use std::io::prelude::*;
use chrono::Local;

pub const LOG_FILE_PATH: &str = "/var/log/virtualmin_start.log";
pub const ERR_ROOT_REQUIRED: &str = "Este script deve ser executado como root";
pub const ERR_NETWORK: &str = "Sem conectividade de rede. Verifique sua conexão e tente novamente.";

pub fn log_message(message: &str) {
    let now = Local::now();
    let log_entry = format!("{} - {}\n", now.format("%Y-%m-%d %H:%M:%S"), message);
    println!("{}", log_entry);

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(LOG_FILE_PATH)
        .expect("Não foi possível abrir o arquivo de log.");
    file.write_all(log_entry.as_bytes()).expect("Não foi possível escrever no arquivo de log.");
}

pub fn check_root() {
    if unsafe { libc::geteuid() } != 0 {
        log_message(ERR_ROOT_REQUIRED);
        std::process::exit(1);
    }
}

pub fn check_dependencies(dependencies: &[&str]) {
    for dep in dependencies {
        if which::which(dep).is_err() {
            log_message(&format!("{} não encontrado. Este script requer {}.", dep, dep));
            std::process::exit(1);
        }
    }
}

pub fn check_network() {
    if std::process::Command::new("ping").arg("-c 1").arg("google.com").status().is_err() {
        log_message(ERR_NETWORK);
        std::process::exit(1);
    }
}
