use strum_macros::{Display, EnumString, EnumIter};

#[derive(Debug, Hash, Eq, PartialEq, Display, EnumString, EnumIter, Clone, Copy)]
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