use worker::Env;

// Get environment variable (Send-safe)
#[worker::send]
pub async fn get_var(env: &Env, name: &str) -> Option<String> {
    env.var(name).ok().map(|v| v.to_string())
}
