pub const GITHUB_API_LATEST_RELEASE: &str = "https://api.github.com/repos/skyline69/HaxRS/releases/latest";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
pub const INPUT_PROMPT: &str = "Selection: ";