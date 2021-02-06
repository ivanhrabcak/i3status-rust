use std::collections::HashMap as Map;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref NONE: Map<String, String> = map_to_owned! {
        "" => "",
        "backlight_empty" => " BRIGHT ",
        "backlight_full" => " BRIGHT ",
        "backlight_partial1" => " BRIGHT ",
        "backlight_partial2" => " BRIGHT ",
        "backlight_partial3" => " BRIGHT ",
        "bat" => " BAT ",
        "bat_charging" => " CHG ",
        "bat_discharging" => " DCG ",
        "bat_empty" => " EMP ",
        "bat_full" => " FULL ",
        "bat_half" => " BAT ",
        "bat_not_available" => " BAT N/A ",
        "bat_quarter" => " BAT ",
        "bat_three_quarters" => " BAT ",
        "bell" => " ON ",
        "bell-slash" => " OFF ",
        "bluetooth" => " BT",
        "calendar" => " CAL ",
        "cogs" => " LOAD ",
        "cpu" => " CPU ",
        "disk_drive" => " DISK ",
        "docker" => " DOCKER ",
        "github" => " GITHUB ",
        "gpu" => " GPU ",
        "headphones" => " HEAD",
        "joystick" => " JOY",
        "keyboard" => " KBD",
        "mail" => " ",
        "memory_mem" => " MEM ",
        "memory_swap" => " SWAP ",
        "mouse" => " MOUSE",
        "music" => " ",
        "music_next" => " > ",
        "music_pause" => " || ",
        "music_play" => " > ",
        "music_prev" => " < ",
        "net_down" => " DOWN ",
        "net_loopback" => " LO ",
        "net_up" => " UP ",
        "net_vpn" => " VPN",
        "net_wired" => " ETH",
        "net_wireless" => " WLAN",
        "notification" => " NOTIF ",
        "phone" => " PHONE ",
        "phone_disconnected" => " PHONE ",
        "ping" => " PING ",
        "pomodoro" => " POMODORO ",
        "resolution" => " RES ",
        "tasks" => " TSK ",
        "thermometer" => " TEMP ",
        "time" => " ",
        "toggle_off" => " OFF ",
        "toggle_on" => " ON ",
        "update" => " UPD ",
        "uptime" => " UP ",
        "volume_empty" => " VOL ",
        "volume_full" => " VOL ",
        "volume_half" => " VOL ",
        "volume_muted" => " VOL MUTED ",
        "microphone_empty" => " MIC ",
        "microphone_full" => " MIC ",
        "microphone_half" => " MIC ",
        "microphone_muted" => " MIC MUTED ",
        "weather_clouds" => " CLOUDY ",
        "weather_default" => " WEATHER ",
        "weather_rain" => " RAIN ",
        "weather_snow" => " SNOW ",
        "weather_sun" => " SUNNY ",
        "weather_thunder" => " STORM ",
        "xrandr" => " SCREEN ",
        "shutdown" => " SHUTDOWN ",
        "restart" => " RESTART "
    };

    // FontAwesome 4
    pub static ref AWESOME: Map<String, String> = map_to_owned! {
        "" => "",
        "backlight_empty" => " \u{1f315} ",
        "backlight_full" => " \u{1f311} ",
        "backlight_partial1" => " \u{1f314} ",
        "backlight_partial2" => " \u{1f313} ",
        "backlight_partial3" => " \u{1f312} ",
        "bat_charging" => " \u{f1e6} ",
        "bat_discharging" => " \u{f242} ",
        "bat_empty" => " \u{f244} ",
        "bat_full" => " \u{f240} ",
        "bat_half" => " \u{f242} ",
        "bat_not_available" => " \u{f244} ",
        "bat_quarter" => " \u{f243} ",
        "bat_three_quarters" => " \u{f241} ",
        "bell" => " \u{f0f3} ",
        "bell-slash" => " \u{f1f7} ",
        "bluetooth" => " \u{f294}",
        "calendar" => " \u{f073} ",
        "cogs" => " \u{f085} ",
        "cpu" => " \u{f0e4} ",
        "disk_drive" => " \u{f0a0} ",
        "docker" => " \u{f21a} ",
        "github" => " \u{f09b} ",
        "gpu" => " \u{f26c} ",
        "headphones" => " \u{f025}",
        "joystick" => " \u{f11b}",
        "keyboard" => " \u{f11c}",
        "mail" => " \u{f0e0} ",
        "memory_mem" => " \u{f2db} ",
        "memory_swap" => " \u{f0a0} ",
        "mouse" => " \u{f245}",
        "music" => " \u{f001} ",
        "music_next" => " \u{f061} ",
        "music_pause" => " \u{f04c} ",
        "music_play" => " \u{f04b} ",
        "music_prev" => " \u{f060} ",
        "net_bridge" => " \u{f0e8} ",
        "net_down" => " \u{2b07} ",
        "net_loopback" => " LO ",
        "net_modem" => " \u{f095} ",
        "net_up" => " \u{2b06} ",
        "net_vpn" => " \u{f023} ",
        "net_wired" => " \u{f0ac} ",
        "net_wireless" => " \u{f1eb} ",
        "notification" => " \u{f0a2} ",
        "phone" => " \u{f10b} ",
        "phone_disconnected" => " \u{1f4f5} ",
        "ping" => " \u{21ba} ",
        "pomodoro" => " \u{1f345} ",
        "resolution" => " \u{f096} ", // fa-square-o
        "tasks" => " \u{f0ae} ",
        "thermometer" => " \u{f2c8} ",
        "time" => " \u{f017} ",
        "toggle_off" => " \u{f204} ",
        "toggle_on" => " \u{f205} ",
        "unknown" => " \u{f128} ",
        "update" => " \u{f062} ", // Same as time symbol.
        "uptime" => " \u{f017} ",
        "volume_empty" => " \u{f026} ",
        "volume_full" => " \u{f028} ",
        "volume_half" => " \u{f027} ",
        "volume_muted" => " \u{f026} \u{f00d} ",
        "microphone_empty" => " \u{f130} ",
        "microphone_full" => " \u{f130} ",
        "microphone_half" => " \u{f130} ",
        "microphone_muted" => " \u{f131} ",
        "weather_clouds" => " \u{f0c2} ",
        "weather_default" => " \u{f0c2} ", // Cloud symbol as default
        "weather_rain" => " \u{f043} ",
        "weather_snow" => " \u{f2dc} ",
        "weather_sun" => " \u{f185} ",
        "weather_thunder" => " \u{f0e7} ",
        "xrandr" => " \u{f26c} ",
        "shutdown" => " \u{f011} ",
        "restart" => " \u{f110} " 
    };

    // FontAwesome 5
    pub static ref AWESOME5: Map<String, String> = map_to_owned! {
        "" => "",
        "backlight_empty" => " \u{1f315} ",
        "backlight_full" => " \u{1f311} ",
        "backlight_partial1" => " \u{1f314} ",
        "backlight_partial2" => " \u{1f313} ",
        "backlight_partial3" => " \u{1f312} ",
        "bat_charging" => " \u{f1e6} ",
        "bat_discharging" => " \u{f242} ",
        "bat_empty" => " \u{f244} ",
        "bat_full" => " \u{f240} ",
        "bat_half" => " \u{f242} ",
        "bat_quarter" => " \u{f243} ",
        "bat_three_quarters" => " \u{f241} ",
        "bell" => " \u{f0f3} ",
        "bell-slash" => " \u{f1f6} ",
        "bluetooth" => " \u{f294}",
        "calendar" => " \u{f073} ",
        "cogs" => " \u{f085} ",
        "cpu" => " \u{f3fd} ",
        "disk_drive" => " \u{f8b5} ",
        "docker" => " \u{f21a} ",
        "github" => " \u{f09b} ",
        "gpu" => " \u{f26c} ",
        "headphones" => " \u{f025}",
        "joystick" => " \u{f11b}",
        "keyboard" => " \u{f11c}",
        "mail" => " \u{f0e0} ",
        "memory_mem" => " \u{f2db} ",
        "memory_swap" => " \u{f0a0} ",
        "mouse" => " \u{f245}",
        "music" => " \u{f001} ",
        "music_next" => " \u{f061} ",
        "music_pause" => " \u{f04c} ",
        "music_play" => " \u{f04b} ",
        "music_prev" => " \u{f060} ",
        "net_bridge" => " \u{f0e8} ",
        "net_down" => " \u{f019} ",
        "net_loopback" => " LO ",
        "net_modem" => " \u{f095} ",
        "net_up" => " \u{f093} ",
        "net_vpn" => " \u{f023} ",
        "net_wired" => " \u{f6ff} ",
        "net_wireless" => " \u{f1eb} ",
        "notification" => " \u{f0f3} ",
        "phone" => " \u{f3cd} ",
        "phone_disconnected" => " \u{1f4f5} ",
        "ping" => " \u{f362} ",
        "pomodoro" => " \u{1f345} ",
        "resolution" => " \u{f096} ", // fa-square-o
        "tasks" => " \u{f0ae} ",
        "thermometer" => " \u{f2c8} ",
        "time" => " \u{f017} ",
        "toggle_off" => " \u{f204} ",
        "toggle_on" => " \u{f205} ",
        "unknown" => " \u{f128} ",
        "update" => " \u{f062} ",
        "uptime" => " \u{f2f2} ",
        "volume_empty" => " \u{f026} ",
        "volume_full" => " \u{f028} ",
        "volume_half" => " \u{f027} ",
        "volume_muted" => " \u{f6a9} ",
        "microphone_full" => " \u{f3c9} ",
        "microphone_half" => " \u{f3c9} ",
        "microphone_empty" => " \u{f3c9} ",
        "microphone_muted" => " \u{f539} ",
        "weather_clouds" => " \u{f0c2} ",
        "weather_default" => " \u{f0c2} ", // Cloud symbol as default
        "weather_rain" => " \u{f043} ",
        "weather_snow" => " \u{f2dc} ",
        "weather_sun" => " \u{f185} ",
        "weather_thunder" => " \u{f0e7} ",
        "xrandr" => " \u{f26c} ",
        "shutdown" => " \u{f011} ",
        "restart" => " \u{f110} "
    };

    pub static ref MATERIAL: Map<String, String> = map_to_owned! {
        "" => "",
        "bat_charging" => " \u{e1a3} ",
        "bat_discharging" => " \u{e19c} ",
        "bat_empty" => " \u{e19c} ",
        "bat_full" => " \u{e1a4} ",
        "bat_half" => " \u{e1a5} ",
        "bat_quarter" => " \u{e1a5} ",
        "bat_three_quarters" => " \u{e1a5} ",
        "bell" => " \u{e7f4} ",
        "bell-slash" => " \u{e7f8} ",
        "bluetooth" => " \u{e1a7}",
        "calendar" => " \u{e935}",
        "cogs" => " \u{e8b8} ",
        "cpu" => " \u{e640} ",
        "disk_drive" => " \u{e1db} ",
        "docker" => " \u{e532} ",
        "github" => " \u{e86f} ",
        "gpu" => " \u{e333} ",
        "headphones" => " \u{e60f}",
        "joystick" => " \u{e30f}",
        "keyboard" => " \u{e312}",
        "mail" => " \u{e0be} ",
        "memory_mem" => " \u{e322} ",
        "memory_swap" => " \u{e8d4} ",
        "mouse" => " \u{e323}",
        "music" => " \u{e405} ",
        "music_next" => " \u{e044} ",
        "music_pause" => " \u{e034} ",
        "music_play" => " \u{e037} ",
        "music_prev" => " \u{e045} ",
        "net_loopback" => " LO ",
        "notification" => " \u{e7f7} ",
        "phone" => " \u{e324} ",
        "phone_disconnected" => " \u{1f4f5} ",
        "pomodoro" => " \u{1f345} ",
        "resolution" => " \u{f152} ", // crop-square-rounded
        "tasks" => " \u{e8f9} ",
        "thermometer" => " \u{f2c8} ", // TODO
        "time" => " \u{e192} ",
        "toggle_off" => " \u{e836} ",
        "toggle_on" => " \u{e837} ",
        "update" => " \u{e8d7} ",
        "uptime" => " \u{e192} ", // Same as time symbol.
        "volume_empty" => " \u{e04e} ",
        "volume_full" => " \u{e050} ",
        "volume_half" => " \u{e04d} ",
        "volume_muted" => " \u{e04e} \u{e04f} ",
        "xrandr" => " \u{e31e} ",
        "shutdown" => " \u{e8ac} ",
        "restart" => " \u{e86a} "
    };
}

pub fn get_icons(name: &str) -> Option<Map<String, String>> {
    match name {
        "material" => Some(MATERIAL.clone()),
        "awesome" => Some(AWESOME.clone()),
        "awesome5" => Some(AWESOME5.clone()),
        "none" => Some(NONE.clone()),
        _ => None,
    }
}

pub fn default() -> Map<String, String> {
    NONE.clone()
}
