use std::process::Command;
use std::fs::OpenOptions;
use std::io::prelude::*;
use chrono::Local;

fn log_message(message: &str) {
    let now = Local::now();
    let log_entry = format!("{} - {}\n", now.format("%Y-%m-%d %H:%M:%S"), message);
    println!("{}", log_entry); 

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("/var/log/virtualmin_start.log")
        .unwrap();
    file.write_all(log_entry.as_bytes()).unwrap();
}

fn check_and_restart_services() {
    let services = vec!["apache2", "dovecot", "postfix", "clamav-daemon"];
    
    for service in services {
        let status = Command::new("systemctl")
            .arg("is-active")
            .arg("--quiet")
            .arg(service)
            .status()
            .expect("failed to execute process");
        
        if !status.success() {
            log_message(&format!("Serviço {} está parado. Reiniciando...", service));
            let restart = Command::new("systemctl")
                .arg("restart")
                .arg(service)
                .status()
                .expect("failed to execute process");
            
            if restart.success() {
                log_message(&format!("Serviço {} reiniciado com sucesso.", service));
            } else {
                log_message(&format!("Falha ao reiniciar o serviço {}.", service));
            }
        } else {
            log_message(&format!("Serviço {} está em execução.", service));
        }
    }
}

fn main() {
    check_and_restart_services();
}
