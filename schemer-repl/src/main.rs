use rustyline::error::ReadlineError;
use rustyline::Editor;
use schemer_lang::parameters::global_flags;
use schemer_lang::read::datum::Datum;
use schemer_lang::types::lists::list;
use schemer_lang::types::{Boolean, Integer, Pair, SchemeRepr, SchemeString, Symbol};
use schemer_lang::{IMPLEMENTATION_NAME, IMPLEMENTATION_VERSION};
use schemer_library::scheme::base::features;
use schemer_library::scheme::eval::eval_datum;
use schemer_library::scheme::repl::interaction_environment;
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

        let env = interaction_environment(5.into()).unwrap();

        loop {
            let result = rl.readline("> ");
            match result {
                Ok(line) => {
                    if !line.trim().is_empty() {
                        rl.add_history_entry(line.as_str());
                        let result = parse_datum_str(&line);
                        match result {
                            Ok(datum) => {
                                let result = eval_datum(datum, &env).unwrap();
                                println!("{}", result.to_repr_string());
                            }
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

fn show_settings(args: &CommandLine) -> bool {
    println!();
    println!("> (features)\n{}\n", features().to_repr_string());
    println!("> (global-flags)\n{}\n", global_flags().to_repr_string());
    println!("> (repl-flags)\n{}\n", repl_flags(args).to_repr_string());
    false
}

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

    #[structopt(subcommand)]
    cmd: Option<SubCommand>,
}

fn repl_flags(args: &CommandLine) -> Box<Pair> {
    list(
        vec![
            ("verbose", Datum::from(Integer::from(args.verbose))),
            ("use-color", Datum::from(Boolean::from(args.use_color))),
            (
                "history-file",
                Datum::from(SchemeString::from(args.history_file.to_string())),
            ),
        ]
        .into_iter()
        .map(|(k, v)| {
            Datum::List(Box::new(Pair::cons(
                Datum::Symbol(Symbol::from_str_unchecked(k)),
                v,
            )))
        })
        .collect(),
    )
}

#[derive(Debug, StructOpt)]
enum SubCommand {
    /// Display current settings and exit
    Settings,
}

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
    if match args.cmd {
        Some(SubCommand::Settings) => show_settings(&args),
        _ => true,
    } {
        args.history_file
    } else {
        std::process::exit(0)
    }
}
