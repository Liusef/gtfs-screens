use serde::{Deserialize, Serialize};

pub mod sfbart;
pub mod wmata;

#[derive(Serialize, Deserialize, Debug)]
pub enum Extensions {
    SFBART(sfbart::SfbartExt),
    WMATA(wmata::WmataExt),
}
