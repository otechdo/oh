#[derive(Default)]
pub struct Mirrors {
    pub country: String,
    pub protocol: String,
    pub sort: String,
    pub save_at: String,
}

#[derive(Default)]
pub struct Keyboard {
    pub keymap: String,
    pub layout: String,
    pub model: String,
    pub options: String,
}

#[derive(Default)]
pub struct Os {
    pub mirrors: Mirrors,
    pub locale: String,
    pub keyboard: Keyboard,
    pub time_zone: String,
}
