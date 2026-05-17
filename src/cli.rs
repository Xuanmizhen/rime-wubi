use clap::Parser;

#[derive(Parser)]
#[command(name = "rime-wubi")]
#[command(about = "Wubi Input Method Dictionary Management Tool", long_about = None)]
pub enum Command {
    #[command(about = "Generate Rime dictionary config file")]
    Generate,

    #[command(about = "Look up Wubi code for a phrase")]
    Lookup {
        #[arg(help = "Phrase to look up")]
        phrase: String,
    },

    #[command(about = "Add phrase to custom.txt")]
    Add {
        #[arg(help = "Phrase to add")]
        phrase: String,
    },
}
