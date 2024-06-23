use std::collections::{HashMap, HashSet};

use gtfs_structures::{self, Id};
pub use agency_structs::sfbart;


const SFBART_STATIC: &str = "https://www.bart.gov/dev/schedules/google_transit.zip";
const SFBART_RT_TRIPUPDATE: &str = "http://api.bart.gov/gtfsrt/tripupdate.aspx";
const SFBART_RT_ALERTS: &str = "http://api.bart.gov/gtfsrt/alerts.aspx";

fn ingest() {
    let gtfs = gtfs_structures::Gtfs::new(SFBART_STATIC);


}

fn gen_lines(gtfs: &mut gtfs_structures::Gtfs) -> HashMap<String, sfbart::Line> {
    let mut r: HashMap<String, sfbart::Line> = HashMap::new();
    
    for (key, val) in gtfs.routes.iter() {
        let id = &val.id().to_string();
        let mut stops: Vec<String> = vec!();
        for (_, v1) in gtfs.trips.iter() {
            if &v1.route_id == id {
                for v in &v1.stop_times {
                    stops.push(v.stop.id.clone());
                }
                break;
            }
        }
        let start = &stops[0];
        let end = &stops[stops.len()-1];
        let mut rt = sfbart::Line{
            id: id.to_owned(),
            shortname: match val.short_name.clone() {
                Some(x) => x,
                None => format!("Route {id}")
            },
            longname: match val.long_name.clone() {
                Some(x) => x,
                None => format!("{start} to {end}")
            },
            color: val.text_color.to_string(),
            route: stops,
            poi: vec!((10, format!("Shortname"), format!("Longname"))),
            trips: HashSet::new()
        };
        for (k1, v1) in gtfs.trips.iter() {
            if &v1.route_id == id {
                rt.trips.insert(k1.clone());
            }
        }
        r.insert(key.to_string(), rt);
    }

    r
}

fn gen_stations(gtfs: gtfs_structures::Gtfs) -> HashMap<String, sfbart::Station> {
    todo!()
}

fn gen_trips(gtfs: gtfs_structures::Gtfs) -> HashMap<String, sfbart::Trip> {
    todo!()
}