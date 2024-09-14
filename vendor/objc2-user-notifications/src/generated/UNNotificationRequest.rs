//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UNNotificationRequest;

    unsafe impl ClassType for UNNotificationRequest {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for UNNotificationRequest {}

unsafe impl NSCopying for UNNotificationRequest {}

unsafe impl NSObjectProtocol for UNNotificationRequest {}

unsafe impl NSSecureCoding for UNNotificationRequest {}

extern_methods!(
    unsafe impl UNNotificationRequest {
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Retained<NSString>;

        #[cfg(feature = "UNNotificationContent")]
        #[method_id(@__retain_semantics Other content)]
        pub unsafe fn content(&self) -> Retained<UNNotificationContent>;

        #[cfg(feature = "UNNotificationTrigger")]
        #[method_id(@__retain_semantics Other trigger)]
        pub unsafe fn trigger(&self) -> Option<Retained<UNNotificationTrigger>>;

        #[cfg(all(feature = "UNNotificationContent", feature = "UNNotificationTrigger"))]
        #[method_id(@__retain_semantics Other requestWithIdentifier:content:trigger:)]
        pub unsafe fn requestWithIdentifier_content_trigger(
            identifier: &NSString,
            content: &UNNotificationContent,
            trigger: Option<&UNNotificationTrigger>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UNNotificationRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
