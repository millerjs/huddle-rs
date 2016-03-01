use std::collections::HashMap;
use ncurses::*;


macro_rules! register_color {
    ($name: expr, $foreground: expr, $background: expr, $map: expr, $i: expr) => {
        {
            $i += 1;
            init_pair($i, $foreground, $background);
            $map.insert($name, $i);
        }
    };
}


lazy_static! {
    pub static ref CPAIRS: HashMap<&'static str, i16> = {
        let mut m = HashMap::new();
        let mut i = 0;
        register_color!("BLACK_ON_WHITE", COLOR_BLACK, COLOR_WHITE, m, i);
        register_color!("WHITE_ON_BLACK", COLOR_WHITE, COLOR_BLACK, m, i);
        register_color!("BLUE_ON_BLACK", COLOR_BLACK, COLOR_BLUE, m, i);
        register_color!("BLACK_ON_BLUE", COLOR_BLACK, COLOR_BLUE, m, i);
        m
    };
}
