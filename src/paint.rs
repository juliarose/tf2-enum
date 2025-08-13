use crate::Attribute;
use strum_macros::{Display, EnumString, EnumIter, EnumCount};
use num_enum::{TryFromPrimitive, IntoPrimitive};
use serde_repr::{Serialize_repr, Deserialize_repr};

/// Paint. `repr` values are mapped to the corresponding color. For team paints this is the color
/// for RED team.
#[derive(
    Serialize_repr,
    Deserialize_repr,
    Debug,
    Hash,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Display,
    EnumString,
    EnumIter,
    EnumCount,
    TryFromPrimitive,
    IntoPrimitive,
    Clone,
    Copy,
)]
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
    pub fn color(&self) -> u32 {
        u32::from(*self)
    }
    
    /// Converts a hexadecimal color into a [`Paint`].
    /// 
    /// # Examples
    /// ```
    /// use tf2_enum::Paint;
    /// 
    /// assert_eq!(Paint::from_color(0x7D4071).unwrap(), Paint::ADeepCommitmentToPurple);
    /// ```
    pub fn from_color(color: u32) -> Option<Self> {
        Self::try_from(color).ok()
    }
    
    /// Converts a hexadecimal color string into a [`Paint`].
    /// 
    /// # Examples
    /// ```
    /// use tf2_enum::Paint;
    /// 
    /// assert_eq!(Paint::from_color_str("#7D4071").unwrap(), Paint::ADeepCommitmentToPurple);
    /// ```
    pub fn from_color_str(color: &str) -> Option<Self> {
        let len = color.len();
        let mut color = color;
        
        if len == 7 && color.starts_with('#') {
            color = &color[1..len];
        } else if len != 6 {
            return None;
        }
        
        let color = u32::from_str_radix(color, 16).ok()?;
        
        Self::from_color(color)
    }
    
    /// Gets the colors for both teams. The RED team appears first. For non-team-color paints the
    /// color will be the same.
    pub fn colors(&self) -> (u32, u32) {
        match self {
            Self::AColorSimilarToSlate => (0x2F4F4F, 0x2F4F4F),
            Self::ADeepCommitmentToPurple => (0x7D4071, 0x7D4071),
            Self::ADistinctiveLackOfHue => (0x141414, 0x141414),
            Self::AMannsMint => (0xBCDDB3, 0xBCDDB3),
            Self::AfterEight => (0x2D2D24, 0x2D2D24),
            Self::AgedMoustacheGrey => (0x7E7E7E, 0x7E7E7E),
            Self::AnExtraordinaryAbundanceOfTinge => (0xE6E6E6, 0xE6E6E6),
            Self::AustraliumGold => (0xE7B53B, 0xE7B53B),
            Self::ColorNo216190216 => (0xD8BED8, 0xD8BED8),
            Self::DarkSalmonInjustice => (0xE9967A, 0xE9967A),
            Self::DrablyOlive => (0x808000, 0x808000),
            Self::IndubitablyGreen => (0x729E42, 0x729E42),
            Self::MannCoOrange => (0xCF7336, 0xCF7336),
            Self::Muskelmannbraun => (0xA57545, 0xA57545),
            Self::NobleHattersViolet => (0x51384A, 0x51384A),
            Self::PeculiarlyDrabTincture => (0xC5AF91, 0xC5AF91),
            Self::PinkAsHell => (0xFF69B4, 0xFF69B4),
            Self::RadiganConagherBrown => (0x694D3A, 0x694D3A),
            Self::TheBitterTasteOfDefeatAndLime => (0x32CD32, 0x32CD32),
            Self::TheColorOfAGentlemannsBusinessPants => (0xF0E68C, 0xF0E68C),
            Self::YeOldeRusticColour => (0x7C6C57, 0x7C6C57),
            Self::ZepheniahsGreed => (0x424F3B, 0x424F3B),
            Self::AnAirOfDebonair => (0x654740, 0x28394D),
            Self::BalaclavasAreForever => (0x3B1F23, 0x18233D),
            Self::CreamSpirit => (0xC36C2D, 0xB88035),
            Self::OperatorsOveralls => (0x483838, 0x384248),
            Self::TeamSpirit => (0xB8383B, 0x5885A2),
            Self::TheValueOfTeamwork => (0x803020, 0x256D8D),
            Self::WaterloggedLabCoat => (0xA89A8C, 0x839FA3),
        }
    }
    
    /// Determines if this paint is a team-colored paint.
    pub fn is_team_paint(&self) -> bool {
        let (red, blu) = self.colors();
        
        red != blu
    }
    
    /// Converts a `defindex` into its related [`Paint`], if it exists.
    pub fn from_defindex(defindex: u32) -> Option<Self> {
        match defindex {
            5052 => Some(Self::AColorSimilarToSlate),
            5031 => Some(Self::ADeepCommitmentToPurple),
            5040 => Some(Self::ADistinctiveLackOfHue),
            5076 => Some(Self::AMannsMint),
            5077 => Some(Self::AfterEight),
            5038 => Some(Self::AgedMoustacheGrey),
            5039 => Some(Self::AnExtraordinaryAbundanceOfTinge),
            5037 => Some(Self::AustraliumGold),
            5030 => Some(Self::ColorNo216190216),
            5056 => Some(Self::DarkSalmonInjustice),
            5053 => Some(Self::DrablyOlive),
            5027 => Some(Self::IndubitablyGreen),
            5032 => Some(Self::MannCoOrange),
            5033 => Some(Self::Muskelmannbraun),
            5029 => Some(Self::NobleHattersViolet),
            5034 => Some(Self::PeculiarlyDrabTincture),
            5051 => Some(Self::PinkAsHell),
            5035 => Some(Self::RadiganConagherBrown),
            5054 => Some(Self::TheBitterTasteOfDefeatAndLime),
            5055 => Some(Self::TheColorOfAGentlemannsBusinessPants),
            5036 => Some(Self::YeOldeRusticColour),
            5028 => Some(Self::ZepheniahsGreed),
            5063 => Some(Self::AnAirOfDebonair),
            5062 => Some(Self::BalaclavasAreForever),
            5065 => Some(Self::CreamSpirit),
            5060 => Some(Self::OperatorsOveralls),
            5046 => Some(Self::TeamSpirit),
            5064 => Some(Self::TheValueOfTeamwork),
            5061 => Some(Self::WaterloggedLabCoat),
            _ => None,
        }
    }
    
    /// Gets the `defindex` related to this [`Paint`].
    pub fn defindex(&self) -> u32 {
        match self {
            Self::AColorSimilarToSlate => 5052,
            Self::ADeepCommitmentToPurple => 5031,
            Self::ADistinctiveLackOfHue => 5040,
            Self::AMannsMint => 5076,
            Self::AfterEight => 5077,
            Self::AgedMoustacheGrey => 5038,
            Self::AnExtraordinaryAbundanceOfTinge => 5039,
            Self::AustraliumGold => 5037,
            Self::ColorNo216190216 => 5030,
            Self::DarkSalmonInjustice => 5056,
            Self::DrablyOlive => 5053,
            Self::IndubitablyGreen => 5027,
            Self::MannCoOrange => 5032,
            Self::Muskelmannbraun => 5033,
            Self::NobleHattersViolet => 5029,
            Self::PeculiarlyDrabTincture => 5034,
            Self::PinkAsHell => 5051,
            Self::RadiganConagherBrown => 5035,
            Self::TheBitterTasteOfDefeatAndLime => 5054,
            Self::TheColorOfAGentlemannsBusinessPants => 5055,
            Self::YeOldeRusticColour => 5036,
            Self::ZepheniahsGreed => 5028,
            Self::AnAirOfDebonair => 5063,
            Self::BalaclavasAreForever => 5062,
            Self::CreamSpirit => 5065,
            Self::OperatorsOveralls => 5060,
            Self::TeamSpirit => 5046,
            Self::TheValueOfTeamwork => 5064,
            Self::WaterloggedLabCoat => 5061,
        }
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
        assert_eq!(16738740_u32, Paint::PinkAsHell as u32);
    }
    
    #[test]
    fn gets_defindex() {
        assert_eq!(5051_u32, Paint::PinkAsHell.defindex());
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
        assert_eq!(Paint::from_color_str("FF69B4").unwrap(), Paint::PinkAsHell);
    }
    
    #[test]
    fn converts_from_hex_str_lowercase() {
        assert_eq!(Paint::from_color_str("ff69b4").unwrap(), Paint::PinkAsHell);
    }
    
    #[test]
    fn converts_from_hex_str_lowercase_with_pound() {
        assert_eq!(Paint::from_color_str("#FF69B4").unwrap(), Paint::PinkAsHell);
    }
}
