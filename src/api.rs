use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ApiResponse {
    pub code: u16,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct Permission {
    #[serde(rename(deserialize = "type"))]
    pub role: String,

    pub r: u8,
    pub w: u8,
    pub c: u8,
}

#[derive(Serialize, Deserialize)]
pub struct BasicEnsemble {
    #[serde(rename(deserialize = "offlineMode"))]
    pub offline_mode: bool,

    #[serde(rename(deserialize = "accessToPerformerName"))]
    pub access_to_performer_name: bool,

    pub id: String,
    pub name: String,
    pub subscribed: bool,
    pub role: String,
    pub permissions: Vec<Permission>,
    pub instrument: String,
}

#[derive(Serialize, Deserialize)]
pub struct Me {
    #[serde(rename(deserialize = "_id"))]
    pub id: String,

    pub email: String,
    pub admin: bool,
    pub first_name: String,
    pub last_name: String,
    pub photo: String,

    #[serde(rename(deserialize = "ensemble"))]
    pub ensembles: Vec<BasicEnsemble>,

    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct Login {
    #[serde(flatten)]
    pub me: Me,

    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct Interval {
    pub checkin: u32,
    pub present: u32,
    pub tardy: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Selected {
    #[serde(rename(deserialize = "_id"))]
    pub id: String,

    #[serde(rename(deserialize = "id"))]
    pub google_id: String,

    pub channel_id: String,
    pub summary: String,

    #[serde(rename(deserialize = "colorId"))]
    pub color_id: String,

    #[serde(rename(deserialize = "backgroundColor"))]
    pub background_color: String,

    #[serde(rename(deserialize = "foregroundColor"))]
    pub foreground_color: String,

    pub sync_token: String,
    pub expiration: String,

    #[serde(rename(deserialize = "resourceId"))]
    pub resource_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct Calendar {
    pub selected: Vec<Selected>,
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct Ensemble {
    #[serde(rename(deserialize = "_id"))]
    pub id: String,

    pub interval: Interval,
    pub calendar: Calendar,
    pub users_count: u32,
    pub users_max: u32,
    pub code: String,

    #[serde(rename(deserialize = "isBlockedByAdmin"))]
    pub is_blocked_by_admin: bool,

    #[serde(rename(deserialize = "offlineMode"))]
    pub offline_mode: bool,

    pub languages: Vec<String>,
    pub sheet_music: bool,
    pub udb_app: bool,
    pub udb_app_pro: bool,
    pub name: String,

    #[serde(rename(deserialize = "subscriptionStart"))]
    pub subscription_start: String,

    #[serde(rename(deserialize = "subscriptionEnd"))]
    pub subscription_end: String,

    pub ensemble_password: String,
    pub subscribed: bool,

    #[serde(rename(deserialize = "createDate"))]
    pub create_date: String,

    pub addresses: Vec<String>,
    pub school_district: String,
    pub school_state: String,
    pub udb_app_trial: bool,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    #[serde(flatten)]
    pub api_response: ApiResponse,

    pub data: Option<Login>,
}

#[derive(Serialize, Deserialize)]
pub struct MeResponse {
    #[serde(flatten)]
    pub api_response: ApiResponse,

    pub data: Option<Me>,
}

#[derive(Serialize, Deserialize)]
pub struct EnsembleResponse {
    #[serde(flatten)]
    pub api_response: ApiResponse,

    pub data: Option<Ensemble>,
}
