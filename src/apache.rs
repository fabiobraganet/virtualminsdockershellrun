use crate::utils::log_message;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;

pub const MSG_APACHE_LINHA_ADICIONADA: &str = "Linha adicionada ao arquivo de configuração do Apache e serviço reiniciado.";
pub const MSG_APACHE_FALHA_REINICIAR: &str = "Falha ao reiniciar o Apache. Configuração revertida.";
pub const MSG_APACHE_LINHA_PRESENTE: &str = "A linha de configuração já está presente no arquivo do Apache.";
pub const ERR_LER_ARQUIVO: &str = "Não foi possível ler o arquivo.";
pub const ERR_BACKUP_ARQUIVO: &str = "Não foi possível fazer backup do arquivo.";
pub const ERR_ABRIR_ARQUIVO: &str = "Não foi possível abrir o arquivo.";
pub const ERR_ESCREVER_ARQUIVO: &str = "Não foi possível escrever no arquivo.";
pub const ERR_RESTAURAR_BACKUP: &str = "Não foi possível restaurar o backup do arquivo.";

pub fn monitor_apache_config(hostname: &str, apache_config_file: &str) {
    add_config_line_to_apache(hostname, apache_config_file);
}

fn add_config_line_to_apache(hostname: &str, apache_config_file: &str) {
    let config_line = format!("ServerName {}", hostname);
    
    if !std::path::Path::new(apache_config_file).exists() {
        log_message(&format!("Arquivo de configuração do Apache não encontrado: {}. Verifique se o caminho está correto e o arquivo existe.", apache_config_file));
        return;
    }

    let config_content = std::fs::read_to_string(apache_config_file).expect(ERR_LER_ARQUIVO);
    
    if !config_content.contains(&config_line) {
        std::fs::copy(apache_config_file, format!("{}.bak", apache_config_file)).expect(ERR_BACKUP_ARQUIVO);

        let mut config_file = OpenOptions::new()
            .append(true)
            .open(apache_config_file)
            .expect(ERR_ABRIR_ARQUIVO);

        writeln!(config_file, "{}", config_line).expect(ERR_ESCREVER_ARQUIVO);

        if Command::new("systemctl").arg("restart").arg("apache2").status().is_ok() {
            log_message(MSG_APACHE_LINHA_ADICIONADA);
        } else {
            std::fs::copy(format!("{}.bak", apache_config_file), apache_config_file).expect(ERR_RESTAURAR_BACKUP);
            log_message(MSG_APACHE_FALHA_REINICIAR);
        }
    } else {
        log_message(MSG_APACHE_LINHA_PRESENTE);
    }
}
