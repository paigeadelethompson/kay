//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKQuery;

    unsafe impl ClassType for CKQuery {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for CKQuery {}

unsafe impl NSCopying for CKQuery {}

unsafe impl NSObjectProtocol for CKQuery {}

unsafe impl NSSecureCoding for CKQuery {}

extern_methods!(
    unsafe impl CKQuery {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, a_decoder: &NSCoder) -> Retained<Self>;

        #[cfg(feature = "CKRecord")]
        #[method_id(@__retain_semantics Init initWithRecordType:predicate:)]
        pub unsafe fn initWithRecordType_predicate(
            this: Allocated<Self>,
            record_type: &CKRecordType,
            predicate: &NSPredicate,
        ) -> Retained<Self>;

        #[cfg(feature = "CKRecord")]
        #[method_id(@__retain_semantics Other recordType)]
        pub unsafe fn recordType(&self) -> Retained<CKRecordType>;

        #[method_id(@__retain_semantics Other predicate)]
        pub unsafe fn predicate(&self) -> Retained<NSPredicate>;

        #[method_id(@__retain_semantics Other sortDescriptors)]
        pub unsafe fn sortDescriptors(&self) -> Option<Retained<NSArray<NSSortDescriptor>>>;

        #[method(setSortDescriptors:)]
        pub unsafe fn setSortDescriptors(
            &self,
            sort_descriptors: Option<&NSArray<NSSortDescriptor>>,
        );
    }
);
