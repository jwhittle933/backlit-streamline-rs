use crate::mp4::atom::{full_box, Info};

/// ISO BMFF Item Reference Box.
///
/// The item reference box allows the linking of one item to others via typed references. All the references
/// for one item of a specific type are collected into a single item type reference box, whose type is the
/// reference type, and which has a ‘from item ID’ field indicating which item is linked. The items linked to
/// are then represented by an array of ‘to item ID’s. All these single item type reference boxes are then
/// collected into the item reference box. The reference types defined for the track reference box defined in
/// 8.3.3 may be used here if appropriate, or other registered reference types. Version 1 of
/// ItemReferenceBox with SingleItemReferenceBoxLarge should only be used when large
/// from_item_ID or to_item_ID values (exceeding 65535) are required or expected to be required.
///
/// NOTE: This design makes it fairly easy to find all the references of a specific type, or from a specific item.
///
/// An item reference of type ‘font’ may be used to indicate that an item uses fonts carried/defined in
/// the referenced item.
#[full_box]
#[derive(Debug, Clone)]
pub struct Iref {
    pub info: Info,
    pub references: Vec<Reference>,
}

#[derive(Debug, Clone)]
pub enum Reference {
    /// Small is used for Version 0.
    Small(SingleItemTypeReference),
    /// Small is used for Version 1.
    Large(SingleItemTypeReferenceLarge),
}

#[derive(Debug, Clone)]
pub struct SingleItemTypeReference {
    pub info: Info,
    pub from_item_id: u16,
    /// reference_count is the number of references.
    pub reference_count: u16,
    pub to: Vec<To>,
}

#[derive(Debug, Clone)]
pub struct SingleItemTypeReferenceLarge {
    pub info: Info,
    pub from_item_id: u32,
    pub reference_count: u16,
    pub to: Vec<To>,
}

#[derive(Debug, Clone)]
pub struct To {
    /// to_item_id contains the ID of the item referred to.
    ///
    /// If Version 1, [`SingleItemTypeReferenceLarge`] is used
    /// and this field will be 32 bits. Otherwise, it'll be 16 bits.
    pub to_item_id: u32,
}
