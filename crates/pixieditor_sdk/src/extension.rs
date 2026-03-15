pub trait Extension {
    fn load() {}
    fn initialize() {}
    fn user_ready() {}
    fn main_window_loaded() {}
    fn user_logged_in() {}
    fn user_logged_out() {}
    fn command_invoked(name: &str) {}
    fn command_invoked_str(name: &str, param: &str) {}
}