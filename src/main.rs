use fourd_language_oop::{cli, server};
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let Some(subcommand) = args.first() else {
        print_usage();
        return ExitCode::from(2);
    };

    let rest = &args[1..];
    let code: u8 = match subcommand.as_str() {
        "query" => dispatch(cli::parse_query_args(rest), cli::run_query),
        "class" => dispatch(cli::parse_name_args(rest, false, false), cli::run_class),
        "member" => dispatch(cli::parse_name_args(rest, false, false), cli::run_member),
        "members" => dispatch(cli::parse_name_args(rest, true, true), cli::run_members),
        "returns" => dispatch(cli::parse_name_args(rest, false, false), cli::run_returns),
        "serve" => dispatch(server::parse_args(rest), server::run),
        "-h" | "--help" | "help" => {
            print_usage();
            0
        }
        other => {
            eprintln!("error: unknown subcommand: {other}");
            print_usage();
            2
        }
    };

    ExitCode::from(code)
}

fn dispatch<T>(parsed: Result<T, String>, run: impl FnOnce(T) -> i32) -> u8 {
    match parsed {
        Ok(args) => run(args) as u8,
        Err(e) => {
            eprintln!("error: {e}");
            print_usage();
            2
        }
    }
}

fn print_usage() {
    eprintln!(
        r#"4d-language-oop: deterministic natural-language lookup for the 4D object (OOP) language class reference.

USAGE:
    4d-language-oop query "<natural language query>" [--limit N] [--json]
    4d-language-oop class <class>                    [--json]
    4d-language-oop member <Class.member>            [--json]
    4d-language-oop members --class <class> [--kind function|property|constructor] [--json]
    4d-language-oop returns <class>                  [--json]
    4d-language-oop serve [--port 8080]

EXAMPLES:
    4d-language-oop query "read a text file line by line"
    4d-language-oop query "send an email with an attachment" --limit 3 --json
    4d-language-oop class 4D.File          # class card: members + how to obtain an instance
    4d-language-oop member Collection.orderBy
    4d-language-oop member File.exists     # inherited members resolve to their declaring class
    4d-language-oop members --class Entity --kind function
    4d-language-oop returns 4D.FileHandle  # what produces an instance of this class
    4d-language-oop serve --port 8080
    curl "http://localhost:8080/lookup?q=order+a+collection&limit=3""#
    );
}
