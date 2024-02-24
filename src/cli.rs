use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(
        long,
        short = 'm',
        help = "Choose AI model: tiny, small, medium",
        default_value = "tiny"
    )]
    model: String,
}
// able to choose modele to use, default to tiny
pub fn get_ai_model() -> String {
    let args = Args::parse();
    //
    // check the ai model. If not in list, default to tiny
    //
    let ai_model = match args.model.as_str() {
        "small" => "mistral-small",
        "medium" => "mistral-medium",
        _ => "mistral-tiny",
    };
    //

    ai_model.to_string()
}
