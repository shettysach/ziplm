#[cfg(feature = "cli")]
mod cli {
    use clap::Parser;
    use std::fs::read_to_string;
    use std::path::PathBuf;
    use ziplm::ZipModel;

    #[derive(Parser, Debug)]
    #[command(version, about, long_about = None)]
    struct Args {
        /// Maximum length of sample generated
        #[arg(short, long, default_value_t = 100)]
        length: u16,

        /// Prefix for sample generated
        #[arg(short, long, default_value_t = String::new())]
        prefix: String,

        /// Temperature for sample generated
        #[arg(short, long, default_value_t = 0.25)]
        temp: f64,

        /// Path of vocabulary data (.txt file)
        /// [Defaults to qwertyuiopasdfghjklzxcvbnm,. '"]
        #[arg(short, long)]
        vocab: Option<PathBuf>,

        /// Path of training data (.txt file)
        /// [Defaults to Mary Shelley's Frankenstein]
        #[arg(short, long)]
        data: Option<PathBuf>,

        /// Conversion factor
        #[arg(short, long, default_value_t = 5.545177444479562)]
        conv: f64,
    }

    fn validate(path: &Option<PathBuf>) -> Option<String> {
        match path {
            Some(path) => match read_to_string(path) {
                Ok(mut content) => {
                    content.retain(|c| c != '\n');
                    Some(content)
                }
                Err(_) => None,
            },
            None => None,
        }
    }

    pub fn run() {
        let args: Args = Args::parse();

        let vocab = match validate(&args.vocab) {
            Some(vocab) => vocab,
            None => r#"qwertyuiopasdfghjklzxcvbnm,. '""#.to_string(),
        };
        let data = match validate(&args.data) {
            Some(data) => data,
            None => include_str!("../data.txt").to_string(),
        };

        let model = ZipModel::new(&vocab, &data, args.conv);
        let _ = model.sample_sequence(args.length, &args.prefix, args.temp);
    }
}

fn main() {
    #[cfg(feature = "cli")]
    cli::run();
}
