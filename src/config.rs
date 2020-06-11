use better_dwm_statusbar::bar_modules::*;
use better_dwm_statusbar::module::ModuleWrapper;
use std::time::Duration;

pub fn get_config() -> Vec<ModuleWrapper> {
    vec![ModuleWrapper(
        Box::new(Time::new("{VALUE}")),
        Duration::from_secs(1),
    )]
}
