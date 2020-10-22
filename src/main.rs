use clap::Clap;

#[derive(Clap)]
#[clap(name = "mybin", version = clap::crate_version!())]
enum Opts {}

fn main() {
    Opts::parse();
}
