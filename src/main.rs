use kintaro::{application::run, error::KintaroError, Config};

fn main() -> Result<(), KintaroError> {
    let filename = "kintaro.socool";
    let config = Config::default();
    run(filename, config)
}
