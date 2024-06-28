use crate::utils::{log_message, check_root, check_dependencies};

pub const HOSTS_PATH: &str = "/etc/hosts";
pub const HOSTNAME_PATH: &str = "/etc/hostname";
pub const DEPENDENCIES: &[&str] = &["systemctl", "wget", "grep", "hostname", "echo"];
pub const MSG_CONFIGURANDO_HOSTNAME: &str = "Configurando hostname para: ";
pub const MSG_HOSTNAME_CONFIGURADO_SUCESSO: &str = "Hostname configurado com sucesso.";
pub const MSG_HOSTNAME_JA_CONFIGURADO: &str = "Hostname já está configurado.";
pub const ERR_ATUAL_HOSTNAME: &str = "Falha ao obter o hostname atual.";
pub const ERR_ESCREVER_HOSTNAME: &str = "Não foi possível escrever no arquivo de hostname.";
pub const ERR_ABRIR_HOSTS: &str = "Não foi possível abrir o arquivo de hosts.";
pub const ERR_ESCREVER_HOSTS: &str = "Não foi possível escrever no arquivo de hosts.";
pub const ERR_EXECUTAR_HOSTNAME: &str = "Falha ao executar o comando hostname.";

pub fn configure_initial_setup(hostname: &str) {
    check_root();
    check_dependencies(DEPENDENCIES);
    configure_hostname(hostname);
}

fn configure_hostname(hostname: &str) {
    use std::fs::OpenOptions;
    use std::io::Write;
    use std::process::Command;

    let current_hostname = Command::new("hostname").output().expect(ERR_ATUAL_HOSTNAME);
    let current_hostname = String::from_utf8(current_hostname.stdout).expect(ERR_ATUAL_HOSTNAME);

    if !current_hostname.contains(hostname) {
        log_message(&format!("{}{}", MSG_CONFIGURANDO_HOSTNAME, hostname));

        std::fs::write(HOSTNAME_PATH, hostname).expect(ERR_ESCREVER_HOSTNAME);

        let mut hosts_file = OpenOptions::new()
            .append(true)
            .open(HOSTS_PATH)
            .expect(ERR_ABRIR_HOSTS);
        writeln!(hosts_file, "127.0.0.1 {} {}", hostname, current_hostname.trim()).expect(ERR_ESCREVER_HOSTS);

        Command::new("hostname")
            .arg(hostname)
            .status()
            .expect(ERR_EXECUTAR_HOSTNAME);

        log_message(MSG_HOSTNAME_CONFIGURADO_SUCESSO);
    } else {
        log_message(MSG_HOSTNAME_JA_CONFIGURADO);
    }
}
