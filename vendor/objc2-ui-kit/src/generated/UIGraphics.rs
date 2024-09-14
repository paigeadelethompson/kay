//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    pub fn UIGraphicsPopContext();
}

extern "C" {
    pub fn UIRectFill(rect: CGRect);
}

extern "C" {
    pub fn UIRectFrame(rect: CGRect);
}

extern "C" {
    pub fn UIRectClip(rect: CGRect);
}

extern "C" {
    #[deprecated = "Replace usage of UIGraphicsBeginImageContext with UIGraphicsImageRenderer."]
    pub fn UIGraphicsBeginImageContext(size: CGSize);
}

extern "C" {
    #[deprecated = "Replace usage of UIGraphicsBeginImageContextWithOptions with UIGraphicsImageRenderer."]
    pub fn UIGraphicsBeginImageContextWithOptions(size: CGSize, opaque: Bool, scale: CGFloat);
}

extern "C" {
    #[cfg(feature = "UIImage")]
    #[deprecated = "Replace usage of UIGraphicsGetImageFromCurrentImageContext with UIGraphicsImageRendererContext.currentImage."]
    pub fn UIGraphicsGetImageFromCurrentImageContext() -> *mut UIImage;
}

extern "C" {
    #[deprecated = "UIGraphicsEndImageContext should only be used alongside UIGraphicsBeginImageContext[WithOptions]."]
    pub fn UIGraphicsEndImageContext();
}

extern "C" {
    pub fn UIGraphicsBeginPDFContextToFile(
        path: &NSString,
        bounds: CGRect,
        document_info: Option<&NSDictionary>,
    ) -> Bool;
}

extern "C" {
    pub fn UIGraphicsBeginPDFContextToData(
        data: &NSMutableData,
        bounds: CGRect,
        document_info: Option<&NSDictionary>,
    );
}

extern "C" {
    pub fn UIGraphicsEndPDFContext();
}

extern "C" {
    pub fn UIGraphicsBeginPDFPage();
}

extern "C" {
    pub fn UIGraphicsBeginPDFPageWithInfo(bounds: CGRect, page_info: Option<&NSDictionary>);
}

extern "C" {
    pub fn UIGraphicsGetPDFContextBounds() -> CGRect;
}

extern "C" {
    pub fn UIGraphicsSetPDFContextURLForRect(url: &NSURL, rect: CGRect);
}

extern "C" {
    pub fn UIGraphicsAddPDFContextDestinationAtPoint(name: &NSString, point: CGPoint);
}

extern "C" {
    pub fn UIGraphicsSetPDFContextDestinationForRect(name: &NSString, rect: CGRect);
}
