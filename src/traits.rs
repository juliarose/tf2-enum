use crate::{AttributeValue, AttributeDef};

/// Attribute values for an item attribute.
pub trait Attribute: Sized {
    /// The defindex.
    const DEFINDEX: u32;
    /// The attribute definition.
    const ATTRIBUTE: AttributeDef;
    
    /// Gets the attribute value.
    fn attribute_value(&self) -> Option<AttributeValue>;
    
    /// Gets the attribute float value.
    fn attribute_float_value(&self) -> Option<f64>;
}

/// Associated attribute values for a set of item attributes.
pub trait Attributes: Sized {
    /// The list of associated defindexes.
    const DEFINDEX: &[u32];
    /// The attribute definition.
    const ATTRIBUTES: &'static [AttributeDef];
    
    /// Gets the attribute value.
    fn attribute_value(&self) -> Option<AttributeValue> {
        None
    }
    
    /// Gets the attribute float value.
    fn attribute_float_value(&self) -> Option<f64> {
        None
    }
    
    fn by_defindex(&self, defindex: u32) -> Option<&AttributeDef> {
        Self::ATTRIBUTES.iter().find(|attr| attr.defindex == defindex)
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
pub trait ItemDefindex: Sized {
    /// Gets the `defindex`.
    fn defindex(&self) -> u32;
    
    /// Converts a `defindex` into its related item, if it exists.
    fn from_defindex(defindex: u32) -> Option<Self>;
}

pub trait AttributeSet: Sized + Default {
    /// Max number of items.
    const MAX_COUNT: usize;
    /// The item type.
    type Item: PartialEq + Copy;
    // An empty set.
    const NONE: Self;
    
    /// Adds an item to the first available slot.
    fn insert(&mut self, item: Self::Item) -> bool;
    
    /// Removes an item from the set. Returns whether the value was present in the set.
    fn remove(&mut self, item: &Self::Item) -> bool;
    
    /// Removes and returns the item in the set, if any, that is equal to the given one.
    fn take(&mut self, item: &Self::Item) -> Option<Self::Item>;
    
    /// Clears the set,.
    fn clear(&mut self);
    
    /// Gets an item from the set by index.
    fn get(&self, index: usize) -> Option<&Self::Item> {
        self.as_slice().get(index).and_then(|opt| opt.as_ref())
    }
    
    /// Returns the number of elements in the set.
    fn len(&self) -> usize {
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
    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a Self::Item> {
        self.as_slice().iter().filter_map(|opt| opt.as_ref())
    }
    
    /// Returns a mutable iterator over the set.
    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut Self::Item> {
        self.as_mut_slice().iter_mut().filter_map(|opt| opt.as_mut())
    }
    
    /// Returns the inner storage as a slice.
    fn as_slice(&self) -> &[Option<Self::Item>];
    
    /// Returns the inner storage as a mutable slice.
    fn as_mut_slice(&mut self) -> &mut [Option<Self::Item>];
}
