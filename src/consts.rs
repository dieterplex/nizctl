use std::collections::HashMap;

pub const VENDOR_ID: u16 = 0x0483;
pub const INTERFACE_ID: i32 = 1;
lazy_static::lazy_static! {
    pub static ref PRODUCT_ID: HashMap<u16, &'static str> = {
        let mut map = HashMap::new();
        map.insert(0x512a, "atom66");
        map.insert(0x5129, "atom68");
        map
    };
}

#[repr(u16)]
pub enum OpCode {
    VersionRead = 0xf9,
    CounterRead = 0xe3,
    KeyLock = 0xd9,

    KeymapDataRead = 0xf2,
    KeymapDataStart = 0xf1,
    KeymapData = 0xf0,
    KeymapDataEnd = 0xf6,

    CalibInit = 0xdb,
    CalibPressed = 0xdd,
}

pub const KEY_CODE_NAME: [&str; 256] = [
    "KC_NO",
    "KC_ESC",
    "KC_F1",
    "KC_F2",
    "KC_F3",
    "KC_F4",
    "KC_F5",
    "KC_F6",
    "KC_F7",
    "KC_F8",
    "KC_F9",
    "KC_F10",
    "KC_F11",
    "KC_F12",
    "KC_GRV",
    "KC_1",
    "KC_2",
    "KC_3",
    "KC_4",
    "KC_5",
    "KC_6",
    "KC_7",
    "KC_8",
    "KC_9",
    "KC_0",
    "KC_MINS",
    "KC_EQL",
    "KC_BSPC",
    "KC_TAB",
    "KC_Q",
    "KC_W",
    "KC_E",
    "KC_R",
    "KC_T",
    "KC_Y",
    "KC_U",
    "KC_I",
    "KC_O",
    "KC_P",
    "KC_LBRC",
    "KC_RBRC",
    "KC_BSLS",
    "KC_CAPS",
    "KC_A",
    "KC_S",
    "KC_D",
    "KC_F",
    "KC_G",
    "KC_H",
    "KC_J",
    "KC_K",
    "KC_L",
    "KC_SCLN",
    "KC_QUOT",
    "KC_ENT",
    "KC_LSFT",
    "KC_Z",
    "KC_X",
    "KC_C",
    "KC_V",
    "KC_B",
    "KC_N",
    "KC_M",
    "KC_COMM",
    "KC_DOT",
    "KC_SLSH",
    "KC_RSHIFT",
    "KC_LCTL",
    "KC_LGUI",
    "KC_LALT",
    "KC_SPC",
    "KC_RALT",
    "KC_RGUI",
    "KC_APPLICATION",
    "KC_RCTL",
    "KC_SYSTEM_WAKE",
    "KC_SYSTEM_SLEEP",
    "KC_SYSTEM_POWER",
    "KC_PSCR",
    "KC_SLCK",
    "KC_PAUS",
    "KC_INS",
    "KC_HOME",
    "KC_PGUP",
    "KC_DEL",
    "KC_END",
    "KC_PGDN",
    "KC_UP",
    "KC_LEFT",
    "KC_DOWN",
    "KC_RGHT",
    "KC_NUMLOCK",
    "KC_KP_SLASH",
    "KC_KP_ASTERISK",
    "KC_KP_7",
    "KC_KP_8",
    "KC_KP_9",
    "KC_KP_4",
    "KC_KP_5",
    "KC_KP_6",
    "KC_KP_1",
    "KC_KP_2",
    "KC_KP_3",
    "KC_KP_0",
    "KC_KP_DOT",
    "KC_KP_MINUS",
    "KC_KP_PLUS",
    "KC_KP_ENTER",
    "KC_MEDIA_NEXT_TRACK",
    "KC_MEDIA_PREV_TRACK",
    "KC_MEDIA_STOP",
    "KC_MEDIA_PLAY_PAUSE",
    "KC_AUDIO_MUTE",
    "KC_AUDIO_VOL_UP",
    "KC_AUDIO_VOL_DOWN",
    "KC_MEDIA_SELECT",
    "KC_MAIL",
    "KC_CALCULATOR",
    "KC_MY_COMPUTER",
    "KC_WWW_SEARCH",
    "KC_WWW_HOME",
    "KC_WWW_BACK",
    "KC_WWW_FORWARD",
    "KC_WWW_STOP",
    "KC_WWW_REFRESH",
    "KC_WWW_FAVORITES",
    "KC_MS_L",
    "KC_MS_R",
    "KC_MS_U",
    "KC_MS_D",
    "KC_BTN1",
    "KC_BTN3",
    "KC_BTN2",
    "KC_MS_WH_UP",
    "KC_MS_WH_DOWN",
    "BL_TOGG",
    "KC_NO",            // backlight macro
    "RGB_MODE_RGBTEST", // backlight demo
    "RGB_MODE_RAINBOW", // backlight star shower
    "RGB_MODE_KNIGHT",  // backlight riffle
    "KC_NO",            // backlight demo stop
    "BL_BRTG",
    "KC_NO", // breathe sequence -
    "KC_NO", // breathe sequence +
    "BL_DEC",
    "BL_INC",
    "KC_NO",         // sunset or relax/aurora
    "KC_NO",         // color breathe
    "KC_NO",         // back color exchange
    "ANY(MI_TRNSU)", // adjust trigger point
    "ANY(MI_TOG)",   // keyboard lock
    "RSFT_T(KC_UP)",
    "CL_SWAP",
    "GUI_OFF",
    "KC_NO",       // mouse lock
    "ANY(UC_MOD)", // exchange win and mac
    "MO(1)",
    "ANY(MI_VELD)", // mouse unit pixel
    "ANY(MI_VELU)", // mouse unit time
    "ANY(MI_MOD)",  // toggle programmable
    "KC_NO",        // backlight record 1
    "KC_NO",        // backlight record 2
    "KC_NO",        // backlight record 3
    "KC_NO",        // backlight record 4
    "KC_NO",        // backlight record 5
    "KC_NO",        // backlight record 6
    "MO(2)",
    "ANY(MI_CHU)",   // wired/wireless
    "ANY(MI_CH1)",   // bluetooth device 1
    "ANY(MI_CH2)",   // bluetooth device 2
    "ANY(MI_CH3)",   // bluetooth device 3
    "ANY(MI_MODSD)", // eco mode
    "ANY(MI_MODSU)", // game mode
    "KC_NO",         // mouse first delay
    "ANY(MI_BENDD)", // key repeat rate
    "ANY(MI_BENDU)", // key response delay
    "KC_NO",         // usb report rate
    "KC_NO",         // key scan period
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
    "KC_NO",
];
