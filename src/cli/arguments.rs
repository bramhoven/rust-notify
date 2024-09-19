use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct CommandLineArguments {
    /// host to listen on
    #[arg(short='u', long="host", default_value="0.0.0.0")]
    pub host: String,

    /// port to listen on
    #[arg(short='p', long="port", default_value_t=3000)]
    pub port: i16,
}
