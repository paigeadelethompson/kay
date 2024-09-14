//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSPersistentStoreRequest")]
    pub struct NSBatchUpdateRequest;

    #[cfg(feature = "NSPersistentStoreRequest")]
    unsafe impl ClassType for NSBatchUpdateRequest {
        #[inherits(NSObject)]
        type Super = NSPersistentStoreRequest;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "NSPersistentStoreRequest")]
unsafe impl NSCopying for NSBatchUpdateRequest {}

#[cfg(feature = "NSPersistentStoreRequest")]
unsafe impl NSObjectProtocol for NSBatchUpdateRequest {}

extern_methods!(
    #[cfg(feature = "NSPersistentStoreRequest")]
    unsafe impl NSBatchUpdateRequest {
        #[method_id(@__retain_semantics Other batchUpdateRequestWithEntityName:)]
        pub unsafe fn batchUpdateRequestWithEntityName(entity_name: &NSString) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithEntityName:)]
        pub unsafe fn initWithEntityName(
            this: Allocated<Self>,
            entity_name: &NSString,
        ) -> Retained<Self>;

        #[cfg(feature = "NSEntityDescription")]
        #[method_id(@__retain_semantics Init initWithEntity:)]
        pub unsafe fn initWithEntity(
            this: Allocated<Self>,
            entity: &NSEntityDescription,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other entityName)]
        pub unsafe fn entityName(&self) -> Retained<NSString>;

        #[cfg(feature = "NSEntityDescription")]
        #[method_id(@__retain_semantics Other entity)]
        pub unsafe fn entity(&self) -> Retained<NSEntityDescription>;

        #[method_id(@__retain_semantics Other predicate)]
        pub unsafe fn predicate(&self) -> Option<Retained<NSPredicate>>;

        #[method(setPredicate:)]
        pub unsafe fn setPredicate(&self, predicate: Option<&NSPredicate>);

        #[method(includesSubentities)]
        pub unsafe fn includesSubentities(&self) -> bool;

        #[method(setIncludesSubentities:)]
        pub unsafe fn setIncludesSubentities(&self, includes_subentities: bool);

        #[cfg(feature = "NSPersistentStoreResult")]
        #[method(resultType)]
        pub unsafe fn resultType(&self) -> NSBatchUpdateRequestResultType;

        #[cfg(feature = "NSPersistentStoreResult")]
        #[method(setResultType:)]
        pub unsafe fn setResultType(&self, result_type: NSBatchUpdateRequestResultType);

        #[method_id(@__retain_semantics Other propertiesToUpdate)]
        pub unsafe fn propertiesToUpdate(&self) -> Option<Retained<NSDictionary>>;

        #[method(setPropertiesToUpdate:)]
        pub unsafe fn setPropertiesToUpdate(&self, properties_to_update: Option<&NSDictionary>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSPersistentStoreRequest")]
    unsafe impl NSBatchUpdateRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
