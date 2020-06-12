use better_dwm_statusbar::prelude::*;
use std::include_str;
use std::time::Duration;

pub fn get_config() -> Vec<ModuleWrapper> {
    vec![
        ModuleWrapper(
            Box::new(CustomScript::new(
                " {VALUE}",
                include_str!("/home/martin/scripts/get_song.sh"),
            )),
            Duration::from_secs(2),
        ),
        ModuleWrapper(Box::new(RAMUsage::new("{VALUE} ")), Duration::from_secs(2)),
        ModuleWrapper(
            Box::new(CustomScript::new(
                "{VALUE} ",
                include_str!("/home/martin/dwm_config/get_volume.sh"),
            )),
            Duration::from_millis(200),
        ),
        ModuleWrapper(
            Box::new(Battery::new(["", "", "", "", ""], "{VALUE}")),
            Duration::from_secs(1),
        ),
        ModuleWrapper(Box::new(Time::new("{VALUE} ")), Duration::from_secs(1)),
    ]
}
