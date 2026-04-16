use syn::{Attribute, Field, FnArg, ItemFn, ItemMod, ItemStruct, ItemUse};

/// Helper trait for all syn nodes that expose `attrs: Vec<Attribute>`
pub trait HasAttributes {
    fn attrs(&self) -> &[Attribute];
}

/// Public trait for attribute checking
pub trait AttributeCheck {
    fn has_attr(&self, name: &str) -> bool;
}

/* -------------------------------------------------------
 * Generic AttributeCheck implementation
 * -------------------------------------------------------*/
impl<T: HasAttributes> AttributeCheck for T {
    fn has_attr(&self, name: &str) -> bool {
        self.attrs().iter().any(|attr| {
            match attr.path().segments.last() {
                Some(seg) => seg.ident == name,
                None => false,
            }
        })
    }
}

/* -------------------------------------------------------
 * HasAttributes implementations for syn types
 * -------------------------------------------------------*/

impl HasAttributes for ItemFn {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}

impl HasAttributes for Field {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}

impl HasAttributes for FnArg {
    fn attrs(&self) -> &[Attribute] {
        match self {
            FnArg::Typed(pat_type) => &pat_type.attrs,
            FnArg::Receiver(receiver) => &receiver.attrs,
        }
    }
}

impl HasAttributes for ItemStruct {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}

impl HasAttributes for ItemMod {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}

impl HasAttributes for ItemUse {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}
