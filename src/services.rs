use crate::utils::log_message;
use std::process::Command;

pub const SERVICES: &[&str] = &["apache2", "dovecot", "postfix", "clamav-daemon"];
pub const ERR_VERIFICAR_SERVICO: &str = "Falha ao verificar o serviço.";
pub const ERR_REINICIAR_SERVICO: &str = "Falha ao reiniciar o serviço.";

fn service_exists(service: &str) -> bool {
    Command::new("systemctl")
        .arg("status")
        .arg(service)
        .output()
        .map_or(false, |output| output.status.success())
}

pub fn check_and_restart_services() {
    for &service in SERVICES {
        if !service_exists(service) {
            log_message(&format!("Serviço {} não encontrado.", service));
            continue;
        }

        let status = Command::new("systemctl")
            .arg("is-active")
            .arg("--quiet")
            .arg(service)
            .status()
            .expect(ERR_VERIFICAR_SERVICO);
        
        if !status.success() {
            log_message(&format!("Serviço {} está parado. Reiniciando...", service));
            let restart = Command::new("systemctl")
                .arg("restart")
                .arg(service)
                .status()
                .expect(ERR_REINICIAR_SERVICO);
            
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
