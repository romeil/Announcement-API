use lazy_static::lazy_static;

pub struct Settings {
    pub auth_cookie_name: String,
    pub admin_cookie_secret: String,
    pub club_cookie_secret: String,
    pub implicit_assertion: String,
}

lazy_static! {
    pub static ref SETTINGS: Settings = {
        Settings {
            auth_cookie_name: "auth".to_string(),
            admin_cookie_secret: "k4.local.0sQSpXmEi016zQbnsxAC4zjRcvYGH7r9tIyDCzu6pi8".to_string(),
            club_cookie_secret: "k4.local.cswAM65Umw_RpxdtkyTmjswdVTFwwhy_RLaTgaPy4TU".to_string(),
            implicit_assertion: "Some assertion".to_string(),
        }
    };
}

pub fn get_settings() -> &'static Settings {
    &SETTINGS
}
