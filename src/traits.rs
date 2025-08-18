use crate::{AttributeDef, AttributeValue, ItemAttribute};
use crate::error::InsertError;

/// Attribute values for an item attribute.
pub trait Attribute: Sized {
    /// The defindex.
    const DEFINDEX: u32;
    /// The attribute definition.
    const ATTRIBUTE: AttributeDef;
    /// Not part of the schema - this is a marker to specify which attribute field is meaningful to
    /// us in obtaining the attribute's value.
    /// 
    /// # Kill eaters example
    /// ```json
    /// {
    ///     "defindex": 214,
    ///     "value": 918,
    ///     "float_value": 1.28639199025018207e-42
    /// },
    /// ```
    /// This is the "kill_eater" attribute. "918" refers to the number of kills. The `float_value`
    /// field is the same number as a 32-bit float.
    /// 
    /// You can perform the conversions yourself with the following code:
    /// ```
    /// let value = 918u32;
    /// let float_value = 1.28639199025018207e-42f32;
    /// 
    /// assert_eq!(f32::from_bits(value), float_value);
    /// assert_eq!(float_value.to_bits(), value);
    /// ```
    /// 
    /// # Sheens example
    /// ```json
    /// {
    ///     "defindex": 2014,
    ///     "value": 1086324736,
    ///     "float_value": 6
    /// }
    /// ```
    /// This is the "killstreak_idleeffect" attribute. "6" refers to the associated sheen (
    /// [`crate::Sheen::VillainousViolet`]), but is stored in the `float_value` field, unlike
    /// "kill_eater". The `value` field is the same number as a 32-bit float.
    /// 
    /// While both values refer to the same value, and internally the attribute's value is its
    /// `float_value`, there are many cases where the `float_value` doesn't mean anything to us
    /// unless converted to a 32-bit integer from its bits, and if the `float_value` does mean
    /// something to us we don't want to convert it to an integer. This can be a little confusing,
    /// but it's just how the API is.
    /// 
    /// By marking each attribute with a `uses_float_value` flag, we can indicate whether the
    /// `float_value` field is meaningful to use for that attribute.
    const USES_FLOAT_VALUE: bool;
    
    /// Gets the attribute value.
    fn attribute_value(&self) -> AttributeValue {
        self.attribute_float_value()
            .map(|v| v.to_bits().into())
            .unwrap_or_default()
    }
    
    /// Gets the attribute float value.
    fn attribute_float_value(&self) -> Option<f32>;
}

/// Associated attribute values for a set of item attributes.
pub trait Attributes: Sized {
    /// The list of associated defindexes.
    const DEFINDEX: &[u32];
    /// The attribute definition.
    const ATTRIBUTES: &'static [AttributeDef];
    /// See [`Attribute::USES_FLOAT_VALUE`]. This applies to all attributes in the set.
    const USES_FLOAT_VALUE: bool;
    
    /// Gets the attribute value.
    fn attribute_value(&self) -> AttributeValue {
        self.attribute_float_value()
            .map(|v| v.to_bits().into())
            .unwrap_or_default()
    }
    
    /// Gets the attribute float value.
    fn attribute_float_value(&self) -> Option<f32> {
        None
    }
    
    /// Gets the attribute definition for a given defindex.
    fn get_attribute_def_by_defindex(defindex: u32) -> Option<&'static AttributeDef> {
        Self::ATTRIBUTES.iter().find(|attr| attr.defindex == defindex)
    }
}

/// Backwards conversion for attributes associated with an integer value.
pub trait TryFromAttributeValueU32: Sized + TryFrom<u32> {
    /// Attempts conversion from an attribute value.
    #[allow(unused_variables)]
    fn try_from_attribute_value(v: AttributeValue) -> Option<Self> {
        None
    }
    
    /// Attempts conversion from an attribute float value.
    fn try_from_attribute_float_value(v: f32) -> Option<Self> {
        if v.fract() != 0.0 || v.is_sign_negative() || v > (u32::MAX as f32) {
            return None;
        }
        
        Self::try_from(v as u32).ok()
    }
}

/// Definitions which are associated with colors.
pub trait Colored: Sized {
    /// Gets the color.
    fn color(&self) -> u32;
    
    /// Attempts to convert a hexadecimal color.
    fn from_color(color: u32) -> Option<Self>;
    
    /// Converts this into a hexademical color string in the format "#FFFFFF".
    fn color_string(&self) -> String {
        format!("#{:06X}", self.color())
    }
    
    /// Attempts to convert a hexadecimal color string.
    fn from_color_str(color: &str) -> Option<Self> {
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
}

/// Definitions which are associated with an item defindex.
pub trait HasItemDefindex: Sized {
    /// Gets the `defindex`.
    fn defindex(&self) -> u32;
    
    /// Converts a `defindex` into its related item, if it exists.
    fn from_defindex(defindex: u32) -> Option<Self>;
}

/// A fixed set of attributes.
pub trait AttributeSet: Sized + Default {
    /// Max number of items.
    const MAX_COUNT: usize;
    /// The item type.
    type Item: PartialEq + Copy + Attributes;
    /// An empty set.
    const NONE: Self;
    
    /// Adds an item to the first available slot. Returns `false` if the set is full or already
    /// contains the value.
    fn insert(&mut self, item: Self::Item) -> bool;
    
    /// Same as `insert`, but returns a [`std::result::Result`] with descriptive error to identify
    /// why the insert failed.
    fn try_insert(&mut self, item: Self::Item) -> Result<(), InsertError>;
    
    /// Adds an item to the first available slot. Replaces the last item in the set if the set is
    /// full. Returns `false` if the set already contains the value.
    fn insert_or_replace_last(&mut self, item: Self::Item) -> bool;
    
    /// Removes an item from the set. Returns whether the value was present in the set.
    fn remove(&mut self, item: &Self::Item) -> bool;
    
    /// Removes and returns the item in the set, if any, that is equal to the given one.
    fn take(&mut self, item: &Self::Item) -> Option<Self::Item>;
    
    /// Replaces an item in the set with a new item. `false` if the item was not present.
    fn replace(&mut self, item: &Self::Item, new_item: Self::Item) -> bool;
    
    /// Clears the set.
    fn clear(&mut self);
    
    /// Gets an item from the set by index.
    fn get(&self, index: usize) -> Option<&Self::Item> {
        self.as_slice().get(index).and_then(|opt| opt.as_ref())
    }
    
    /// Returns the number of elements in the set.
    fn len(&self) -> usize {
        // The sets are small so iteration is fine.
        self.as_slice()
            .iter()
            .filter(|x| x.is_some())
            .count()
    }

    /// Returns true if the set contains the given item.
    fn contains(&self, item: &Self::Item) -> bool {
        self.as_slice().contains(&Some(*item))
    }
    
    /// Returns true if the set is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    
    /// Returns true if the set is full, i.e., it contains the maximum number of elements.
    fn is_full(&self) -> bool {
        self.len() == Self::MAX_COUNT
    }
    
    /// Gets the capacity of the set.
    fn capacity(&self) -> usize {
        Self::MAX_COUNT
    }
    
    /// Gets the first item in the set.
    fn first(&self) -> Option<&Self::Item> {
        self.iter().next()
    }
    
    /// Gets the last item in the set.
    fn last(&self) -> Option<&Self::Item> {
        self.iter().last()
    }
    
    /// Returns the items that are in `self` but not in `other`.
    fn difference(&self, other: &Self) -> Self {
        let mut result = Self::default();
        
        for s in self.iter() {
            if !other.contains(s) {
                result.insert(*s);
            }
        }
        
        result
    }
    
    /// Returns the items that are both in `self` and `other`.
    fn intersection(&self, other: &Self) -> Self {
        let mut result = Self::default();
        
        for s in self.iter() {
            if other.contains(s) {
                result.insert(*s);
            }
        }

        result
    }
    
    /// Returns `true` if `self` has no items in common with `other`. This is equivalent to 
    /// checking for an empty intersection.
    fn is_disjoint(&self, other: &Self) -> bool {
        self.intersection(other).is_empty()
    }
    
    /// Returns true if the set is a subset of another, i.e., other contains at least all the
    /// values in self.
    fn is_subset(&self, other: &Self) -> bool {
        if self.len() > other.len() {
            return false;
        }
        
        self.iter().all(|spell| other.contains(spell))
    }
    
    /// Returns true if the set is a superset of another, i.e., self contains at least all the
    /// values in other.
    fn is_superset(&self, other: &Self) -> bool {
        other.is_subset(self)
    }
    
    /// Returns an iterator over the set.
    #[inline(always)]
    fn iter(&self) -> impl Iterator<Item = &Self::Item> {
        self.as_slice().iter().filter_map(|opt| opt.as_ref())
    }
    
    /// Returns a mutable iterator over the set.
    fn iter_mut(&mut self) -> impl Iterator<Item = &mut Self::Item> {
        self.as_mut_slice().iter_mut().filter_map(|opt| opt.as_mut())
    }
    
    /// Retains only the items specified by the predicate.
    fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&Self::Item) -> bool,
    {
        for slot in self.as_mut_slice() {
            if let Some(ref item) = slot {
                if !f(item) {
                    *slot = None;
                }
            }
        }
    }
    
    /// Extends items from an iterator into the set.
    fn extend<I: IntoIterator<Item = Self::Item>>(&mut self, iter: I) {
        for item in iter {
            self.insert(item);
        }
    }
    
    /// Converts each element to an [`ItemAttribute`]. 
    fn iter_attributes(&self) -> impl Iterator<Item = ItemAttribute>;
    
    /// Returns the inner storage as a slice.
    fn as_slice(&self) -> &[Option<Self::Item>];
    
    /// Returns the inner storage as a mutable slice.
    fn as_mut_slice(&mut self) -> &mut [Option<Self::Item>];
}
