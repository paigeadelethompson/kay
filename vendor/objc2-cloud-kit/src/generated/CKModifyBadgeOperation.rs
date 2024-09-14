//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CKOperation")]
    #[deprecated = "No longer supported, will cease working at some point in the future"]
    pub struct CKModifyBadgeOperation;

    #[cfg(feature = "CKOperation")]
    unsafe impl ClassType for CKModifyBadgeOperation {
        #[inherits(NSOperation, NSObject)]
        type Super = CKOperation;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CKOperation")]
unsafe impl NSObjectProtocol for CKModifyBadgeOperation {}

extern_methods!(
    #[cfg(feature = "CKOperation")]
    unsafe impl CKModifyBadgeOperation {
        #[deprecated = "No longer supported, will cease working at some point in the future"]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[deprecated = "No longer supported, will cease working at some point in the future"]
        #[method_id(@__retain_semantics Init initWithBadgeValue:)]
        pub unsafe fn initWithBadgeValue(
            this: Allocated<Self>,
            badge_value: NSUInteger,
        ) -> Retained<Self>;

        #[deprecated = "No longer supported, will cease working at some point in the future"]
        #[method(badgeValue)]
        pub unsafe fn badgeValue(&self) -> NSUInteger;

        #[deprecated = "No longer supported, will cease working at some point in the future"]
        #[method(setBadgeValue:)]
        pub unsafe fn setBadgeValue(&self, badge_value: NSUInteger);

        #[cfg(feature = "block2")]
        #[deprecated = "No longer supported, will cease working at some point in the future"]
        #[method(modifyBadgeCompletionBlock)]
        pub unsafe fn modifyBadgeCompletionBlock(&self)
            -> *mut block2::Block<dyn Fn(*mut NSError)>;

        #[cfg(feature = "block2")]
        #[deprecated = "No longer supported, will cease working at some point in the future"]
        #[method(setModifyBadgeCompletionBlock:)]
        pub unsafe fn setModifyBadgeCompletionBlock(
            &self,
            modify_badge_completion_block: Option<&block2::Block<dyn Fn(*mut NSError)>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CKOperation")]
    unsafe impl CKModifyBadgeOperation {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
