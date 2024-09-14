//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[deprecated]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSPopoverAppearance(pub NSInteger);
impl NSPopoverAppearance {
    #[deprecated]
    #[doc(alias = "NSPopoverAppearanceMinimal")]
    pub const Minimal: Self = Self(0);
    #[deprecated]
    #[doc(alias = "NSPopoverAppearanceHUD")]
    pub const HUD: Self = Self(1);
}

unsafe impl Encode for NSPopoverAppearance {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSPopoverAppearance {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSPopoverBehavior(pub NSInteger);
impl NSPopoverBehavior {
    #[doc(alias = "NSPopoverBehaviorApplicationDefined")]
    pub const ApplicationDefined: Self = Self(0);
    #[doc(alias = "NSPopoverBehaviorTransient")]
    pub const Transient: Self = Self(1);
    #[doc(alias = "NSPopoverBehaviorSemitransient")]
    pub const Semitransient: Self = Self(2);
}

unsafe impl Encode for NSPopoverBehavior {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSPopoverBehavior {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSResponder")]
    pub struct NSPopover;

    #[cfg(feature = "NSResponder")]
    unsafe impl ClassType for NSPopover {
        #[inherits(NSObject)]
        type Super = NSResponder;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(all(feature = "NSAccessibilityProtocols", feature = "NSResponder"))]
unsafe impl NSAccessibility for NSPopover {}

#[cfg(all(feature = "NSAccessibilityProtocols", feature = "NSResponder"))]
unsafe impl NSAccessibilityElementProtocol for NSPopover {}

#[cfg(all(feature = "NSAppearance", feature = "NSResponder"))]
unsafe impl NSAppearanceCustomization for NSPopover {}

#[cfg(feature = "NSResponder")]
unsafe impl NSCoding for NSPopover {}

#[cfg(feature = "NSResponder")]
unsafe impl NSObjectProtocol for NSPopover {}

extern_methods!(
    #[cfg(feature = "NSResponder")]
    unsafe impl NSPopover {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Retained<ProtocolObject<dyn NSPopoverDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn NSPopoverDelegate>>);

        #[cfg(feature = "NSAppearance")]
        #[method_id(@__retain_semantics Other appearance)]
        pub unsafe fn appearance(&self) -> Option<Retained<NSAppearance>>;

        #[cfg(feature = "NSAppearance")]
        #[method(setAppearance:)]
        pub unsafe fn setAppearance(&self, appearance: Option<&NSAppearance>);

        #[cfg(feature = "NSAppearance")]
        #[method_id(@__retain_semantics Other effectiveAppearance)]
        pub unsafe fn effectiveAppearance(&self) -> Retained<NSAppearance>;

        #[method(behavior)]
        pub unsafe fn behavior(&self) -> NSPopoverBehavior;

        #[method(setBehavior:)]
        pub unsafe fn setBehavior(&self, behavior: NSPopoverBehavior);

        #[method(animates)]
        pub unsafe fn animates(&self) -> bool;

        #[method(setAnimates:)]
        pub unsafe fn setAnimates(&self, animates: bool);

        #[cfg(feature = "NSViewController")]
        #[method_id(@__retain_semantics Other contentViewController)]
        pub unsafe fn contentViewController(&self) -> Option<Retained<NSViewController>>;

        #[cfg(feature = "NSViewController")]
        #[method(setContentViewController:)]
        pub unsafe fn setContentViewController(
            &self,
            content_view_controller: Option<&NSViewController>,
        );

        #[method(contentSize)]
        pub unsafe fn contentSize(&self) -> NSSize;

        #[method(setContentSize:)]
        pub unsafe fn setContentSize(&self, content_size: NSSize);

        #[method(isShown)]
        pub unsafe fn isShown(&self) -> bool;

        #[method(isDetached)]
        pub unsafe fn isDetached(&self) -> bool;

        #[method(positioningRect)]
        pub unsafe fn positioningRect(&self) -> NSRect;

        #[method(setPositioningRect:)]
        pub unsafe fn setPositioningRect(&self, positioning_rect: NSRect);

        #[method(hasFullSizeContent)]
        pub unsafe fn hasFullSizeContent(&self) -> bool;

        #[method(setHasFullSizeContent:)]
        pub unsafe fn setHasFullSizeContent(&self, has_full_size_content: bool);

        #[cfg(feature = "NSView")]
        #[method(showRelativeToRect:ofView:preferredEdge:)]
        pub unsafe fn showRelativeToRect_ofView_preferredEdge(
            &self,
            positioning_rect: NSRect,
            positioning_view: &NSView,
            preferred_edge: NSRectEdge,
        );

        #[cfg(feature = "NSToolbarItem")]
        #[method(showRelativeToToolbarItem:)]
        pub unsafe fn showRelativeToToolbarItem(&self, toolbar_item: &NSToolbarItem);

        #[method(performClose:)]
        pub unsafe fn performClose(&self, sender: Option<&AnyObject>);

        #[method(close)]
        pub unsafe fn close(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSResponder")]
    unsafe impl NSPopover {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern "C" {
    pub static NSPopoverCloseReasonKey: &'static NSString;
}

// NS_TYPED_ENUM
pub type NSPopoverCloseReasonValue = NSString;

extern "C" {
    pub static NSPopoverCloseReasonStandard: &'static NSPopoverCloseReasonValue;
}

extern "C" {
    pub static NSPopoverCloseReasonDetachToWindow: &'static NSPopoverCloseReasonValue;
}

extern "C" {
    pub static NSPopoverWillShowNotification: &'static NSNotificationName;
}

extern "C" {
    pub static NSPopoverDidShowNotification: &'static NSNotificationName;
}

extern "C" {
    pub static NSPopoverWillCloseNotification: &'static NSNotificationName;
}

extern "C" {
    pub static NSPopoverDidCloseNotification: &'static NSNotificationName;
}

extern_protocol!(
    pub unsafe trait NSPopoverDelegate: NSObjectProtocol + IsMainThreadOnly {
        #[cfg(feature = "NSResponder")]
        #[optional]
        #[method(popoverShouldClose:)]
        unsafe fn popoverShouldClose(&self, popover: &NSPopover) -> bool;

        #[cfg(feature = "NSResponder")]
        #[optional]
        #[method(popoverShouldDetach:)]
        unsafe fn popoverShouldDetach(&self, popover: &NSPopover) -> bool;

        #[cfg(feature = "NSResponder")]
        #[optional]
        #[method(popoverDidDetach:)]
        unsafe fn popoverDidDetach(&self, popover: &NSPopover);

        #[cfg(all(feature = "NSResponder", feature = "NSWindow"))]
        #[optional]
        #[method_id(@__retain_semantics Other detachableWindowForPopover:)]
        unsafe fn detachableWindowForPopover(
            &self,
            popover: &NSPopover,
        ) -> Option<Retained<NSWindow>>;

        #[optional]
        #[method(popoverWillShow:)]
        unsafe fn popoverWillShow(&self, notification: &NSNotification);

        #[optional]
        #[method(popoverDidShow:)]
        unsafe fn popoverDidShow(&self, notification: &NSNotification);

        #[optional]
        #[method(popoverWillClose:)]
        unsafe fn popoverWillClose(&self, notification: &NSNotification);

        #[optional]
        #[method(popoverDidClose:)]
        unsafe fn popoverDidClose(&self, notification: &NSNotification);
    }

    unsafe impl ProtocolType for dyn NSPopoverDelegate {}
);
