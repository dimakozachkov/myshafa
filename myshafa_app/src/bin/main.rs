use clap::{Arg, Command};
use myshafa_app_lib::filters::auth_filters;
use myshafa_core::configs::AppConfig;
use warp::Filter;

const DEFAULT_CONFIG_FILENAME: &str = "./config.yml";

fn cli() -> Command {
    let config_arg = Arg::new("config")
        .long("config")
        .short('c')
        .help("Path to a config");

    Command::new("myshafa")
        .about("The myshafa is a stupid project")
        .version("1.0.0")
        .next_line_help(true)
        .arg(config_arg)
}

#[tokio::main]
async fn main() {
    let args = cli().get_matches();

    let arg_path_to_config = args.get_one::<String>("config");
    let path_to_config: &str;
    if let Some(p) = arg_path_to_config {
        path_to_config = p;
    } else {
        path_to_config = DEFAULT_CONFIG_FILENAME;
    }

    let app_config = AppConfig::new(path_to_config).unwrap();

    let hello = warp::path::end().map(|| format!("Hello, World!!!"));

    let api = warp::path("api").map(auth_filters::auth);

    let routes = warp::any().and(api);

    warp::serve(routes)
        .run((app_config.server.host, app_config.server.port))
        .await;
}
