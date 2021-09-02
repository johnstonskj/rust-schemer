use rustyline::error::ReadlineError;
use rustyline::Editor;
use schemer_lang::eval::eval_datum;
use schemer_lang::types::{Ref, SchemeRepr};
use schemer_lang::{IMPLEMENTATION_NAME, IMPLEMENTATION_VERSION};
use schemer_library::{make_preset_environment, PresetEnvironmentKind};
use schemer_parse::parser::parse_datum_str;
use structopt::StructOpt;

fn main() {
    if isatty::stdin_isatty() {
        println!(
            "Welcome to {}, v{}.",
            IMPLEMENTATION_NAME, IMPLEMENTATION_VERSION
        );

        let history_file = parse_command_line();

        // `()` can be used when no completer is required
        let mut rl = Editor::<()>::new();
        if rl.load_history(&history_file).is_err() {
            println!("No previous history.");
        }

        let mut env = make_preset_environment(PresetEnvironmentKind::Interaction).unwrap();

        loop {
            let result = rl.readline("> ");
            match result {
                Ok(line) => {
                    if !line.trim().is_empty() {
                        rl.add_history_entry(line.as_str());
                        let result = parse_datum_str(&line);
                        match result {
                            Ok(datum) => match eval_datum(Ref::new(datum), &mut env) {
                                Ok(result) => {
                                    println!("{}", result.to_repr_string());
                                }
                                Err(err) => {
                                    println!("{}", err);
                                }
                            },
                            Err(e) => {
                                eprintln!("{}", e);
                            }
                        }
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    println!("Interrupted");
                    break;
                }
                Err(ReadlineError::Eof) => {
                    println!("Goodbye");
                    break;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }
        rl.save_history(&history_file).unwrap();
    }
}

// ------------------------------------------------------------------------------------------------
// Commands
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Command-Line
// ------------------------------------------------------------------------------------------------

#[derive(Debug, StructOpt)]
#[structopt(name = IMPLEMENTATION_NAME, about = "Simple schemer repl.")]
struct CommandLine {
    /// The level of logging to perform; from off to trace
    #[structopt(long, short = "v", parse(from_occurrences))]
    verbose: i8,

    /// Colorize output where applicable
    #[structopt(long)]
    use_color: bool,

    /// The name of the file for command history
    #[structopt(long, default_value = "schemer-history.txt")]
    history_file: String,
}

// fn repl_flags(args: &CommandLine) -> Pair {
//     vector_to_list(
//         vec![
//             ("verbose", Datum::from(Integer::from(args.verbose))),
//             ("use-color", Datum::from(Boolean::from(args.use_color))),
//             (
//                 "history-file",
//                 Datum::from(SchemeString::from(args.history_file.to_string())),
//             ),
//         ]
//         .into_iter()
//         .map(|(k, v)| {
//             Datum::List(Pair::cons(
//                 Datum::Symbol(Identifier::from_str_unchecked(k)).into(),
//                 v.into(),
//             ))
//         })
//         .collect(),
//     )
// }

// TODO: find config path, load "init.sc"
// (
//     "config-dir",
//     Datum::String(SchemeString::new_unchecked(xdirs::config_dir_for(
//         IMPLEMENTATION_NAME,
//     ))),
// ),

// TODO: find library path
// (
//     "library-dir",
//     Datum::String(SchemeString::new_unchecked(xdirs::data_local_dir_for(
//         IMPLEMENTATION_NAME,
//     ))),
// ),

fn parse_command_line() -> String {
    let args = CommandLine::from_args();

    pretty_env_logger::formatted_builder()
        .filter_level(match args.verbose {
            0 => log::LevelFilter::Off,
            1 => log::LevelFilter::Error,
            2 => log::LevelFilter::Warn,
            3 => log::LevelFilter::Info,
            4 => log::LevelFilter::Debug,
            _ => log::LevelFilter::Trace,
        })
        .init();

    args.history_file
}
