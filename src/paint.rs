use strum_macros::{Display, EnumString, EnumIter};
use num_enum::{TryFromPrimitive, IntoPrimitive};
use serde_repr::{Serialize_repr, Deserialize_repr};
use crate::Attribute;

#[derive(Serialize_repr, Deserialize_repr, Debug, Hash, Eq, PartialEq, Display, EnumString, EnumIter, TryFromPrimitive, IntoPrimitive, Clone)]
#[repr(u32)]
pub enum Paint {
    #[strum(serialize = "A Color Similar to Slate")]
    AColorSimilarToSlate = 3100495,
    #[strum(serialize = "A Deep Commitment to Purple")]
    ADeepCommitmentToPurple = 8208497,
    #[strum(serialize = "A Distinctive Lack of Hue")]
    ADistinctiveLackOfHue = 1315860,
    #[strum(serialize = "A Mann's Mint")]
    AMannsMint = 12377523,
    #[strum(serialize = "After Eight")]
    AfterEight = 2960676,
    #[strum(serialize = "Aged Moustache Grey")]
    AgedMoustacheGrey = 8289918,
    #[strum(serialize = "An Extraordinary Abundance of Tinge")]
    AnExtraordinaryAbundanceOfTinge = 15132390,
    #[strum(serialize = "Australium Gold")]
    AustraliumGold = 15185211,
    #[strum(serialize = "Color No. 216-190-216")]
    ColorNo216190216 = 14204632,
    #[strum(serialize = "Dark Salmon Injustice")]
    DarkSalmonInjustice = 15308410,
    #[strum(serialize = "Drably Olive")]
    DrablyOlive = 8421376,
    #[strum(serialize = "Indubitably Green")]
    IndubitablyGreen = 7511618,
    #[strum(serialize = "Mann Co. Orange")]
    MannCoOrange = 13595446,
    #[strum(serialize = "Muskelmannbraun")]
    Muskelmannbraun = 10843461,
    #[strum(serialize = "Noble Hatter's Violet")]
    NobleHattersViolet = 5322826,
    #[strum(serialize = "Peculiarly Drab Tincture")]
    PeculiarlyDrabTincture = 12955537,
    #[strum(serialize = "Pink as Hell")]
    PinkAsHell = 16738740,
    #[strum(serialize = "Radigan Conagher Brown")]
    RadiganConagherBrown = 6901050,
    #[strum(serialize = "The Bitter Taste of Defeat and Lime")]
    TheBitterTasteOfDefeatAndLime = 3329330,
    #[strum(serialize = "The Color of a Gentlemann's Business Pants")]
    TheColorOfAGentlemannsBusinessPants = 15787660,
    #[strum(serialize = "Ye Olde Rustic Colour")]
    YeOldeRusticColour = 8154199,
    #[strum(serialize = "Zepheniah's Greed")]
    ZepheniahsGreed = 4345659,
    #[strum(serialize = "An Air of Debonair")]
    AnAirOfDebonair = 6637376,
    #[strum(serialize = "Balaclavas Are Forever")]
    BalaclavasAreForever = 3874595,
    #[strum(serialize = "Cream Spirit")]
    CreamSpirit = 12807213,
    #[strum(serialize = "Operator's Overalls")]
    OperatorsOveralls = 4732984,
    #[strum(serialize = "Team Spirit")]
    TeamSpirit = 12073019,
    #[strum(serialize = "The Value of Teamwork")]
    TheValueOfTeamwork = 8400928,
    #[strum(serialize = "Waterlogged Lab Coat")]
    WaterloggedLabCoat = 11049612,
}

impl Paint {
    
    pub fn from_color_str(color: &str) -> Option<Self> {
        if color.len() != 6 {
            return None;
        }
        
        match color.to_ascii_uppercase().as_str() {
            "2F4F4F" => Some(Paint::AColorSimilarToSlate),
            "7D4071" => Some(Paint::ADeepCommitmentToPurple),
            "141414" => Some(Paint::ADistinctiveLackOfHue),
            "BCDDB3" => Some(Paint::AMannsMint),
            "2D2D24" => Some(Paint::AfterEight),
            "7E7E7E" => Some(Paint::AgedMoustacheGrey),
            "E6E6E6" => Some(Paint::AnExtraordinaryAbundanceOfTinge),
            "E7B53B" => Some(Paint::AustraliumGold),
            "D8BED8" => Some(Paint::ColorNo216190216),
            "E9967A" => Some(Paint::DarkSalmonInjustice),
            "808000" => Some(Paint::DrablyOlive),
            "729E42" => Some(Paint::IndubitablyGreen),
            "CF7336" => Some(Paint::MannCoOrange),
            "A57545" => Some(Paint::Muskelmannbraun),
            "51384A" => Some(Paint::NobleHattersViolet),
            "C5AF91" => Some(Paint::PeculiarlyDrabTincture),
            "FF69B4" => Some(Paint::PinkAsHell),
            "694D3A" => Some(Paint::RadiganConagherBrown),
            "32CD32" => Some(Paint::TheBitterTasteOfDefeatAndLime),
            "F0E68C" => Some(Paint::TheColorOfAGentlemannsBusinessPants),
            "7C6C57" => Some(Paint::YeOldeRusticColour),
            "424F3B" => Some(Paint::ZepheniahsGreed),
            "654740" => Some(Paint::AnAirOfDebonair),
            "28394D" => Some(Paint::AnAirOfDebonair),
            "3B1F23" => Some(Paint::BalaclavasAreForever),
            "18233D" => Some(Paint::BalaclavasAreForever),
            "C36C2D" => Some(Paint::CreamSpirit),
            "B88035" => Some(Paint::CreamSpirit),
            "483838" => Some(Paint::OperatorsOveralls),
            "384248" => Some(Paint::OperatorsOveralls),
            "B8383B" => Some(Paint::TeamSpirit),
            "5885A2" => Some(Paint::TeamSpirit),
            "803020" => Some(Paint::TheValueOfTeamwork),
            "256D8D" => Some(Paint::TheValueOfTeamwork),
            "A89A8C" => Some(Paint::WaterloggedLabCoat),
            "839FA3" => Some(Paint::WaterloggedLabCoat),
            _ => None,
        }
    }
    
    pub fn defindex(&self) -> u32 {
        match self {
            Paint::AColorSimilarToSlate => 5052,
            Paint::ADeepCommitmentToPurple => 5031,
            Paint::ADistinctiveLackOfHue => 5040,
            Paint::AMannsMint => 5076,
            Paint::AfterEight => 5077,
            Paint::AgedMoustacheGrey => 5038,
            Paint::AnExtraordinaryAbundanceOfTinge => 5039,
            Paint::AustraliumGold => 5037,
            Paint::ColorNo216190216 => 5030,
            Paint::DarkSalmonInjustice => 5056,
            Paint::DrablyOlive => 5053,
            Paint::IndubitablyGreen => 5027,
            Paint::MannCoOrange => 5032,
            Paint::Muskelmannbraun => 5033,
            Paint::NobleHattersViolet => 5029,
            Paint::PeculiarlyDrabTincture => 5034,
            Paint::PinkAsHell => 5051,
            Paint::RadiganConagherBrown => 5035,
            Paint::TheBitterTasteOfDefeatAndLime => 5054,
            Paint::TheColorOfAGentlemannsBusinessPants => 5055,
            Paint::YeOldeRusticColour => 5036,
            Paint::ZepheniahsGreed => 5028,
            Paint::AnAirOfDebonair => 5063,
            Paint::BalaclavasAreForever => 5062,
            Paint::CreamSpirit => 5065,
            Paint::OperatorsOveralls => 5060,
            Paint::TeamSpirit => 5046,
            Paint::TheValueOfTeamwork => 5064,
            Paint::WaterloggedLabCoat => 5061,
        }
    }
    
    pub fn color(&self) -> u32 {
        u32::from(self.clone())
    }
    
    pub fn colors(&self) -> (u32, u32) {
        match self {
            Paint::AColorSimilarToSlate => (0x2F4F4F, 0x2F4F4F),
            Paint::ADeepCommitmentToPurple => (0x7D4071, 0x7D4071),
            Paint::ADistinctiveLackOfHue => (0x141414, 0x141414),
            Paint::AMannsMint => (0xBCDDB3, 0xBCDDB3),
            Paint::AfterEight => (0x2D2D24, 0x2D2D24),
            Paint::AgedMoustacheGrey => (0x7E7E7E, 0x7E7E7E),
            Paint::AnExtraordinaryAbundanceOfTinge => (0xE6E6E6, 0xE6E6E6),
            Paint::AustraliumGold => (0xE7B53B, 0xE7B53B),
            Paint::ColorNo216190216 => (0xD8BED8, 0xD8BED8),
            Paint::DarkSalmonInjustice => (0xE9967A, 0xE9967A),
            Paint::DrablyOlive => (0x808000, 0x808000),
            Paint::IndubitablyGreen => (0x729E42, 0x729E42),
            Paint::MannCoOrange => (0xCF7336, 0xCF7336),
            Paint::Muskelmannbraun => (0xA57545, 0xA57545),
            Paint::NobleHattersViolet => (0x51384A, 0x51384A),
            Paint::PeculiarlyDrabTincture => (0xC5AF91, 0xC5AF91),
            Paint::PinkAsHell => (0xFF69B4, 0xFF69B4),
            Paint::RadiganConagherBrown => (0x694D3A, 0x694D3A),
            Paint::TheBitterTasteOfDefeatAndLime => (0x32CD32, 0x32CD32),
            Paint::TheColorOfAGentlemannsBusinessPants => (0xF0E68C, 0xF0E68C),
            Paint::YeOldeRusticColour => (0x7C6C57, 0x7C6C57),
            Paint::ZepheniahsGreed => (0x424F3B, 0x424F3B),
            Paint::AnAirOfDebonair => (0x654740, 0x28394D),
            Paint::BalaclavasAreForever => (0x3B1F23, 0x18233D),
            Paint::CreamSpirit => (0xC36C2D, 0xB88035),
            Paint::OperatorsOveralls => (0x483838, 0x384248),
            Paint::TeamSpirit => (0xB8383B, 0x5885A2),
            Paint::TheValueOfTeamwork => (0x803020, 0x256D8D),
            Paint::WaterloggedLabCoat => (0xA89A8C, 0x839FA3),
        }
    }
    
    pub fn is_team_paint(&self) -> bool {
        let (red, blu) = self.colors();
        
        red != blu
    }
}

impl Attribute for Paint {
    const DEFINDEX: u32 = 142;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_to_primitive() {
        assert_eq!(16738740 as u32, Paint::PinkAsHell.into());
    }
    
    #[test]
    fn gets_defindex() {
        assert_eq!(5051 as u32, Paint::PinkAsHell.defindex());
    }
    
    #[test]
    fn displays_as_string() {
        assert_eq!("Pink as Hell", &format!("{}", Paint::PinkAsHell));
    }
    
    #[test]
    fn converts_to_hex() {
        assert_eq!(0xFF69B4, Paint::PinkAsHell.color());
    }
    
    #[test]
    fn converts_from_hex_str() {
        assert_eq!(Paint::from_color_str("839FA3").unwrap(), Paint::WaterloggedLabCoat);
    }
    
    #[test]
    fn converts_from_hex_str_lowercase() {
        assert_eq!(Paint::from_color_str("839fa3").unwrap(), Paint::WaterloggedLabCoat);
    }
}