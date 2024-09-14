//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIScreenshotService;

    unsafe impl ClassType for UIScreenshotService {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UIScreenshotService {}

extern_methods!(
    unsafe impl UIScreenshotService {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn UIScreenshotServiceDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn UIScreenshotServiceDelegate>>,
        );

        #[cfg(all(
            feature = "UIResponder",
            feature = "UIScene",
            feature = "UIWindowScene"
        ))]
        #[method_id(@__retain_semantics Other windowScene)]
        pub unsafe fn windowScene(&self) -> Option<Retained<UIWindowScene>>;
    }
);

extern_methods!(
    /// UIScreenshotService
    #[cfg(all(
        feature = "UIResponder",
        feature = "UIScene",
        feature = "UIWindowScene"
    ))]
    unsafe impl UIWindowScene {
        #[method_id(@__retain_semantics Other screenshotService)]
        pub unsafe fn screenshotService(&self) -> Option<Retained<UIScreenshotService>>;
    }
);

extern_protocol!(
    pub unsafe trait UIScreenshotServiceDelegate:
        NSObjectProtocol + IsMainThreadOnly
    {
        #[cfg(feature = "block2")]
        #[optional]
        #[method(screenshotService:generatePDFRepresentationWithCompletion:)]
        unsafe fn screenshotService_generatePDFRepresentationWithCompletion(
            &self,
            screenshot_service: &UIScreenshotService,
            completion_handler: &block2::Block<dyn Fn(*mut NSData, NSInteger, CGRect)>,
        );
    }

    unsafe impl ProtocolType for dyn UIScreenshotServiceDelegate {}
);
