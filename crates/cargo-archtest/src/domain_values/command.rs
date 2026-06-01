#[derive(Debug, StructOpt)]
pub enum Command {
    Archtest {
        #[structopt(
            short,
            long,
            about = "Compares layers found with provided",
            help = "Compares layers found with provided"
        )]
        check_for_complete_layer_specification: bool,
        #[structopt(
            short,
            long,
            default_value = "Cargo.toml",
            about = "Path to Cargo.toml",
            help = "Path to Cargo.toml"
        )]
        toml_path: String,
        #[structopt(
            short = "x",
            long = "exclude-crate",
            about = "Exclude a crate from architecture check (can be repeated)",
            help = "Exclude a crate from architecture check (can be repeated)"
        )]
        exclude_crates: Vec<String>,
    },
}
