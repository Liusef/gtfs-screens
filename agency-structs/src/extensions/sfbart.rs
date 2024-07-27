use std::string::String;
use crate::structures::{Shortname, Longname};
use serde::{Serialize, Deserialize};

pub enum OpeningSide {
    Left, Right, Both, None(i16)
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SfbartExt {
    pub route       : Option<SfbartExtRoute>,
    // pub route_rt    : Option<SfbartExtRouteRt>,
    // pub station     : Option<SfbartExtStation>,
    // pub stop        : Option<SfbartExtStop>,
    // pub stop_arr    : Option<SfbartExtStopArrival>,
    // pub stop_rt     : Option<SfbartExtStopRt>,
    // pub trip_arr    : Option<SfbartExtTripArrival>,
    // pub trip        : Option<SfbartExtTrip>,
    pub trip_rt     : Option<SfbartExtTripRt>,
    pub advisory    : Option<SfbartExtAdvisory>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SfbartExtRoute {
    pub poi: Vec<(i32, Shortname, Longname)>,
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct SfbartExtRouteRt {

// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct SfbartExtStation {

// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct SfbartExtStop {

// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct SfbartExtStopArrival {

// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct SfbartExtStopRt {

// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct SfbartExtTripArrival {

// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct SfbartExtTrip {

// }

#[derive(Serialize, Deserialize, Debug)]
pub struct SfbartExtTripRt {
    pub train_generation: i32,
    pub num_cars: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SfbartExtAdvisory {
    pub title: String
}

