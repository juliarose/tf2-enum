use strum_macros::{Display, EnumString};

#[derive(Debug, Hash, Eq, PartialEq, Display, EnumString, Clone)]
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