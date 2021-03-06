mod args;
mod error;

use args::Args;

use pete_core::engine::{Engine};

fn main() -> Result<(), error::Error> {
    let engine = Engine::new();
    let args = Args::create_from_cli()?;
    match engine.render(args.template, args.params) {
        Ok(output_string) => {
            print!("{}", output_string);
        },
        Err(error) => {
            println!("Failed to render the template:\n{}", error);
        }
    }

    Ok(())
}
