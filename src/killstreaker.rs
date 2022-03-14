use strum_macros::{Display, EnumString};
use num_enum::{TryFromPrimitive, IntoPrimitive};
use serde_repr::{Serialize_repr, Deserialize_repr};

#[derive(Serialize_repr, Deserialize_repr, Debug, Hash, Eq, PartialEq, Display, EnumString, TryFromPrimitive, IntoPrimitive, Clone)]
#[repr(u32)]
pub enum Killstreaker {
    #[strum(serialize = "Fire Horns")]
    FireHorns = 2002,
    #[strum(serialize = "Cerebral Discharge")]
    CerebralDischarge = 2003,
    #[strum(serialize = "Tornado")]
    Tornado = 2004,
    #[strum(serialize = "Flames")]
    Flames = 2005,
    #[strum(serialize = "Singularity")]
    Singularity = 2006,
    #[strum(serialize = "Incinerator")]
    Incinerator = 2007,
    #[strum(serialize = "Hypno-Beam")]
    HypnoBeam = 2008,
}