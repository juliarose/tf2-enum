//! Set for holding up to 3 strange parts.

use crate::{Attributes, AttributeSet, StrangePart, TryFromAttributeValueU32};
use crate::error::InsertError;
use crate::serialize::SerializedAttribute;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{BitAnd, Sub};
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::ser::SerializeSeq;
use serde::de::{SeqAccess, Visitor};

const STRANGE_PART_COUNT: usize = 3;

/// Contains up to 3 strange parts. Although the underlying data structure is an array, this structure
/// behaves like a set. Most methods mimic those of [`HashSet`](std::collections::HashSet).
/// 
/// This struct solves the following problems:
/// - An item can only hold up to 3 strange parts.
/// - An item cannot have duplicate strange parts.
/// - Comparing strange parts for equality is order-agnostic.
/// - Hashing is order-agnostic.
/// - The type is `Copy`, allowing for cheap and easy duplication.
/// 
/// Most methods require pulling in the [`AttributeSet`] trait.
/// 
/// # Examples
/// ```
/// use tf2_enum::{StrangePartSet, StrangePart, AttributeSet};
/// 
/// // Create a set for strange parts with two strange parts.
/// let mut strange_parts = StrangePartSet::double(
///     StrangePart::CriticalKills,
///     StrangePart::DamageDealt,
/// );
/// 
/// // Check that strange parts contains Damage Dealt.
/// assert!(strange_parts.contains(&StrangePart::DamageDealt));
/// assert_eq!(strange_parts.len(), 2);
/// 
/// // Add a strange part.
/// strange_parts.insert(StrangePart::EngineersKilled);
/// 
/// assert_eq!(strange_parts.len(), 3);
/// 
/// // If a strange part is added when strange parts are full, the insert will fail.
/// assert!(!strange_parts.insert(StrangePart::MedicsKilled));
/// assert!(!strange_parts.contains(&StrangePart::MedicsKilled));
/// 
/// // Iterate over strange parts.
/// for strange_part in strange_parts {
///     println!("{strange_part}");
/// }
/// ```
#[derive(Debug, Default, Clone, Copy, Eq, PartialOrd, Ord)]
pub struct StrangePartSet {
    inner: [Option<StrangePart>; STRANGE_PART_COUNT],
}

impl StrangePartSet {
    /// Creates a set for strange parts.
    /// 
    /// # Examples
    /// ```
    /// use tf2_enum::StrangePartSet;
    /// 
    /// let strange_parts = StrangePartSet::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Creates a set for strange parts with one strange part.
    /// 
    /// # Examples
    /// ```
    /// use tf2_enum::{StrangePartSet, StrangePart, AttributeSet};
    /// 
    /// let strange_parts = StrangePartSet::single(
    ///     StrangePart::DamageDealt,
    /// );
    /// 
    /// assert_eq!(strange_parts.len(), 1);
    /// ```
    pub fn single(strange_part: StrangePart) -> Self {
        Self::from([
            Some(strange_part),
            None,
            None,
        ])
    }
    
    /// Creates a set for strange parts with two strange parts.
    /// 
    /// If the same strange part is added multiple times, only one will be kept.
    /// 
    /// # Examples
    /// ```
    /// use tf2_enum::{StrangePartSet, StrangePart, AttributeSet};
    /// 
    /// let strange_parts = StrangePartSet::double(
    ///     StrangePart::DamageDealt,
    ///     StrangePart::CriticalKills,
    /// );
    /// 
    /// assert_eq!(strange_parts.len(), 2);
    /// ```
    pub fn double(
        strange_part1: StrangePart,
        strange_part2: StrangePart,
    ) -> Self {
        Self::from([
            Some(strange_part1),
            Some(strange_part2),
            None,
        ])
    }
    
    /// Creates a set for strange parts with two strange parts.
    /// 
    /// If the same strange part is added multiple times, only one will be kept.
    /// 
    /// # Examples
    /// ```
    /// use tf2_enum::{StrangePartSet, StrangePart, AttributeSet};
    /// 
    /// let strange_parts = StrangePartSet::triple(
    ///     StrangePart::DamageDealt,
    ///     StrangePart::CriticalKills,
    ///     StrangePart::EngineersKilled,
    /// );
    /// 
    /// assert_eq!(strange_parts.len(), 3);
    /// ```
    pub fn triple(
        strange_part1: StrangePart,
        strange_part2: StrangePart,
        strange_part3: StrangePart,
    ) -> Self {
        Self::from([
            Some(strange_part1),
            Some(strange_part2),
            Some(strange_part3),
        ])
    }
}

impl AttributeSet for StrangePartSet {
    /// Max number of items.
    const MAX_COUNT: usize = STRANGE_PART_COUNT;
    /// An empty [`StrangePartSet`].
    const NONE: Self = Self {
        inner: [None, None, None],
    };
    /// The item type.
    type Item = StrangePart;
    
    /// Clears the set, removing all strange parts.
    /// 
    /// # Examples
    /// ```
    /// use tf2_enum::{StrangePartSet, StrangePart, AttributeSet};
    /// 
    /// let mut strange_parts = StrangePartSet::double(
    ///     StrangePart::CriticalKills,
    ///     StrangePart::DamageDealt,
    /// );
    /// 
    /// strange_parts.clear();
    /// 
    /// assert_eq!(strange_parts.len(), 0);
    /// ```
    fn clear(&mut self) {
        self.inner = [None, None, None];
    }
    
    /// Adds a strange part to the first available slot. If no slots are available, the new strange 
    /// part will be ignored.
    /// 
    /// Returns `false` if:
    /// - The strange part is already in the set.
    /// - The set is full.
    /// 
    /// # Examples
    /// ```
    /// use tf2_enum::{StrangePartSet, StrangePart, AttributeSet};
    /// 
    /// let mut strange_parts = StrangePartSet::double(
    ///     StrangePart::CriticalKills,
    ///     StrangePart::DamageDealt,
    /// );
    /// 
    /// assert_eq!(strange_parts.len(), 2);
    /// 
    /// strange_parts.insert(StrangePart::EngineersKilled);
    /// 
    /// assert_eq!(strange_parts.len(), 3);
    /// 
    /// // Strange parts are full.
    /// assert!(!strange_parts.insert(StrangePart::MedicsKilled));
    /// ```
    fn insert(&mut self, strange_part: StrangePart) -> bool {
        self.try_insert(strange_part).is_ok()
    }
    
    fn try_insert(&mut self, strange_part: Self::Item) -> Result<(), InsertError> {
        if self.contains(&strange_part) {
            return Err(InsertError::Duplicate);
        }
        
        if let Some(slot) = self.inner.iter_mut().find(|slot| slot.is_none()) {
            *slot = Some(strange_part);
            return Ok(());
        }
        
        // full set, insertion failed
        Err(InsertError::Full)
    }
    
    /// Removes a strange part.
    /// 
    /// # Examples
    /// ```
    /// use tf2_enum::{StrangePartSet, StrangePart, AttributeSet};
    /// 
    /// let mut strange_parts = StrangePartSet::single(StrangePart::CriticalKills);
    /// 
    /// assert!(strange_parts.remove(&StrangePart::CriticalKills));
    /// assert!(!strange_parts.contains(&StrangePart::CriticalKills));
    /// ```
    fn remove(&mut self, strange_part: &StrangePart) -> bool {
        for s in self.inner.iter_mut() {
            if *s == Some(*strange_part) {
                *s = None;
                return true;
            }
        }
        
        false
    }
    
    /// Removes and returns the strange part in the set, if any, that is equal to the given one.
    fn take(&mut self, strange_part: &StrangePart) -> Option<StrangePart> {
        for s in self.inner.iter_mut() {
            if *s == Some(*strange_part) {
                *s = None;
                return Some(*strange_part);
            }
        }
        
        None
    }
    
    /// Returns the inner storage as a slice.
    fn as_slice(&self) -> &[Option<StrangePart>] {
        &self.inner
    }
    
    /// Returns the inner storage as a mutable slice.
    fn as_mut_slice(&mut self) -> &mut [Option<StrangePart>] {
        &mut self.inner
    }
}

// Only Sub is implemented because Add wouldn't make much sense with strange parts being limited 
// to 3.
impl Sub for StrangePartSet {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self::Output {
        self.difference(&other)
    }
}

impl Sub for &StrangePartSet {
    type Output = StrangePartSet;
    
    fn sub(self, other: &StrangePartSet) -> Self::Output {
        self.difference(other)
    }
}

impl BitAnd for StrangePartSet {
    type Output = Self;
    
    fn bitand(self, other: Self) -> Self::Output {
        self.intersection(&other)
    }
}

impl BitAnd for &StrangePartSet {
    type Output = StrangePartSet;
    
    fn bitand(self, other: &StrangePartSet) -> Self::Output {
        self.intersection(other)
    }
}

impl PartialEq<Self> for StrangePartSet {
    fn eq(&self, other: &Self) -> bool {
        let mut a = self.inner;
        let mut b = other.inner;
        
        a.sort_unstable();
        b.sort_unstable();
        
        a == b
    }
}

impl Hash for StrangePartSet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut values = self.inner;
        
        values.sort_unstable();
        
        for value in values {
            value.hash(state);
        }
    }
}

impl From<[Option<StrangePart>; STRANGE_PART_COUNT]> for StrangePartSet {
    fn from(inner: [Option<StrangePart>; STRANGE_PART_COUNT]) -> Self {
        let mut inner = inner;
        
        // remove duplicates
        for i in 0..STRANGE_PART_COUNT {
            if let Some(val_i) = inner[i] {
                // check elements after i for duplicates
                for j in (i + 1)..STRANGE_PART_COUNT {
                    if inner[j] == Some(val_i) {
                        // later occurrence exists, remove current
                        inner[i] = None;
                        break;
                    }
                }
            }
        }
        
        Self {
            inner,
        }
    }
}

impl From<StrangePartSet> for Vec<StrangePart>{
    fn from(spell_set: StrangePartSet) -> Self {
        spell_set.into_iter().collect()
    }
}

impl From<&StrangePartSet> for Vec<StrangePart> {
    fn from(spell_set: &StrangePartSet) -> Self {
        (*spell_set).into()
    }
}

impl FromIterator<StrangePart> for StrangePartSet {
    fn from_iter<I: IntoIterator<Item = StrangePart>>(iter: I) -> Self {
        let mut strange_parts = Self::new();
        
        for strange_part in iter {
            strange_parts.insert(strange_part);
        }
        
        strange_parts
    }
}

impl<'a> FromIterator<&'a StrangePart> for StrangePartSet {
    fn from_iter<I: IntoIterator<Item = &'a StrangePart>>(iter: I) -> Self {
        let mut strange_part_set = Self::new();
        
        for strange_part in iter {
            strange_part_set.insert(*strange_part);
        }
        
        strange_part_set
    }
}

impl FromIterator<Option<StrangePart>> for StrangePartSet {
    fn from_iter<I: IntoIterator<Item = Option<StrangePart>>>(iter: I) -> Self {
        let mut set = Self::new();
        
        for val in iter.into_iter().flatten() {
            set.insert(val);
        }
        
        set
    }
}

impl<'a> FromIterator<Option<&'a StrangePart>> for StrangePartSet {
    fn from_iter<I: IntoIterator<Item = Option<&'a StrangePart>>>(iter: I) -> Self {
        let mut set = Self::new();
        
        for val in iter.into_iter().flatten() {
            set.insert(*val);
        }
        
        set
    }
}

impl IntoIterator for StrangePartSet {
    type Item = StrangePart;
    type IntoIter = StrangePartSetIterator;
    
    fn into_iter(self) -> Self::IntoIter {
        StrangePartSetIterator {
            inner: self.inner.into_iter(),
        }
    }
}

impl IntoIterator for &StrangePartSet {
    type Item = StrangePart;
    type IntoIter = StrangePartSetIterator;
    
    fn into_iter(self) -> Self::IntoIter {
        (*self).into_iter()
    }
}

/// Iterator for strange parts.
#[derive(Debug, Clone)]
pub struct StrangePartSetIterator {
    inner: std::array::IntoIter<Option<StrangePart>, STRANGE_PART_COUNT>,
}

impl Iterator for StrangePartSetIterator {
    type Item = StrangePart;

    fn next(&mut self) -> Option<Self::Item> {
        let iter = self.inner.by_ref();
        
        for opt in iter {
            if opt.is_some() {
                return opt;
            }
        }
        
        None
    }
}

impl fmt::Display for StrangePartSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut iter = self.into_iter();
        
        if let Some(first) = iter.next() {
            write!(f, "{first}")?;
            
            for s in iter {
                write!(f, ", {s}")?;
            }
        }
        
        Ok(())
    }
}

impl Serialize for StrangePartSet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let parts: Vec<_> = self.into_iter().collect();
        let mut seq = serializer.serialize_seq(Some(parts.len()))?;
        
        for (part, defindex) in parts.iter().zip(StrangePart::DEFINDEX.iter()) {
            seq.serialize_element(&SerializedAttribute {
                defindex: *defindex,
                value: None,
                float_value: part.attribute_float_value() ,
            })?;
        }

        seq.end()
    }
}

impl<'de> Deserialize<'de> for StrangePartSet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StrangePartSetVisitor;

        impl<'de> Visitor<'de> for StrangePartSetVisitor {
            type Value = StrangePartSet;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an array of maps with defindex, float_value")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<StrangePartSet, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut set = StrangePartSet::new();
                
                while let Some(map) = seq.next_element::<SerializedAttribute>()? {
                    if !<StrangePartSet as AttributeSet>::Item::DEFINDEX.contains(&map.defindex) {
                        // Skip if defindex is not for a score type
                        continue;
                    }
                    
                    let float_value = map.float_value
                        .ok_or_else(|| serde::de::Error::missing_field("float_value"))?;
                    let part = <StrangePartSet as AttributeSet>::Item::try_from_attribute_float_value(float_value)
                        .ok_or_else(|| serde::de::Error::custom("cannot convert from float_value"))?;
                    
                    set.insert(part);
                }
                
                Ok(set)
            }
        }
        
        deserializer.deserialize_seq(StrangePartSetVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Attributes;
    
    #[test]
    fn serializes() {
        let strange_parts = StrangePartSet::from([
            Some(StrangePart::TauntKills),
            Some(StrangePart::KillsWhileExplosiveJumping),
            Some(StrangePart::CriticalKills),
        ]);
        let json = serde_json::to_string(&strange_parts).unwrap();
        let raw = r#"[{"defindex":380,"float_value":77},{"defindex":382,"float_value":34},{"defindex":384,"float_value":33}]"#;
        
        assert_eq!(json, raw);
    }
    
    #[test]
    fn deserializes() {
        let raw = r#"[{"defindex":380,"float_value":77},{"defindex":382,"float_value":34},{"defindex":384,"float_value":33}]"#;
        let strange_parts: StrangePartSet = serde_json::from_str(raw).unwrap();
        let expected = StrangePartSet::from([
            Some(StrangePart::TauntKills),
            Some(StrangePart::KillsWhileExplosiveJumping),
            Some(StrangePart::CriticalKills),
        ]);

        assert_eq!(strange_parts, expected);
    }
    
    #[test]
    fn iterates_strange_parts() {
        let strange_parts = StrangePartSet::from([
            Some(StrangePart::TauntKills),
            Some(StrangePart::KillsWhileExplosiveJumping),
            Some(StrangePart::CriticalKills),
        ]);
        let mut iter = strange_parts.into_iter();
        
        assert_eq!(iter.next(), Some(StrangePart::TauntKills));
        assert_eq!(iter.next(), Some(StrangePart::KillsWhileExplosiveJumping));
        assert_eq!(iter.next(), Some(StrangePart::CriticalKills));
        assert_eq!(iter.next(), None);
        
        let mut count = 0;
        
        for _strange_part in &strange_parts {
            count += 1;
        }
        
        assert_eq!(count, 3);
    }
    
    #[test]
    fn mutates_strange_parts() {
        let mut strange_parts = StrangePartSet::from([
            Some(StrangePart::TauntKills),
            Some(StrangePart::KillsWhileExplosiveJumping),
            Some(StrangePart::CriticalKills),
        ]);
        
        assert_eq!(strange_parts.len(), 3);
        assert!(strange_parts.contains(&StrangePart::CriticalKills));
        
        strange_parts.remove(&StrangePart::CriticalKills);
        
        assert!(!strange_parts.contains(&StrangePart::CriticalKills));
        assert_eq!(strange_parts.len(), 2);
        
        strange_parts.insert(StrangePart::DamageDealt);
        
        assert!(strange_parts.contains(&StrangePart::DamageDealt));
        assert_eq!(strange_parts.len(), 3);
    }
    
    #[test]
    fn strange_parts_no_duplicates() {
        assert_eq!(StrangePartSet::from([
            Some(StrangePart::CriticalKills),
            Some(StrangePart::CriticalKills),
            Some(StrangePart::CriticalKills),
        ]), StrangePartSet::from([
            Some(StrangePart::CriticalKills),
            None,
            None,
        ]));
    }
    
    #[test]
    fn is_empty() {
        assert!(StrangePartSet::from([
            None,
            None,
            None,
        ]).is_empty());
    }
    
    #[test]
    fn iter_zip() {
        let strange_parts = StrangePartSet::from([
            Some(StrangePart::TauntKills),
            Some(StrangePart::KillsWhileExplosiveJumping),
            Some(StrangePart::CriticalKills),
        ]);
        let with_attribute_defindex = strange_parts.into_iter()
            .zip(StrangePart::DEFINDEX.to_owned())
            .collect::<Vec<_>>();
        
        assert_eq!(with_attribute_defindex, vec![
            (StrangePart::TauntKills, 380),
            (StrangePart::KillsWhileExplosiveJumping, 382),
            (StrangePart::CriticalKills, 384),
        ]);
    }
    
    #[test]
    fn stringify() {
        let strange_parts = StrangePartSet::from([
            Some(StrangePart::TauntKills),
            Some(StrangePart::KillsWhileExplosiveJumping),
            Some(StrangePart::CriticalKills),
        ]);
        
        assert_eq!(strange_parts.to_string(), "Taunt Kills, Kills While Explosive-Jumping, Critical Kills");
    }
    
    #[test]
    fn bit_and() {
        let set1 = StrangePartSet::from([
            Some(StrangePart::TauntKills),
            Some(StrangePart::KillsWhileExplosiveJumping),
            Some(StrangePart::CriticalKills),
        ]);
        let set2 = StrangePartSet::from([
            Some(StrangePart::TauntKills),
            Some(StrangePart::DamageDealt),
            None,
        ]);
        let intersection = set1 & set2;
        assert_eq!(intersection, StrangePartSet::from([
            Some(StrangePart::TauntKills),
            None,
            None,
        ]));
    }
    
    #[test]
    fn iterate_borrowed() {
        let strange_parts = StrangePartSet::from([
            Some(StrangePart::TauntKills),
            Some(StrangePart::KillsWhileExplosiveJumping),
            Some(StrangePart::CriticalKills),
        ]);
        
        for strange_part in &strange_parts {
            assert!(strange_parts.contains(&strange_part));
        }
    }
}
