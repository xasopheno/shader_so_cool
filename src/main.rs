use kintaro::{application::run, error::KintaroError, Config};

fn main() -> Result<(), KintaroError> {
    let config = Config::default();
    run(config)
}
