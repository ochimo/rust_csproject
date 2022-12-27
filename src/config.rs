
pub struct Config {
    pub file_path: String,
    pub hash_size: usize
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get file path"),
        };

        let hash_size: usize = match args.next() {
            Some(arg) => arg.parse().unwrap(),
            None => return Err("Did not get hash size"),
        };

        Ok(Config {
            file_path,
            hash_size
        })
    }
}