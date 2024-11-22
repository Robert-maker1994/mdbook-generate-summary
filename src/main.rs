use clap::{Arg, ArgMatches, Command};
use mdbook::errors::Error;
use mdbook::errors::Result;
use mdbook::preprocess::Preprocessor;
use mdbook::preprocess::CmdPreprocessor;
use mdbook_generate_summary::{
    markdown_summary_generator::MarkdownSummaryGenerator,
    summary_generator::SummaryGenerator};
use std::io;
use std::process;

pub fn make_app() -> Command {
    Command::new("summary-generator")
        .about("A mdbook preprocessor that adds support for generate summary pages")
        .subcommand(
            Command::new("supports")
                .arg(Arg::new("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        )
}

fn main() {
    let sg = MarkdownSummaryGenerator::new();
    sg.build();
    
    let pre = SummaryGenerator;

    let matches: ArgMatches = make_app().get_matches();

    if let Some(sub_args) = matches.subcommand_matches("supports") {
        handle_supports(sub_args, &pre);
    } else if let Err(e) = handle_preprocessing(&pre) {
        println!("errrr {}", e);
        process::exit(1);
    }
}

fn handle_preprocessing(pre: &dyn Preprocessor) -> Result<(), Error> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;
    
    let processed_book = pre.run(&ctx, book)?;

    serde_json::to_writer(io::stdout(), &processed_book)?;
    Ok(())
}

fn handle_supports(sub_args: &ArgMatches, pre: &dyn Preprocessor) -> ! {

    let renderer = sub_args
        .get_one::<String>("renderer")
        .expect("Required argument");

    let supported = pre.supports_renderer(renderer);

    if supported {
        process::exit(0);
    } else {
        process::exit(1);
    }
}

