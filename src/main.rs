mod config;
use better_dwm_statusbar::start_bar;

start_bar!(config::get_config());
