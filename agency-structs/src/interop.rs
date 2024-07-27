use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Asset {
    pub path: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Departure {
    pub remaining_time: i32,
    pub dest: String,
    pub dest_extended: Option<String>,
    pub line_id: String,
    pub line_name: String,
    pub line_shortname: Option<String>,
    pub line_icon: Option<Asset>,
    pub other: HashMap<String, String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transfer {
    pub line_id: String,
    pub line_name: String,
    pub line_shortname: Option<String>,
    pub line_icon: Option<Asset>,
    pub line_color: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TripStatus {
    pub dest: String,
    pub line_id: String,
    pub line_name: String,
    pub line_color: String,
    pub next: String,
    pub travel_prog: f64,
    pub transfers: Vec<Transfer>,
    pub other: HashMap<String, String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Announcement {
    pub advisory: bool,
    pub important: bool,
    pub category: Option<String>,
    pub content: String,
    pub other: HashMap<String, String>
}


