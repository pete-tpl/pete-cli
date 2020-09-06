mod args;
mod error;

use args::Args;

use pete_core::engine::Engine;

fn main() -> Result<(), error::Error> {
    let engine = Engine::new();
    let args = Args::create_from_cli()?;
    match engine.render(args.template, args.params) {
        Ok(output_string) => {
            println!("{}", output_string);
        },
        Err(error) => {
            println!("{}", error);
        }
    }

    Ok(())
}
