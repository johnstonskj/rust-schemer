use rustyline::error::ReadlineError;
use rustyline::Editor;
use schemer_lang::error::{Error, ErrorKind};
use schemer_lang::eval::eval_datum;
use schemer_lang::types::{Ref, SchemeRepr};
use schemer_lang::{IMPLEMENTATION_NAME, IMPLEMENTATION_VERSION};
use schemer_library::{
    make_preset_environment, PresetEnvironmentKind, DEFAULT_SCHEME_ENVIRONMENT_VERSION,
};
use schemer_parse::parser::parse_datum_str;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use structopt::StructOpt;

fn main() {
    let command_args = parse_command_line();

    if isatty::stdin_isatty() {
        println!(
            "Welcome to {}, v{}.",
            IMPLEMENTATION_NAME, IMPLEMENTATION_VERSION
        );

        let history_file = command_args.history_file;

        // `()` can be used when no completer is required
        let mut rl = Editor::<()>::new();
        if rl.load_history(&history_file).is_err() {
            println!("No previous history.");
        }

        let mut env = make_preset_environment(match command_args.base_environment {
            BaseEnvironment::Interaction => PresetEnvironmentKind::Interaction,
            BaseEnvironment::R5Rs => {
                PresetEnvironmentKind::Report(DEFAULT_SCHEME_ENVIRONMENT_VERSION)
            }
            BaseEnvironment::SchemeBase => PresetEnvironmentKind::SchemeBase,
            BaseEnvironment::Null => {
                PresetEnvironmentKind::Null(DEFAULT_SCHEME_ENVIRONMENT_VERSION)
            }
        })
        .unwrap();

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

#[derive(Copy, Clone, Debug)]
pub enum BaseEnvironment {
    Interaction,
    R5Rs,
    SchemeBase,
    Null,
}

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

    /// The base environment to load
    #[structopt(long, short, default_value = "interaction")]
    base_environment: BaseEnvironment,
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

fn parse_command_line() -> CommandLine {
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

    args
}

impl Display for BaseEnvironment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Interaction => "interaction",
                Self::R5Rs => "r5rs",
                Self::SchemeBase => "scheme-base",
                Self::Null => "null",
            }
        )
    }
}

impl FromStr for BaseEnvironment {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "interaction" => Ok(Self::Interaction),
            "r5rs" => Ok(Self::R5Rs),
            "scheme-base" => Ok(Self::SchemeBase),
            "null" => Ok(Self::Null),
            _ => Err(Error::from(ErrorKind::BadArguments)),
        }
    }
}
