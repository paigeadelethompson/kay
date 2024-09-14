//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSDeleteRule(pub NSUInteger);
impl NSDeleteRule {
    pub const NSNoActionDeleteRule: Self = Self(0);
    pub const NSNullifyDeleteRule: Self = Self(1);
    pub const NSCascadeDeleteRule: Self = Self(2);
    pub const NSDenyDeleteRule: Self = Self(3);
}

unsafe impl Encode for NSDeleteRule {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSDeleteRule {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSPropertyDescription")]
    pub struct NSRelationshipDescription;

    #[cfg(feature = "NSPropertyDescription")]
    unsafe impl ClassType for NSRelationshipDescription {
        #[inherits(NSObject)]
        type Super = NSPropertyDescription;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "NSPropertyDescription")]
unsafe impl NSCoding for NSRelationshipDescription {}

#[cfg(feature = "NSPropertyDescription")]
unsafe impl NSCopying for NSRelationshipDescription {}

#[cfg(feature = "NSPropertyDescription")]
unsafe impl NSObjectProtocol for NSRelationshipDescription {}

extern_methods!(
    #[cfg(feature = "NSPropertyDescription")]
    unsafe impl NSRelationshipDescription {
        #[cfg(feature = "NSEntityDescription")]
        #[method_id(@__retain_semantics Other destinationEntity)]
        pub unsafe fn destinationEntity(&self) -> Option<Retained<NSEntityDescription>>;

        #[cfg(feature = "NSEntityDescription")]
        #[method(setDestinationEntity:)]
        pub unsafe fn setDestinationEntity(&self, destination_entity: Option<&NSEntityDescription>);

        #[method_id(@__retain_semantics Other inverseRelationship)]
        pub unsafe fn inverseRelationship(&self) -> Option<Retained<NSRelationshipDescription>>;

        #[method(setInverseRelationship:)]
        pub unsafe fn setInverseRelationship(
            &self,
            inverse_relationship: Option<&NSRelationshipDescription>,
        );

        #[method(maxCount)]
        pub unsafe fn maxCount(&self) -> NSUInteger;

        #[method(setMaxCount:)]
        pub unsafe fn setMaxCount(&self, max_count: NSUInteger);

        #[method(minCount)]
        pub unsafe fn minCount(&self) -> NSUInteger;

        #[method(setMinCount:)]
        pub unsafe fn setMinCount(&self, min_count: NSUInteger);

        #[method(deleteRule)]
        pub unsafe fn deleteRule(&self) -> NSDeleteRule;

        #[method(setDeleteRule:)]
        pub unsafe fn setDeleteRule(&self, delete_rule: NSDeleteRule);

        #[method(isToMany)]
        pub unsafe fn isToMany(&self) -> bool;

        #[method_id(@__retain_semantics Other versionHash)]
        pub unsafe fn versionHash(&self) -> Retained<NSData>;

        #[method(isOrdered)]
        pub unsafe fn isOrdered(&self) -> bool;

        #[method(setOrdered:)]
        pub unsafe fn setOrdered(&self, ordered: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSPropertyDescription")]
    unsafe impl NSRelationshipDescription {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
