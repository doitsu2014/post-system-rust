use env_logger::{Builder, Env};

pub fn init_logger() {
    let env = Env::default().filter("LOG_LEVEL").write_style("LOG_STYLE");
    let mut builder = Builder::from_env(env);
    builder.init();
}
