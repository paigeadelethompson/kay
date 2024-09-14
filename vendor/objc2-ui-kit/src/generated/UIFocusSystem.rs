//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIFocusSystem;

    unsafe impl ClassType for UIFocusSystem {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UIFocusSystem {}

extern_methods!(
    unsafe impl UIFocusSystem {
        #[cfg(feature = "UIFocus")]
        #[method_id(@__retain_semantics Other focusedItem)]
        pub unsafe fn focusedItem(&self) -> Option<Retained<ProtocolObject<dyn UIFocusItem>>>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "UIFocus")]
        #[method_id(@__retain_semantics Other focusSystemForEnvironment:)]
        pub unsafe fn focusSystemForEnvironment(
            environment: &ProtocolObject<dyn UIFocusEnvironment>,
        ) -> Option<Retained<UIFocusSystem>>;

        #[cfg(feature = "UIFocus")]
        #[method(requestFocusUpdateToEnvironment:)]
        pub unsafe fn requestFocusUpdateToEnvironment(
            &self,
            environment: &ProtocolObject<dyn UIFocusEnvironment>,
        );

        #[method(updateFocusIfNeeded)]
        pub unsafe fn updateFocusIfNeeded(&self);

        #[cfg(feature = "UIFocus")]
        #[method(environment:containsEnvironment:)]
        pub unsafe fn environment_containsEnvironment(
            environment: &ProtocolObject<dyn UIFocusEnvironment>,
            other_environment: &ProtocolObject<dyn UIFocusEnvironment>,
        ) -> bool;

        #[cfg(feature = "UIFocus")]
        #[method(registerURL:forSoundIdentifier:)]
        pub unsafe fn registerURL_forSoundIdentifier(
            sound_file_url: &NSURL,
            identifier: &UIFocusSoundIdentifier,
            mtm: MainThreadMarker,
        );
    }
);

extern_methods!(
    /// UIFocusSystem
    #[cfg(all(
        feature = "UIResponder",
        feature = "UIScene",
        feature = "UIWindowScene"
    ))]
    unsafe impl UIWindowScene {
        #[method_id(@__retain_semantics Other focusSystem)]
        pub unsafe fn focusSystem(&self) -> Option<Retained<UIFocusSystem>>;
    }
);
