mod utils;

use wasm_bindgen::prelude::*;
use std::{collections::HashMap, rc::Rc};
use agency_structs::structures::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, gtfs-screens-backend!");
}

pub enum _State {
    Uninitialized,
    StaticLoaded,
    Running,
    Stopped,
    Other(i32)
}

pub struct _InternalState {
    pub state: _State,
    pub provider: String,
    pub agency_data: AgencyData,
    pub static_routes: HashMap<RouteId, Route>,
    pub static_stops: HashMap<StopId, Stop>,
    pub static_trips: HashMap<TripId, Trip>
}

pub static mut STATE: Option<Rc<_InternalState>> = None;

#[wasm_bindgen]
pub async fn load_provider(provider: &str) {
    let 狗屎 = "hello";
}

