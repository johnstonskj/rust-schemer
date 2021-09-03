#[macro_use]
extern crate log;

use rustyline::completion::{Completer, Pair};
use rustyline::config::OutputStreamType;
use rustyline::error::ReadlineError;
use rustyline::highlight::{Highlighter, MatchingBracketHighlighter};
use rustyline::hint::{Hinter, HistoryHinter};
use rustyline::line_buffer::LineBuffer;
use rustyline::validate::{self, MatchingBracketValidator, Validator};
use rustyline::{Cmd, ColorMode, CompletionType, Config, Context, EditMode, Editor, KeyEvent};
use rustyline_derive::Helper;
use schemer_lang::error::{Error, ErrorKind};
use schemer_lang::eval::{eval_datum, Environment, Expression};
use schemer_lang::read::syntax_str::{
    SYNTAX_LEFT_PARENTHESIS_CHAR, SYNTAX_RIGHT_PARENTHESIS_CHAR, SYNTAX_SPACE_CHAR,
};
use schemer_lang::types::{Identifier, MutableRef, Ref, SchemeRepr, SchemeString};
use schemer_lang::{IMPLEMENTATION_NAME, IMPLEMENTATION_VERSION};
use schemer_library::{
    make_preset_environment, PresetEnvironmentKind, DEFAULT_SCHEME_ENVIRONMENT_VERSION,
};
use schemer_parse::parser::parse_datum_str;
use search_path::SearchPath;
use std::borrow::Cow;
use std::borrow::Cow::{Borrowed, Owned};
use std::env;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;

// ------------------------------------------------------------------------------------------------
// Main
// ------------------------------------------------------------------------------------------------

fn main() {
    let command_args = parse_command_line();

    if isatty::stdin_isatty() {
        println!(
            "Welcome to {}, v{}.",
            IMPLEMENTATION_NAME, IMPLEMENTATION_VERSION
        );

        let base_env = make_preset_environment(match command_args.base_environment {
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
        let mut env = Environment::new_child_named(base_env, "*repl*");

        let history_file = command_args.history_file;

        let config = Config::builder()
            .history_ignore_space(true)
            .color_mode(ColorMode::Enabled)
            .completion_type(CompletionType::List)
            .edit_mode(EditMode::Emacs)
            .output_stream(OutputStreamType::Stdout)
            .build();
        let h = ReplHelper {
            env: env.clone(),
            highlighter: MatchingBracketHighlighter::new(),
            hinter: HistoryHinter {},
            colored_prompt: "".to_owned(),
            validator: MatchingBracketValidator::new(),
        };
        let mut rl = Editor::with_config(config);
        rl.set_helper(Some(h));
        rl.bind_sequence(KeyEvent::alt('n'), Cmd::HistorySearchForward);
        rl.bind_sequence(KeyEvent::alt('p'), Cmd::HistorySearchBackward);

        if rl.load_history(&history_file).is_err() {
            println!("No previous history.");
        }

        info!("'(schemer-repl-history-file . {:?})", history_file);
        let _ = env.borrow_mut().insert(
            Identifier::from_str_unchecked("schemer-repl-history-file"),
            Expression::String(SchemeString::from(history_file.clone())),
        );
        init_file_path().map(|p| {
            let _ = env.borrow_mut().insert(
                Identifier::from_str_unchecked("schemer-repl-init-file"),
                Expression::String(SchemeString::from(p.to_string_lossy().to_string())),
            );
            info!("(load '(schemer-repl-init-file . {:?}))", p);
        });

        let default_prompt = "> ";
        loop {
            rl.helper_mut().expect("No helper").colored_prompt =
                format!("\x1b[1;32m{}\x1b[0m", default_prompt);
            let result = rl.readline(&default_prompt);

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
        rl.append_history(&history_file).unwrap();
    }
}

// ------------------------------------------------------------------------------------------------
// Read-line Support
// ------------------------------------------------------------------------------------------------

#[derive(Helper)]
struct ReplHelper {
    env: MutableRef<Environment>,
    highlighter: MatchingBracketHighlighter,
    validator: MatchingBracketValidator,
    hinter: HistoryHinter,
    colored_prompt: String,
}

impl Completer for ReplHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>), ReadlineError> {
        let rpos = line[0..pos]
            .chars()
            .rev()
            .position(|c| {
                [
                    SYNTAX_RIGHT_PARENTHESIS_CHAR,
                    SYNTAX_LEFT_PARENTHESIS_CHAR,
                    SYNTAX_SPACE_CHAR,
                ]
                .contains(&c)
            })
            .unwrap_or(pos);

        let prefix = &line[(pos - rpos)..];

        let candidates: Vec<Pair> = self
            .env
            .borrow()
            .completions(prefix)
            .into_iter()
            .filter_map(|(id, expr)| {
                if id.starts_with(prefix) {
                    Some(Pair {
                        display: expr,
                        replacement: id,
                    })
                } else {
                    None
                }
            })
            .collect();

        Ok(((pos - rpos), candidates))
    }

    fn update(&self, line: &mut LineBuffer, start: usize, elected: &str) {
        let end = line.pos();
        line.replace(start..end, elected)
    }
}

impl Hinter for ReplHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<String> {
        self.hinter.hint(line, pos, ctx)
    }
}

impl Highlighter for ReplHelper {
    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }

    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> Cow<'b, str> {
        if default {
            Borrowed(&self.colored_prompt)
        } else {
            Borrowed(prompt)
        }
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned("\x1b[1m".to_owned() + hint + "\x1b[m")
    }

    fn highlight_char(&self, line: &str, pos: usize) -> bool {
        self.highlighter.highlight_char(line, pos)
    }
}

impl Validator for ReplHelper {
    fn validate(
        &self,
        ctx: &mut validate::ValidationContext,
    ) -> rustyline::Result<validate::ValidationResult> {
        self.validator.validate(ctx)
    }

    fn validate_while_typing(&self) -> bool {
        self.validator.validate_while_typing()
    }
}

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

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn init_file_path() -> Option<PathBuf> {
    let mut search_path = SearchPath::default();
    let _ = xdirs::config_dir_for(IMPLEMENTATION_NAME).map(|p| search_path.append(p));
    let _ = env::var("HOME").map(|p| search_path.append(PathBuf::from(p)));
    search_path.append(PathBuf::from("."));
    search_path.find("repl-init.sr".as_ref())
}
