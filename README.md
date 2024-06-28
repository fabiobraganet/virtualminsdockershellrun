# Virtualmin Docker Shell Run

[![Build Status](https://github.com/fabiobraganet/virtualminsdockershellrun/actions/workflows/rust.yml/badge.svg)](https://github.com/fabiobraganet/virtualminsdockershellrun/actions)

## Índice

- [Sobre o Projeto](#sobre-o-projeto)
- [Instalação](#instalação)
- [Uso](#uso)
- [Estrutura do Projeto](#estrutura-do-projeto)
- [Contribuição](#contribuição)
- [Licença](#licença)
- [Contato](#contato)

## Sobre o Projeto

O **Virtualmin Docker Shell Run** é uma ferramenta desenvolvida em Rust para facilitar a administração de servidores Virtualmin em contêineres Docker. Ele automatiza a verificação e reinicialização de serviços críticos, configuração de hostname, monitoramento e atualização de configurações do Apache, bem como a instalação e inicialização do Virtualmin.

## Instalação

Para instalar o projeto, siga os passos abaixo:

1. Clone o repositório:
   ```sh
   git clone https://github.com/username/virtualminsdockershellrun.git
   cd virtualminsdockershellrun
   ```

2. Compile o projeto:
   ```sh
   cargo build --release
   ```

3. O executável será gerado na pasta `target/release/`.

## Uso

Para utilizar o **Virtualmin Docker Shell Run**, execute o binário gerado com os parâmetros necessários:

```sh
sudo ./target/release/virtualminsdockershellrun --hostname <seu-hostname>
```

### Parâmetros

- `--hostname`: Define o hostname do servidor.
- `--apache_config_file` (opcional): Caminho para o arquivo de configuração do Apache. O valor padrão é `/etc/apache2/apache2.conf`.

## Estrutura do Projeto

A estrutura do projeto é organizada da seguinte forma:

```
virtualminsdockershellrun/
├── Cargo.toml                # Arquivo de configuração do cargo
└── src
    ├── main.rs               # Arquivo principal do projeto
    ├── lib.rs                # Módulo principal que exporta todos os módulos
    ├── utils.rs              # Utilitários comuns ao projeto
    ├── setup.rs              # Configuração inicial do servidor
    ├── virtualmin.rs         # Instalação e inicialização do Virtualmin
    ├── apache.rs             # Monitoramento e atualização das configurações do Apache
    └── services.rs           # Verificação e reinicialização de serviços críticos
```

## Contribuição

Contribuições são bem-vindas! Sinta-se à vontade para abrir issues e pull requests.

### Como Contribuir

1. Faça um fork do projeto.
2. Crie uma branch para sua feature (`git checkout -b feature/nova-feature`).
3. Commit suas alterações (`git commit -m 'Adiciona nova feature'`).
4. Faça push para a branch (`git push origin feature/nova-feature`).
5. Abra um pull request.

## Licença

Sem Licença `FREE LICENSE`

## Contato

Fabio Braga 
Link do Projeto: [https://github.com/fabiobraganet/virtualminsdockershellrun](https://github.com/fabiobraganet/virtualminsdockershellrun)

