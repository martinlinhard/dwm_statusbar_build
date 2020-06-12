use better_dwm_statusbar::prelude::*;

pub fn get_config() -> Vec<ModuleWrapper> {
    vec![
        ModuleWrapper(
            Box::new(CustomScript::new(
                " {VALUE}",
                "/home/martin/scripts/get_song.sh",
            )),
            PauseBetweenYields::from_secs(2),
        ),
        ModuleWrapper(
            Box::new(RAMUsage::new("{VALUE} ")),
            PauseBetweenYields::from_secs(2),
        ),
        ModuleWrapper(
            Box::new(CustomScript::new(
                "{VALUE} ",
                "/home/martin/dwm_config/get_volume.sh",
            )),
            PauseBetweenYields::from_millis(200),
        ),
        ModuleWrapper(
            Box::new(Battery::new(["", "", "", "", ""], "{VALUE}")),
            PauseBetweenYields::from_secs(1),
        ),
        ModuleWrapper(
            Box::new(Time::new("{VALUE} ")),
            PauseBetweenYields::from_secs(1),
        ),
    ]
}
