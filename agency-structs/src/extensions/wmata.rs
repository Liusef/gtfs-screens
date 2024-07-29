use std::string::String;
use crate::structures::{Shortname, Longname};
use serde::{Serialize, Deserialize};

pub enum OpeningSide {
    Left, Right, Both, None(i16)
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct WmataExt {
    // pub agency      : Option<WmataExtAgency>,
    // pub route       : Option<WmataExtRoute>,
    // pub route_rt    : Option<WmataExtRouteRt>,
    // pub station     : Option<WmataExtStation>,
    // pub stop        : Option<WmataExtStop>,
    // pub stop_arr    : Option<WmataExtStopArrival>,
    // pub stop_rt     : Option<WmataExtStopRt>,
    // pub trip_arr    : Option<WmataExtTripArrival>,
    // pub trip        : Option<WmataExtTrip>,
    // pub trip_rt     : Option<WmataExtTripRt>,
    // pub advisory    : Option<WmataExtAdvisory>
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct WmataExtAgency {
    
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct WmataExtRoute {
    
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct WmataExtRouteRt {

// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct WmataExtStation {

// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct WmataExtStop {

// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct WmataExtStopArrival {

// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct WmataExtStopRt {

// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct WmataExtTripArrival {

// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct WmataExtTrip {

// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct WmataExtTripRt {

// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct WmataExtAdvisory {

// }

