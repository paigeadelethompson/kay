//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    pub unsafe trait UITraitDefinition: IsMainThreadOnly {
        #[optional]
        #[method_id(@__retain_semantics Other identifier)]
        unsafe fn identifier(mtm: MainThreadMarker) -> Retained<NSString>;

        #[optional]
        #[method_id(@__retain_semantics Other name)]
        unsafe fn name(mtm: MainThreadMarker) -> Retained<NSString>;

        #[optional]
        #[method(affectsColorAppearance)]
        unsafe fn affectsColorAppearance(mtm: MainThreadMarker) -> bool;
    }

    unsafe impl ProtocolType for dyn UITraitDefinition {}
);

pub type UITrait = *mut AnyClass;

extern_protocol!(
    pub unsafe trait UICGFloatTraitDefinition: UITraitDefinition + IsMainThreadOnly {
        #[method(defaultValue)]
        unsafe fn defaultValue(mtm: MainThreadMarker) -> CGFloat;
    }

    unsafe impl ProtocolType for dyn UICGFloatTraitDefinition {}
);

pub type UICGFloatTrait = *mut AnyClass;

extern_protocol!(
    pub unsafe trait UINSIntegerTraitDefinition:
        UITraitDefinition + IsMainThreadOnly
    {
        #[method(defaultValue)]
        unsafe fn defaultValue(mtm: MainThreadMarker) -> NSInteger;
    }

    unsafe impl ProtocolType for dyn UINSIntegerTraitDefinition {}
);

pub type UINSIntegerTrait = *mut AnyClass;

extern_protocol!(
    pub unsafe trait UIObjectTraitDefinition: UITraitDefinition + IsMainThreadOnly {
        #[method_id(@__retain_semantics Other defaultValue)]
        unsafe fn defaultValue(mtm: MainThreadMarker) -> Option<Retained<NSObject>>;
    }

    unsafe impl ProtocolType for dyn UIObjectTraitDefinition {}
);

pub type UIObjectTrait = *mut AnyClass;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitUserInterfaceIdiom;

    unsafe impl ClassType for UITraitUserInterfaceIdiom {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UITraitUserInterfaceIdiom {}

unsafe impl UINSIntegerTraitDefinition for UITraitUserInterfaceIdiom {}

unsafe impl UITraitDefinition for UITraitUserInterfaceIdiom {}

extern_methods!(
    unsafe impl UITraitUserInterfaceIdiom {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITraitUserInterfaceIdiom {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitUserInterfaceStyle;

    unsafe impl ClassType for UITraitUserInterfaceStyle {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UITraitUserInterfaceStyle {}

unsafe impl UINSIntegerTraitDefinition for UITraitUserInterfaceStyle {}

unsafe impl UITraitDefinition for UITraitUserInterfaceStyle {}

extern_methods!(
    unsafe impl UITraitUserInterfaceStyle {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITraitUserInterfaceStyle {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitLayoutDirection;

    unsafe impl ClassType for UITraitLayoutDirection {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UITraitLayoutDirection {}

unsafe impl UINSIntegerTraitDefinition for UITraitLayoutDirection {}

unsafe impl UITraitDefinition for UITraitLayoutDirection {}

extern_methods!(
    unsafe impl UITraitLayoutDirection {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITraitLayoutDirection {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitDisplayScale;

    unsafe impl ClassType for UITraitDisplayScale {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UITraitDisplayScale {}

unsafe impl UICGFloatTraitDefinition for UITraitDisplayScale {}

unsafe impl UITraitDefinition for UITraitDisplayScale {}

extern_methods!(
    unsafe impl UITraitDisplayScale {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITraitDisplayScale {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitHorizontalSizeClass;

    unsafe impl ClassType for UITraitHorizontalSizeClass {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UITraitHorizontalSizeClass {}

unsafe impl UINSIntegerTraitDefinition for UITraitHorizontalSizeClass {}

unsafe impl UITraitDefinition for UITraitHorizontalSizeClass {}

extern_methods!(
    unsafe impl UITraitHorizontalSizeClass {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITraitHorizontalSizeClass {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitVerticalSizeClass;

    unsafe impl ClassType for UITraitVerticalSizeClass {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UITraitVerticalSizeClass {}

unsafe impl UINSIntegerTraitDefinition for UITraitVerticalSizeClass {}

unsafe impl UITraitDefinition for UITraitVerticalSizeClass {}

extern_methods!(
    unsafe impl UITraitVerticalSizeClass {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITraitVerticalSizeClass {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitForceTouchCapability;

    unsafe impl ClassType for UITraitForceTouchCapability {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UITraitForceTouchCapability {}

unsafe impl UINSIntegerTraitDefinition for UITraitForceTouchCapability {}

unsafe impl UITraitDefinition for UITraitForceTouchCapability {}

extern_methods!(
    unsafe impl UITraitForceTouchCapability {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITraitForceTouchCapability {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitPreferredContentSizeCategory;

    unsafe impl ClassType for UITraitPreferredContentSizeCategory {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UITraitPreferredContentSizeCategory {}

unsafe impl UIObjectTraitDefinition for UITraitPreferredContentSizeCategory {}

unsafe impl UITraitDefinition for UITraitPreferredContentSizeCategory {}

extern_methods!(
    unsafe impl UITraitPreferredContentSizeCategory {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITraitPreferredContentSizeCategory {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitDisplayGamut;

    unsafe impl ClassType for UITraitDisplayGamut {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UITraitDisplayGamut {}

unsafe impl UINSIntegerTraitDefinition for UITraitDisplayGamut {}

unsafe impl UITraitDefinition for UITraitDisplayGamut {}

extern_methods!(
    unsafe impl UITraitDisplayGamut {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITraitDisplayGamut {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitAccessibilityContrast;

    unsafe impl ClassType for UITraitAccessibilityContrast {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UITraitAccessibilityContrast {}

unsafe impl UINSIntegerTraitDefinition for UITraitAccessibilityContrast {}

unsafe impl UITraitDefinition for UITraitAccessibilityContrast {}

extern_methods!(
    unsafe impl UITraitAccessibilityContrast {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITraitAccessibilityContrast {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitUserInterfaceLevel;

    unsafe impl ClassType for UITraitUserInterfaceLevel {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UITraitUserInterfaceLevel {}

unsafe impl UINSIntegerTraitDefinition for UITraitUserInterfaceLevel {}

unsafe impl UITraitDefinition for UITraitUserInterfaceLevel {}

extern_methods!(
    unsafe impl UITraitUserInterfaceLevel {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITraitUserInterfaceLevel {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitLegibilityWeight;

    unsafe impl ClassType for UITraitLegibilityWeight {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UITraitLegibilityWeight {}

unsafe impl UINSIntegerTraitDefinition for UITraitLegibilityWeight {}

unsafe impl UITraitDefinition for UITraitLegibilityWeight {}

extern_methods!(
    unsafe impl UITraitLegibilityWeight {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITraitLegibilityWeight {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitActiveAppearance;

    unsafe impl ClassType for UITraitActiveAppearance {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UITraitActiveAppearance {}

unsafe impl UINSIntegerTraitDefinition for UITraitActiveAppearance {}

unsafe impl UITraitDefinition for UITraitActiveAppearance {}

extern_methods!(
    unsafe impl UITraitActiveAppearance {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITraitActiveAppearance {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitToolbarItemPresentationSize;

    unsafe impl ClassType for UITraitToolbarItemPresentationSize {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UITraitToolbarItemPresentationSize {}

unsafe impl UINSIntegerTraitDefinition for UITraitToolbarItemPresentationSize {}

unsafe impl UITraitDefinition for UITraitToolbarItemPresentationSize {}

extern_methods!(
    unsafe impl UITraitToolbarItemPresentationSize {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITraitToolbarItemPresentationSize {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitImageDynamicRange;

    unsafe impl ClassType for UITraitImageDynamicRange {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UITraitImageDynamicRange {}

unsafe impl UINSIntegerTraitDefinition for UITraitImageDynamicRange {}

unsafe impl UITraitDefinition for UITraitImageDynamicRange {}

extern_methods!(
    unsafe impl UITraitImageDynamicRange {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITraitImageDynamicRange {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitTypesettingLanguage;

    unsafe impl ClassType for UITraitTypesettingLanguage {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UITraitTypesettingLanguage {}

unsafe impl UIObjectTraitDefinition for UITraitTypesettingLanguage {}

unsafe impl UITraitDefinition for UITraitTypesettingLanguage {}

extern_methods!(
    unsafe impl UITraitTypesettingLanguage {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITraitTypesettingLanguage {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitSceneCaptureState;

    unsafe impl ClassType for UITraitSceneCaptureState {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UITraitSceneCaptureState {}

unsafe impl UINSIntegerTraitDefinition for UITraitSceneCaptureState {}

unsafe impl UITraitDefinition for UITraitSceneCaptureState {}

extern_methods!(
    unsafe impl UITraitSceneCaptureState {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITraitSceneCaptureState {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
