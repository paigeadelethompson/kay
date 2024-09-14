//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_TYPED_EXTENSIBLE_ENUM
pub type UIPasteboardName = NSString;

extern "C" {
    pub static UIPasteboardNameGeneral: &'static UIPasteboardName;
}

extern "C" {
    pub static UIPasteboardNameFind: &'static NSString;
}

// NS_TYPED_ENUM
pub type UIPasteboardDetectionPattern = NSString;

extern "C" {
    pub static UIPasteboardDetectionPatternProbableWebURL: &'static UIPasteboardDetectionPattern;
}

extern "C" {
    pub static UIPasteboardDetectionPatternProbableWebSearch: &'static UIPasteboardDetectionPattern;
}

extern "C" {
    pub static UIPasteboardDetectionPatternNumber: &'static UIPasteboardDetectionPattern;
}

extern "C" {
    pub static UIPasteboardDetectionPatternLink: &'static UIPasteboardDetectionPattern;
}

extern "C" {
    pub static UIPasteboardDetectionPatternPhoneNumber: &'static UIPasteboardDetectionPattern;
}

extern "C" {
    pub static UIPasteboardDetectionPatternEmailAddress: &'static UIPasteboardDetectionPattern;
}

extern "C" {
    pub static UIPasteboardDetectionPatternPostalAddress: &'static UIPasteboardDetectionPattern;
}

extern "C" {
    pub static UIPasteboardDetectionPatternCalendarEvent: &'static UIPasteboardDetectionPattern;
}

extern "C" {
    pub static UIPasteboardDetectionPatternShipmentTrackingNumber:
        &'static UIPasteboardDetectionPattern;
}

extern "C" {
    pub static UIPasteboardDetectionPatternFlightNumber: &'static UIPasteboardDetectionPattern;
}

extern "C" {
    pub static UIPasteboardDetectionPatternMoneyAmount: &'static UIPasteboardDetectionPattern;
}

// NS_TYPED_ENUM
pub type UIPasteboardOption = NSString;

extern "C" {
    pub static UIPasteboardOptionExpirationDate: &'static UIPasteboardOption;
}

extern "C" {
    pub static UIPasteboardOptionLocalOnly: &'static UIPasteboardOption;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIPasteboard;

    unsafe impl ClassType for UIPasteboard {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for UIPasteboard {}

unsafe impl Sync for UIPasteboard {}

unsafe impl NSObjectProtocol for UIPasteboard {}

extern_methods!(
    unsafe impl UIPasteboard {
        #[method_id(@__retain_semantics Other generalPasteboard)]
        pub unsafe fn generalPasteboard() -> Retained<UIPasteboard>;

        #[method_id(@__retain_semantics Other pasteboardWithName:create:)]
        pub unsafe fn pasteboardWithName_create(
            pasteboard_name: &UIPasteboardName,
            create: bool,
        ) -> Option<Retained<UIPasteboard>>;

        #[method_id(@__retain_semantics Other pasteboardWithUniqueName)]
        pub unsafe fn pasteboardWithUniqueName() -> Retained<UIPasteboard>;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Retained<UIPasteboardName>;

        #[method(removePasteboardWithName:)]
        pub unsafe fn removePasteboardWithName(pasteboard_name: &UIPasteboardName);

        #[method(isPersistent)]
        pub unsafe fn isPersistent(&self) -> bool;

        #[deprecated = "Do not set persistence on pasteboards. This property is set automatically."]
        #[method(setPersistent:)]
        pub unsafe fn setPersistent(&self, persistent: bool);

        #[method(changeCount)]
        pub unsafe fn changeCount(&self) -> NSInteger;

        #[method_id(@__retain_semantics Other itemProviders)]
        pub unsafe fn itemProviders(&self) -> Retained<NSArray<NSItemProvider>>;

        #[method(setItemProviders:)]
        pub unsafe fn setItemProviders(&self, item_providers: &NSArray<NSItemProvider>);

        #[method(setItemProviders:localOnly:expirationDate:)]
        pub unsafe fn setItemProviders_localOnly_expirationDate(
            &self,
            item_providers: &NSArray<NSItemProvider>,
            local_only: bool,
            expiration_date: Option<&NSDate>,
        );

        #[method(setObjects:)]
        pub unsafe fn setObjects(
            &self,
            objects: &NSArray<ProtocolObject<dyn NSItemProviderWriting>>,
        );

        #[method(setObjects:localOnly:expirationDate:)]
        pub unsafe fn setObjects_localOnly_expirationDate(
            &self,
            objects: &NSArray<ProtocolObject<dyn NSItemProviderWriting>>,
            local_only: bool,
            expiration_date: Option<&NSDate>,
        );

        #[method_id(@__retain_semantics Other pasteboardTypes)]
        pub unsafe fn pasteboardTypes(&self) -> Retained<NSArray<NSString>>;

        #[method(containsPasteboardTypes:)]
        pub unsafe fn containsPasteboardTypes(&self, pasteboard_types: &NSArray<NSString>) -> bool;

        #[method_id(@__retain_semantics Other dataForPasteboardType:)]
        pub unsafe fn dataForPasteboardType(
            &self,
            pasteboard_type: &NSString,
        ) -> Option<Retained<NSData>>;

        #[method_id(@__retain_semantics Other valueForPasteboardType:)]
        pub unsafe fn valueForPasteboardType(
            &self,
            pasteboard_type: &NSString,
        ) -> Option<Retained<AnyObject>>;

        #[method(setValue:forPasteboardType:)]
        pub unsafe fn setValue_forPasteboardType(
            &self,
            value: &AnyObject,
            pasteboard_type: &NSString,
        );

        #[method(setData:forPasteboardType:)]
        pub unsafe fn setData_forPasteboardType(&self, data: &NSData, pasteboard_type: &NSString);

        #[method(numberOfItems)]
        pub unsafe fn numberOfItems(&self) -> NSInteger;

        #[method_id(@__retain_semantics Other pasteboardTypesForItemSet:)]
        pub unsafe fn pasteboardTypesForItemSet(
            &self,
            item_set: Option<&NSIndexSet>,
        ) -> Option<Retained<NSArray<NSArray<NSString>>>>;

        #[method(containsPasteboardTypes:inItemSet:)]
        pub unsafe fn containsPasteboardTypes_inItemSet(
            &self,
            pasteboard_types: &NSArray<NSString>,
            item_set: Option<&NSIndexSet>,
        ) -> bool;

        #[method_id(@__retain_semantics Other itemSetWithPasteboardTypes:)]
        pub unsafe fn itemSetWithPasteboardTypes(
            &self,
            pasteboard_types: &NSArray<NSString>,
        ) -> Option<Retained<NSIndexSet>>;

        #[method_id(@__retain_semantics Other valuesForPasteboardType:inItemSet:)]
        pub unsafe fn valuesForPasteboardType_inItemSet(
            &self,
            pasteboard_type: &NSString,
            item_set: Option<&NSIndexSet>,
        ) -> Option<Retained<NSArray>>;

        #[method_id(@__retain_semantics Other dataForPasteboardType:inItemSet:)]
        pub unsafe fn dataForPasteboardType_inItemSet(
            &self,
            pasteboard_type: &NSString,
            item_set: Option<&NSIndexSet>,
        ) -> Option<Retained<NSArray<NSData>>>;

        #[method_id(@__retain_semantics Other items)]
        pub unsafe fn items(&self) -> Retained<NSArray<NSDictionary<NSString, AnyObject>>>;

        #[method(setItems:)]
        pub unsafe fn setItems(&self, items: &NSArray<NSDictionary<NSString, AnyObject>>);

        #[method(addItems:)]
        pub unsafe fn addItems(&self, items: &NSArray<NSDictionary<NSString, AnyObject>>);

        #[method(setItems:options:)]
        pub unsafe fn setItems_options(
            &self,
            items: &NSArray<NSDictionary<NSString, AnyObject>>,
            options: &NSDictionary<UIPasteboardOption, AnyObject>,
        );

        #[method_id(@__retain_semantics Other string)]
        pub unsafe fn string(&self) -> Option<Retained<NSString>>;

        #[method(setString:)]
        pub unsafe fn setString(&self, string: Option<&NSString>);

        #[method_id(@__retain_semantics Other strings)]
        pub unsafe fn strings(&self) -> Option<Retained<NSArray<NSString>>>;

        #[method(setStrings:)]
        pub unsafe fn setStrings(&self, strings: Option<&NSArray<NSString>>);

        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Option<Retained<NSURL>>;

        #[method(setURL:)]
        pub unsafe fn setURL(&self, url: Option<&NSURL>);

        #[method_id(@__retain_semantics Other URLs)]
        pub unsafe fn URLs(&self) -> Option<Retained<NSArray<NSURL>>>;

        #[method(setURLs:)]
        pub unsafe fn setURLs(&self, ur_ls: Option<&NSArray<NSURL>>);

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Retained<UIImage>>;

        #[cfg(feature = "UIImage")]
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&UIImage>);

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other images)]
        pub unsafe fn images(&self) -> Option<Retained<NSArray<UIImage>>>;

        #[cfg(feature = "UIImage")]
        #[method(setImages:)]
        pub unsafe fn setImages(&self, images: Option<&NSArray<UIImage>>);

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other color)]
        pub unsafe fn color(&self) -> Option<Retained<UIColor>>;

        #[cfg(feature = "UIColor")]
        #[method(setColor:)]
        pub unsafe fn setColor(&self, color: Option<&UIColor>);

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other colors)]
        pub unsafe fn colors(&self) -> Option<Retained<NSArray<UIColor>>>;

        #[cfg(feature = "UIColor")]
        #[method(setColors:)]
        pub unsafe fn setColors(&self, colors: Option<&NSArray<UIColor>>);

        #[method(hasStrings)]
        pub unsafe fn hasStrings(&self) -> bool;

        #[method(hasURLs)]
        pub unsafe fn hasURLs(&self) -> bool;

        #[method(hasImages)]
        pub unsafe fn hasImages(&self) -> bool;

        #[method(hasColors)]
        pub unsafe fn hasColors(&self) -> bool;

        #[cfg(feature = "block2")]
        #[method(detectPatternsForPatterns:completionHandler:)]
        pub unsafe fn detectPatternsForPatterns_completionHandler(
            &self,
            patterns: &NSSet<UIPasteboardDetectionPattern>,
            completion_handler: &block2::Block<
                dyn Fn(*mut NSSet<UIPasteboardDetectionPattern>, *mut NSError),
            >,
        );

        #[cfg(feature = "block2")]
        #[method(detectPatternsForPatterns:inItemSet:completionHandler:)]
        pub unsafe fn detectPatternsForPatterns_inItemSet_completionHandler(
            &self,
            patterns: &NSSet<UIPasteboardDetectionPattern>,
            item_set: Option<&NSIndexSet>,
            completion_handler: &block2::Block<
                dyn Fn(*mut NSArray<NSSet<UIPasteboardDetectionPattern>>, *mut NSError),
            >,
        );

        #[cfg(feature = "block2")]
        #[method(detectValuesForPatterns:completionHandler:)]
        pub unsafe fn detectValuesForPatterns_completionHandler(
            &self,
            patterns: &NSSet<UIPasteboardDetectionPattern>,
            completion_handler: &block2::Block<
                dyn Fn(*mut NSDictionary<UIPasteboardDetectionPattern, AnyObject>, *mut NSError),
            >,
        );

        #[cfg(feature = "block2")]
        #[method(detectValuesForPatterns:inItemSet:completionHandler:)]
        pub unsafe fn detectValuesForPatterns_inItemSet_completionHandler(
            &self,
            patterns: &NSSet<UIPasteboardDetectionPattern>,
            item_set: Option<&NSIndexSet>,
            completion_handler: &block2::Block<
                dyn Fn(
                    *mut NSArray<NSDictionary<UIPasteboardDetectionPattern, AnyObject>>,
                    *mut NSError,
                ),
            >,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIPasteboard {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern "C" {
    pub static UIPasteboardChangedNotification: &'static NSNotificationName;
}

extern "C" {
    pub static UIPasteboardChangedTypesAddedKey: &'static NSString;
}

extern "C" {
    pub static UIPasteboardChangedTypesRemovedKey: &'static NSString;
}

extern "C" {
    pub static UIPasteboardRemovedNotification: &'static NSNotificationName;
}

extern "C" {
    pub static UIPasteboardTypeListString: &'static NSArray<NSString>;
}

extern "C" {
    pub static UIPasteboardTypeListURL: &'static NSArray<NSString>;
}

extern "C" {
    pub static UIPasteboardTypeListImage: &'static NSArray<NSString>;
}

extern "C" {
    pub static UIPasteboardTypeListColor: &'static NSArray<NSString>;
}

extern "C" {
    pub static UIPasteboardTypeAutomatic: &'static NSString;
}
