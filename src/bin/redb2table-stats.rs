use std::io;
use std::process::ExitCode;

use rs_redb2table_stats::path2db2stats2sink_default;

fn io_envkey2val(key: &'static str) -> impl Fn() -> String {
    move || std::env::var(key).unwrap_or_default()
}

fn io_path2db() -> impl Fn() -> String {
    io_envkey2val("ENV_INPUT_DB_PATH")
}

fn io_main() -> impl FnOnce() -> Result<(), io::Error> {
    || {
        let pat: String = io_path2db()();
        path2db2stats2sink_default(pat)
    }
}

fn sub() -> Result<(), io::Error> {
    io_main()()
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        let p: String = io_path2db()();
        eprintln!("ENV_INPUT_DB_PATH: {p}");
        ExitCode::FAILURE
    })
}
