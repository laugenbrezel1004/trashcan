use users::os::unix::UserExt;
use users::{get_current_uid, get_user_by_uid};

// hier noch mehr werte definieren, soll auch Umgebungsvariablen und.config gelesen werden
pub struct Trashcan {
    pub trashcan_homedirectory_location: String,
}

pub fn initialize_trashcan() -> Result<Trashcan, String> {
    let trashcan = Trashcan {
        trashcan_homedirectory_location: get_user_by_uid(get_current_uid())
            .map(|user| format!("{}/.local/share/trashcan", user.home_dir().display()))
            .unwrap_or_else(|| "error".to_string()),
    };

    trashcan.create_trashcan_directory()?; 
    Ok(trashcan)
}

