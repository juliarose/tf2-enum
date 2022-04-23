use serde::Serialize;
use strum_macros::{Display, EnumString, EnumIter};

#[derive(Debug, Serialize, Hash, Eq, PartialEq, Display, EnumString, EnumIter, Clone, Copy)]
pub enum Class {
    Scout,
    Soldier,
    Pyro,
    Demoman,
    Heavy,
    Engineer,
    Medic,
    Sniper,
    Spy,
}