mod utils;
mod setup;
mod virtualmin;
mod apache;
mod services;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(long)]
    hostname: String,

    #[structopt(long, default_value = "/etc/apache2/apache2.conf")]
    apache_config_file: String,
}

fn main() {
    let args = Cli::from_args();

    setup::configure_initial_setup(&args.hostname);
    services::check_and_restart_services();
    apache::monitor_apache_config(&args.hostname, &args.apache_config_file);
    virtualmin::install_or_start_virtualmin();
}
