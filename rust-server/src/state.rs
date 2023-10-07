#[derive(Clone)]
pub struct AppState {
    pub config: config::Config,
    pub log: slog::Logger,
    pub db: sqlx::MySqlPool,
}
