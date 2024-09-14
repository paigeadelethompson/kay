//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIFontPickerViewControllerConfiguration;

    unsafe impl ClassType for UIFontPickerViewControllerConfiguration {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSCopying for UIFontPickerViewControllerConfiguration {}

unsafe impl NSObjectProtocol for UIFontPickerViewControllerConfiguration {}

extern_methods!(
    unsafe impl UIFontPickerViewControllerConfiguration {
        #[method(includeFaces)]
        pub unsafe fn includeFaces(&self) -> bool;

        #[method(setIncludeFaces:)]
        pub unsafe fn setIncludeFaces(&self, include_faces: bool);

        #[method(displayUsingSystemFont)]
        pub unsafe fn displayUsingSystemFont(&self) -> bool;

        #[method(setDisplayUsingSystemFont:)]
        pub unsafe fn setDisplayUsingSystemFont(&self, display_using_system_font: bool);

        #[cfg(feature = "UIFontDescriptor")]
        #[method(filteredTraits)]
        pub unsafe fn filteredTraits(&self) -> UIFontDescriptorSymbolicTraits;

        #[cfg(feature = "UIFontDescriptor")]
        #[method(setFilteredTraits:)]
        pub unsafe fn setFilteredTraits(&self, filtered_traits: UIFontDescriptorSymbolicTraits);

        #[method_id(@__retain_semantics Other filteredLanguagesPredicate)]
        pub unsafe fn filteredLanguagesPredicate(&self) -> Option<Retained<NSPredicate>>;

        #[method(setFilteredLanguagesPredicate:)]
        pub unsafe fn setFilteredLanguagesPredicate(
            &self,
            filtered_languages_predicate: Option<&NSPredicate>,
        );

        #[method_id(@__retain_semantics Other filterPredicateForFilteredLanguages:)]
        pub unsafe fn filterPredicateForFilteredLanguages(
            filtered_languages: &NSArray<NSString>,
            mtm: MainThreadMarker,
        ) -> Option<Retained<NSPredicate>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIFontPickerViewControllerConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
