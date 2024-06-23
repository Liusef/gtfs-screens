use std::collections::{HashMap, HashSet};
use std::string::String;


pub struct Line {
    pub id: String,
    pub shortname: String,
    pub longname: String,
    pub color: String,
    pub route: Vec<String>,
    pub poi: Vec<(i32, String, String)>,
    pub trips: HashSet<String>
}

pub struct LineRealtime {
    pub id: String,
    pub shortname: String, 
    pub active_trips: HashSet<String>
}

pub struct Station {
    pub id: String,
    pub name: String, 
    pub routes: Vec<String>,
    pub routes_shortname: Vec<String>,
    pub line_index: HashMap<String, i32>,
    pub platforms: HashMap<String, i32>,
    pub opening_side: String,
    pub transfers: HashMap<String, Vec<(String, String)>>,
    pub travel_times: HashMap<String, i64>
}

pub struct StationArrival {
    pub trip_id: String,
    pub est_arrival: i64
}

pub struct StationRealtime {
    pub id: String,
    pub name: String,
    pub arrivals: HashMap<i32, Vec<StationArrival>>
}

pub struct TripArrival {
    pub trip_id: String,
    pub station_id: String,
    pub line_id: String,
    pub arrival: i64,
    pub departure: i64
}

pub struct Trip {
    pub id: String,
    pub line_id: String,
    pub line_shortname: String,
    pub departures: HashMap<String, TripArrival>
}

pub struct TripRealtime {
    pub cancelled: bool,
    pub train_generation: i32,
    pub num_cars: i32,
    pub state: String,
    pub currprev_stn_id: String,
    pub next_stn_id: String,
    pub prev_dep_time: i64,
    pub next_arr_time: i64,
    pub progress: f64
}

pub struct Advisory {
    pub msg_type: String,
    pub message: String
}
