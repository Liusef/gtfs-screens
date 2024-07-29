use std::collections::{HashMap, HashSet};
use std::string::String;
use serde::{Serialize, Deserialize};
use crate::extensions::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum OpeningSide {
    Left, Right, Both, Other(i16)
}

pub type RouteId = String;
pub type StopId = String;
pub type TripId = String;
pub type Index = i32;
pub type Shortname = String;
pub type Longname = String;
pub type UnixTime = i64;
pub type PlatformNo = i32;
pub type Asset = String;


#[derive(Serialize, Deserialize, Debug)]
pub struct AgencyData {
    pub name: String,
    pub timezone: String,
    pub uri: String,
    pub static_uri: String,
    pub tripupdate_uri: Option<String>,
    pub vehiclepos_uri: Option<String>,
    pub advisories_uri: Option<String>,
    pub extension: Extensions,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Route {
    pub id: RouteId,
    pub shortname: Shortname,
    pub longname: Longname,
    pub asset: Option<Asset>,
    pub color: String,
    pub text_color: String, 
    pub trips: HashSet<TripId>,
    pub extension: Extensions,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RouteRealtime {
    pub id: RouteId,
    pub shortname: String,
    pub active_trips: HashSet<TripId>,
    pub extension: Extensions
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Station {
    pub platforms: HashMap<RouteId, PlatformNo>,
    pub opening_side: HashMap<RouteId, OpeningSide>,
    pub extension: Extensions
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stop {
    pub id: String,
    pub name: String, 
    pub routes: HashSet<RouteId>,
    pub travel_times: HashMap<RouteId, HashMap<StopId, UnixTime>>,  // Travel times to neighboring stops
    pub station_meta: Option<Station>,
    pub transfers: Option<Transfers>,
    pub extension: Extensions,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StopArrival {
    pub trip_id: String,
    pub est_arrival: UnixTime,
    pub extension: Extensions
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StopRealtime {
    pub id: StopId,
    pub name: String,
    pub arrivals: HashMap<PlatformNo, Vec<StopArrival>>,
    pub extension: Extensions
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Transfers {
    pub rail_transfers: HashMap<RouteId, Vec<RouteId>>,
    pub bus_transfers: HashMap<RouteId, Vec<RouteId>>,
    pub interagency_transfers: HashMap<RouteId, Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TripArrival {
    pub stop_id: StopId,
    pub arrival: UnixTime,
    pub departure: UnixTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Trip {
    pub id: TripId,
    pub route_id: RouteId,
    pub route_shortname: Shortname,
    pub departures: Vec<TripArrival>,
    pub extension: Extensions,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TripRealtime<T> {
    pub cancelled: bool,
    pub currprev_stop_id: StopId,
    pub next_stop_id: StopId,
    pub prev_dep_time: UnixTime,
    pub next_arr_time: UnixTime,
    pub progress: f64,
    pub extension: T,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Advisory<T> {
    pub msg_type: String,
    pub message: String,
    pub extension: T,
}







