use std::{collections::{HashMap, HashSet}, fs};

use agency_structs::{structures, extensions::sfbart::{SfbartExt, SfbartExtTrip}, structures::Station};
use gtfs_structures::{self, Id, LocationType};

const SFBART_PREFIX: &str = "sfbart";
const SFBART_STATIC: &str = "https://www.bart.gov/dev/schedules/google_transit.zip";
const SFBART_RT_TRIPUPDATE: &str = "http://api.bart.gov/gtfsrt/tripupdate.aspx";
const SFBART_RT_ALERTS: &str = "http://api.bart.gov/gtfsrt/alerts.aspx";

const SFBART_POINTS_OF_INTEREST_ARR: [(&str, (&str, &str));4] = [
    ("SFIA", ("SFO Airport", "San Francisco International Airport")),
    ("BALB", ("SF", "San Francisco")),
    ("EMBR", ("SF", "San Francisco")),
    ("COLS", ("OAK Airport", "Oakland International Airport"))
];

pub fn ingest(path: &std::path::Path) {
    let gtfs_res = gtfs_structures::Gtfs::new(SFBART_STATIC);
    let mut gtfs = match gtfs_res {
        Ok(val) => val,
        Err(e) => panic!("{e}"),
    };

    let agency = init_agency(&gtfs);
    let mut routes = init_routes(&mut gtfs);
    let mut stops = init_stops(&mut gtfs);
    let mut trips = init_trips(&mut gtfs);
    hydrate_stage_1(&mut stops, &mut routes, &mut trips);
    hydrate_stage_2(&mut stops, &mut routes, &mut trips);

    // serialize
    let agency_serialized = serde_json::to_string_pretty(&agency).expect("Could not serialize agency");
    let route_serialized = serde_json::to_string_pretty(&routes).expect("Could not serialize route");
    let stops_serialized = serde_json::to_string_pretty(&stops).expect("Could not serialize stops");
    let trips_serialized = serde_json::to_string_pretty(&trips).expect("Could not serialize trips");

    let npath = path.join(SFBART_PREFIX);
    if !npath.exists() {
        fs::create_dir(&npath).expect(format!("Could not create folder {}", &npath.to_str().unwrap()).as_str());
    }

    fs::write(npath.join("agency.json"), &agency_serialized).expect("Could not write to file agency.json");
    fs::write(npath.join("routes.json"), &route_serialized).expect("Could not write to file routes.json");
    fs::write(npath.join("stops.json"), &stops_serialized).expect("Could not write to file stops.json");
    fs::write(npath.join("trips.json"), &trips_serialized).expect("Could not write to file trips.json");
}

type SfbartRoute = structures::Route<SfbartExt>;
type SfbartStop = structures::Stop<SfbartExt>;
type SfbartTrip = structures::Trip<SfbartExt>;

fn init_agency(gtfs: &gtfs_structures::Gtfs) -> structures::AgencyData<SfbartExt> {
    structures::AgencyData {
        name: gtfs.agencies[0].name.clone(),
        timezone: gtfs.agencies[0].timezone.clone(),
        uri: gtfs.agencies[0].url.clone(),
        static_uri: SFBART_STATIC.to_string(),
        tripupdate_uri: Some(SFBART_RT_TRIPUPDATE.to_string()),
        vehiclepos_uri: None,
        advisories_uri: Some(SFBART_RT_ALERTS.to_string()),
        extension: SfbartExt::default()
    }
}

fn init_routes(gtfs: &mut gtfs_structures::Gtfs) -> HashMap<String, SfbartRoute> {
    let mut retval: HashMap<String, SfbartRoute> = HashMap::new();
    
    // Iterate through all routes provided in GTFS static data
    for (key, val) in gtfs.routes.iter() {
        let id = &val.id().to_string();
        let (mut start, mut end) = ("Start".to_string(), "End".to_string());

        // Only way to get stop list is to find a trip under this route
        // then save the sequence of stops
        for (_, v1) in gtfs.trips.iter() {
            if &v1.route_id == id {
                start = v1.stop_times[0].stop.as_ref().name.as_ref().unwrap().to_string();
                end = v1.stop_times[v1.stop_times.len() - 1].stop.name.as_ref().unwrap().to_string();
            }
        }

        let mut rt = structures::Route{
            // ID is required
            id: id.to_owned(),

            // Shortname isn't always provided but usually is. If not provided defaults to "Route <id>"
            shortname: match val.short_name.clone() {
                Some(x) => x,
                None => format!("Route {id}")
            }, 

            // Longname isn't always provided. If not provided, defaults to "<Start> to <End> Line"
            longname: match val.long_name.clone() {
                Some(x) => x,
                None => format!("{start} to {end} Line")
            },
            asset: None,
            color: val.color.to_string(),
            text_color: val.text_color.to_string(),
            trips: HashSet::new(),
            extension: SfbartExt::default(),
        };

        // Generating a list of trips associated w/ this route
        for (k1, v1) in gtfs.trips.iter() {
            if &v1.route_id == id {
                rt.trips.insert(k1.to_string());
            }
        }
        retval.insert(key.to_string(), rt);
    }

    // remaining data to initialize in hydration or by hand:
    //  asset: Icon representing a given line, should probably use gen2 icon w/ associated color

    retval
}

fn init_stops(gtfs: &mut gtfs_structures::Gtfs) -> HashMap<String, SfbartStop> {
    let mut retval: HashMap<String, structures::Stop<SfbartExt>> = HashMap::new();

    // Loop through all stops in the system
    for (k, v) in gtfs.stops.iter() {
        if v.location_type != LocationType::StopPoint { continue }

        let mut stop = structures::Stop{
            id: v.id().to_string(),
            name: v.name.clone().unwrap(),
            routes: HashSet::new(),
            travel_times: HashMap::new(),
            station_meta: Some(Station{
                platforms: HashMap::new(),
                opening_side: HashMap::new(),
                extension: SfbartExt::default(),
            }),
            transfers: None,
            extension: SfbartExt::default(),
        };

        retval.insert(k.to_string(), stop);

    }

    // remaining data to initialize in hydration or by hand:
    //  routes
    //  travel_times
    //  all station metadata

    retval
}

fn init_trips(gtfs: &mut gtfs_structures::Gtfs) -> HashMap<String, SfbartTrip> {
    let mut r: HashMap<String, structures::Trip<SfbartExt>> = HashMap::new();

    for (k, v) in gtfs.trips.iter() {
        let mut tr = structures::Trip {
            id: v.id().to_string(),
            route_id: (&v.route_id).to_string(),
            route_shortname: String::new(),
            departures: vec!(),
            extension: SfbartExt::default(),
        };

        for (idx, item) in v.stop_times.iter().enumerate() {
            if idx > 0 && item.stop.id == v.stop_times[idx-1].stop.id {
                let depts = &mut tr.departures;
                let ld = depts.len();
                depts[ld - 1].departure = item.departure_time.unwrap() as i64;
                continue;
            }
            tr.departures.push(structures::TripArrival{
                trip_id: k.to_string(),
                stop_id: item.stop.id.to_string(),
                arrival: item.arrival_time.unwrap() as i64,
                departure: item.departure_time.unwrap() as i64,
                extension: SfbartExt::default(),
            })
        }

        tr.extension.trip = Some(SfbartExtTrip{
            poi: vec![]
        });
        
        r.insert(k.to_string(), tr);
    }

    r
}

fn hydrate_stage_1(stops: &mut HashMap<String, SfbartStop>, 
    routes: &mut HashMap<String, SfbartRoute>, 
    trips: &mut HashMap<String, SfbartTrip>) -> () {

        // Hydrate stop routes and travel times
        for (_, trip) in trips.iter() {
            for (idx, arrival) in trip.departures.iter().enumerate() {
                // hydrate which routes go to this station
                let stop = stops.get_mut(&arrival.stop_id).unwrap();
                stop.routes.insert(trip.route_id.to_string());

                // populate travel times
                if idx >= trip.departures.len() - 1 { continue } // need to see next stop on the line, if at EOL then what's the next stn?
                let next_arrival = &trip.departures[idx+1];
                let curr_diff = next_arrival.arrival - arrival.departure;

                let (rte, stn) = (trip.route_id.to_string(), next_arrival.stop_id.to_string());
                if !stop.travel_times.contains_key(&rte) {
                    stop.travel_times.insert(rte.clone(), HashMap::new());
                }
                let populated = stop.travel_times.contains_key(&stn);
                if !populated || populated && stop.travel_times[&rte][&stn] < curr_diff {
                    stop.travel_times.get_mut(&rte).unwrap().insert(stn.clone(), curr_diff);
                }
            }
        }


        // Hydrate points of interest along trips
        let poimap = HashMap::from(SFBART_POINTS_OF_INTEREST_ARR);
        for (_, trip) in trips.iter_mut() {
            let mut visited: HashSet<&str> = HashSet::new();
            if trip.extension.trip.is_none() {
                trip.extension.trip = Some(SfbartExtTrip {
                    poi: vec!()
                });
            }
            for (idx, arrival) in trip.departures.iter().enumerate() {
                if poimap.contains_key(arrival.stop_id.as_str()) {
                    let (sn, ln) = poimap[arrival.stop_id.as_str()];
                    if visited.contains(sn) { continue; }
                    visited.insert(sn);
                    trip.extension.trip.as_mut().unwrap().poi.push((idx as i32, sn.to_string(), ln.to_string()));
                }
            }
        }



}

fn hydrate_stage_2(stops: &mut HashMap<String, SfbartStop>, 
    routes: &mut HashMap<String, SfbartRoute>, 
    trips: &mut HashMap<String, SfbartTrip>) -> () {

}
