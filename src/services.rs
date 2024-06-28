use crate::utils::log_message;
use std::process::Command;

pub const SERVICES: &[&str] = &["apache2", "dovecot", "postfix", "clamav-daemon"];
pub const MSG_SERVICO_PARADO: &str = "Serviço {} está parado. Reiniciando...";
pub const MSG_SERVICO_REINICIADO: &str = "Serviço {} reiniciado com sucesso.";
pub const MSG_SERVICO_FALHA_REINICIAR: &str = "Falha ao reiniciar o serviço {}.";
pub const MSG_SERVICO_EXECUTANDO: &str = "Serviço {} está em execução.";
pub const ERR_VERIFICAR_SERVICO: &str = "Falha ao verificar o serviço.";
pub const ERR_REINICIAR_SERVICO: &str = "Falha ao reiniciar o serviço.";

pub fn check_and_restart_services() {
    for &service in SERVICES {
        let status = Command::new("systemctl")
            .arg("is-active")
            .arg("--quiet")
            .arg(service)
            .status()
            .expect(ERR_VERIFICAR_SERVICO);
        
        if !status.success() {
            log_message(&format!("{} {}", MSG_SERVICO_PARADO, service));
            let restart = Command::new("systemctl")
                .arg("restart")
                .arg(service)
                .status()
                .expect(ERR_REINICIAR_SERVICO);
            
            if restart.success() {
                log_message(&format!("{} {}", MSG_SERVICO_REINICIADO, service));
            } else {
                log_message(&format!("{} {}", MSG_SERVICO_FALHA_REINICIAR, service));
            }
        } else {
            log_message(&format!("{} {}", MSG_SERVICO_EXECUTANDO, service));
        }
    }
}
