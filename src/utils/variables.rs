use std::env;

lazy_static! {
    // Application settings
    pub static ref HOST: String = env::var("JUNE_HOST").expect("Missing `JUNE_HOST` environment variable");
}
