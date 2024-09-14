//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
use objc2_quartz_core::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIFocusEffect;

    unsafe impl ClassType for UIFocusEffect {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCopying for UIFocusEffect {}

unsafe impl NSObjectProtocol for UIFocusEffect {}

extern_methods!(
    unsafe impl UIFocusEffect {
        #[method_id(@__retain_semantics Other effect)]
        pub unsafe fn effect() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIFocusHaloEffectPosition(pub NSInteger);
impl UIFocusHaloEffectPosition {
    #[doc(alias = "UIFocusHaloEffectPositionAutomatic")]
    pub const Automatic: Self = Self(0);
    #[doc(alias = "UIFocusHaloEffectPositionOutside")]
    pub const Outside: Self = Self(1);
    #[doc(alias = "UIFocusHaloEffectPositionInside")]
    pub const Inside: Self = Self(2);
}

unsafe impl Encode for UIFocusHaloEffectPosition {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIFocusHaloEffectPosition {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIFocusHaloEffect;

    unsafe impl ClassType for UIFocusHaloEffect {
        #[inherits(NSObject)]
        type Super = UIFocusEffect;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCopying for UIFocusHaloEffect {}

unsafe impl NSObjectProtocol for UIFocusHaloEffect {}

extern_methods!(
    unsafe impl UIFocusHaloEffect {
        #[method_id(@__retain_semantics Other effectWithRect:)]
        pub unsafe fn effectWithRect(rect: CGRect) -> Retained<Self>;

        #[cfg(feature = "objc2-quartz-core")]
        #[cfg(not(target_os = "watchos"))]
        #[method_id(@__retain_semantics Other effectWithRoundedRect:cornerRadius:curve:)]
        pub unsafe fn effectWithRoundedRect_cornerRadius_curve(
            rect: CGRect,
            corner_radius: CGFloat,
            curve: &CALayerCornerCurve,
        ) -> Retained<Self>;

        #[cfg(feature = "UIBezierPath")]
        #[method_id(@__retain_semantics Other effectWithPath:)]
        pub unsafe fn effectWithPath(bezier_path: &UIBezierPath) -> Retained<Self>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method_id(@__retain_semantics Other containerView)]
        pub unsafe fn containerView(&self, mtm: MainThreadMarker) -> Option<Retained<UIView>>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method(setContainerView:)]
        pub unsafe fn setContainerView(&self, container_view: Option<&UIView>);

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method_id(@__retain_semantics Other referenceView)]
        pub unsafe fn referenceView(&self, mtm: MainThreadMarker) -> Option<Retained<UIView>>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method(setReferenceView:)]
        pub unsafe fn setReferenceView(&self, reference_view: Option<&UIView>);

        #[method(position)]
        pub unsafe fn position(&self) -> UIFocusHaloEffectPosition;

        #[method(setPosition:)]
        pub unsafe fn setPosition(&self, position: UIFocusHaloEffectPosition);
    }
);

extern_methods!(
    /// Methods declared on superclass `UIFocusEffect`
    unsafe impl UIFocusHaloEffect {
        #[method_id(@__retain_semantics Other effect)]
        pub unsafe fn effect() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
