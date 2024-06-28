use crate::utils::{log_message, check_root, check_dependencies, check_network};
use std::process::Command;

pub const VIRTUALMIN_INSTALL_PATH: &str = "/usr/sbin/virtualmin";
pub const VIRTUALMIN_INSTALL_URL: &str = "http://software.virtualmin.com/gpl/scripts/install.sh";
pub const MSG_VIRTUALMIN_JA_INSTALADO: &str = "Virtualmin já está instalado";
pub const MSG_VIRTUALMIN_INICIADO_SUCESSO: &str = "Virtualmin iniciado com sucesso.";
pub const MSG_VIRTUALMIN_FALHA_TENTATIVAS: &str = "O Virtualmin falhou mesmo depois de muitas tentativas de iniciá-lo.";
pub const MSG_VIRTUALMIN_BAIXANDO: &str = "Virtualmin - Baixando o instalador";
pub const MSG_VIRTUALMIN_ERRO_BAIXAR: &str = "Erro ao baixar o instalador do Virtualmin.";
pub const MSG_VIRTUALMIN_EXECUTANDO: &str = "Virtualmin - Executando o instalador";
pub const MSG_VIRTUALMIN_ERRO_INSTALAR: &str = "Erro durante a instalação do Virtualmin.";
pub const MSG_VIRTUALMIN_INSTALADO_SUCESSO: &str = "Virtualmin instalado com sucesso";
pub const ERR_TORNAR_EXECUTAVEL: &str = "Não foi possível tornar o instalador executável.";

pub fn install_or_start_virtualmin() {
    check_root();
    check_dependencies(&["systemctl", "wget"]);
    check_network();
    
    if std::path::Path::new(VIRTUALMIN_INSTALL_PATH).exists() {
        log_message(MSG_VIRTUALMIN_JA_INSTALADO);
        start_virtualmin();
    } else {
        install_virtualmin();
    }
}

fn start_virtualmin() {
    let max_attempts = 5;

    for attempt in 0..max_attempts {
        if Command::new("systemctl").arg("start").arg("webmin").status().is_ok() {
            log_message(MSG_VIRTUALMIN_INICIADO_SUCESSO);
            return;
        } else {
            log_message(&format!("Tentativa {} falhou. Tentando novamente em 5 segundos...", attempt + 1));
            std::thread::sleep(std::time::Duration::from_secs(5));
        }
    }

    log_message(MSG_VIRTUALMIN_FALHA_TENTATIVAS);
}

fn install_virtualmin() {
    log_message(MSG_VIRTUALMIN_BAIXANDO);
    if Command::new("wget").arg(VIRTUALMIN_INSTALL_URL).arg("-O").arg("/root/install.sh").status().is_err() {
        log_message(MSG_VIRTUALMIN_ERRO_BAIXAR);
        std::process::exit(1);
    }

    Command::new("chmod").arg("+x").arg("/root/install.sh").status().expect(ERR_TORNAR_EXECUTAVEL);

    log_message(MSG_VIRTUALMIN_EXECUTANDO);
    if Command::new("/root/install.sh").arg("-f").status().is_err() {
        log_message(MSG_VIRTUALMIN_ERRO_INSTALAR);
        std::process::exit(1);
    }

    log_message(MSG_VIRTUALMIN_INSTALADO_SUCESSO);
}
