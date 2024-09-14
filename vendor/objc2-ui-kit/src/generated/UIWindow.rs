//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
use objc2_quartz_core::*;

use crate::*;

// NS_TYPED_EXTENSIBLE_ENUM
pub type UIWindowLevel = CGFloat;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    pub struct UIWindow;

    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl ClassType for UIWindow {
        #[inherits(UIResponder, NSObject)]
        type Super = UIView;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(all(
    feature = "UIResponder",
    feature = "UIView",
    feature = "objc2-quartz-core"
))]
#[cfg(not(target_os = "watchos"))]
unsafe impl CALayerDelegate for UIWindow {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl NSCoding for UIWindow {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl NSObjectProtocol for UIWindow {}

#[cfg(all(feature = "UIAppearance", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIAppearance for UIWindow {}

#[cfg(all(feature = "UIAppearance", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIAppearanceContainer for UIWindow {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl UICoordinateSpace for UIWindow {}

#[cfg(all(
    feature = "UIDynamicBehavior",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIDynamicItem for UIWindow {}

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIFocusEnvironment for UIWindow {}

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIFocusItem for UIWindow {}

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIFocusItemContainer for UIWindow {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl UIResponderStandardEditActions for UIWindow {}

#[cfg(all(
    feature = "UIResponder",
    feature = "UITraitCollection",
    feature = "UIView"
))]
unsafe impl UITraitEnvironment for UIWindow {}

extern_methods!(
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl UIWindow {
        #[cfg(all(feature = "UIScene", feature = "UIWindowScene"))]
        #[method_id(@__retain_semantics Init initWithWindowScene:)]
        pub unsafe fn initWithWindowScene(
            this: Allocated<Self>,
            window_scene: &UIWindowScene,
        ) -> Retained<Self>;

        #[cfg(all(feature = "UIScene", feature = "UIWindowScene"))]
        #[method_id(@__retain_semantics Other windowScene)]
        pub unsafe fn windowScene(&self) -> Option<Retained<UIWindowScene>>;

        #[cfg(all(feature = "UIScene", feature = "UIWindowScene"))]
        #[method(setWindowScene:)]
        pub unsafe fn setWindowScene(&self, window_scene: Option<&UIWindowScene>);

        #[method(canResizeToFitContent)]
        pub unsafe fn canResizeToFitContent(&self) -> bool;

        #[method(setCanResizeToFitContent:)]
        pub unsafe fn setCanResizeToFitContent(&self, can_resize_to_fit_content: bool);

        #[cfg(feature = "UIScreen")]
        #[method_id(@__retain_semantics Other screen)]
        pub fn screen(&self) -> Retained<UIScreen>;

        #[cfg(feature = "UIScreen")]
        #[method(setScreen:)]
        pub fn setScreen(&self, screen: &UIScreen);

        #[method(windowLevel)]
        pub unsafe fn windowLevel(&self) -> UIWindowLevel;

        #[method(setWindowLevel:)]
        pub unsafe fn setWindowLevel(&self, window_level: UIWindowLevel);

        #[method(isKeyWindow)]
        pub fn isKeyWindow(&self) -> bool;

        #[method(canBecomeKeyWindow)]
        pub unsafe fn canBecomeKeyWindow(&self) -> bool;

        #[method(becomeKeyWindow)]
        pub unsafe fn becomeKeyWindow(&self);

        #[method(resignKeyWindow)]
        pub unsafe fn resignKeyWindow(&self);

        #[method(makeKeyWindow)]
        pub unsafe fn makeKeyWindow(&self);

        #[method(makeKeyAndVisible)]
        pub fn makeKeyAndVisible(&self);

        #[cfg(feature = "UIViewController")]
        #[method_id(@__retain_semantics Other rootViewController)]
        pub fn rootViewController(&self) -> Option<Retained<UIViewController>>;

        #[cfg(feature = "UIViewController")]
        #[method(setRootViewController:)]
        pub fn setRootViewController(&self, root_view_controller: Option<&UIViewController>);

        #[cfg(feature = "UIEvent")]
        #[method(sendEvent:)]
        pub unsafe fn sendEvent(&self, event: &UIEvent);

        #[method(convertPoint:toWindow:)]
        pub unsafe fn convertPoint_toWindow(
            &self,
            point: CGPoint,
            window: Option<&UIWindow>,
        ) -> CGPoint;

        #[method(convertPoint:fromWindow:)]
        pub unsafe fn convertPoint_fromWindow(
            &self,
            point: CGPoint,
            window: Option<&UIWindow>,
        ) -> CGPoint;

        #[method(convertRect:toWindow:)]
        pub unsafe fn convertRect_toWindow(
            &self,
            rect: CGRect,
            window: Option<&UIWindow>,
        ) -> CGRect;

        #[method(convertRect:fromWindow:)]
        pub unsafe fn convertRect_fromWindow(
            &self,
            rect: CGRect,
            window: Option<&UIWindow>,
        ) -> CGRect;
    }
);

extern_methods!(
    /// Methods declared on superclass `UIView`
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl UIWindow {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame: CGRect) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl UIWindow {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_protocol!(
    pub unsafe trait UILayoutGuideAspectFitting: NSObjectProtocol {
        #[method(aspectRatio)]
        unsafe fn aspectRatio(&self) -> CGFloat;

        #[method(setAspectRatio:)]
        unsafe fn setAspectRatio(&self, aspect_ratio: CGFloat);
    }

    unsafe impl ProtocolType for dyn UILayoutGuideAspectFitting {}
);

extern_methods!(
    /// UIWindowLayout
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl UIWindow {
        #[cfg(feature = "UILayoutGuide")]
        #[method_id(@__retain_semantics Other safeAreaAspectFitLayoutGuide)]
        pub unsafe fn safeAreaAspectFitLayoutGuide(&self) -> Retained<UILayoutGuide>;
    }
);

extern "C" {
    pub static UIWindowLevelNormal: UIWindowLevel;
}

extern "C" {
    pub static UIWindowLevelAlert: UIWindowLevel;
}

extern "C" {
    pub static UIWindowLevelStatusBar: UIWindowLevel;
}

extern "C" {
    pub static UIWindowDidBecomeVisibleNotification: &'static NSNotificationName;
}

extern "C" {
    pub static UIWindowDidBecomeHiddenNotification: &'static NSNotificationName;
}

extern "C" {
    pub static UIWindowDidBecomeKeyNotification: &'static NSNotificationName;
}

extern "C" {
    pub static UIWindowDidResignKeyNotification: &'static NSNotificationName;
}

extern "C" {
    pub static UIKeyboardWillShowNotification: &'static NSNotificationName;
}

extern "C" {
    pub static UIKeyboardDidShowNotification: &'static NSNotificationName;
}

extern "C" {
    pub static UIKeyboardWillHideNotification: &'static NSNotificationName;
}

extern "C" {
    pub static UIKeyboardDidHideNotification: &'static NSNotificationName;
}

extern "C" {
    pub static UIKeyboardFrameBeginUserInfoKey: &'static NSString;
}

extern "C" {
    pub static UIKeyboardFrameEndUserInfoKey: &'static NSString;
}

extern "C" {
    pub static UIKeyboardAnimationDurationUserInfoKey: &'static NSString;
}

extern "C" {
    pub static UIKeyboardAnimationCurveUserInfoKey: &'static NSString;
}

extern "C" {
    pub static UIKeyboardIsLocalUserInfoKey: &'static NSString;
}

extern "C" {
    pub static UIKeyboardWillChangeFrameNotification: &'static NSNotificationName;
}

extern "C" {
    pub static UIKeyboardDidChangeFrameNotification: &'static NSNotificationName;
}

extern "C" {
    pub static UIKeyboardCenterBeginUserInfoKey: &'static NSString;
}

extern "C" {
    pub static UIKeyboardCenterEndUserInfoKey: &'static NSString;
}

extern "C" {
    pub static UIKeyboardBoundsUserInfoKey: &'static NSString;
}
