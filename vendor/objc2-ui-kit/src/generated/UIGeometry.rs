//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UIEdgeInsets {
    pub top: CGFloat,
    pub left: CGFloat,
    pub bottom: CGFloat,
    pub right: CGFloat,
}

unsafe impl Encode for UIEdgeInsets {
    const ENCODING: Encoding = Encoding::Struct(
        "UIEdgeInsets",
        &[
            <CGFloat>::ENCODING,
            <CGFloat>::ENCODING,
            <CGFloat>::ENCODING,
            <CGFloat>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for UIEdgeInsets {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

unsafe impl Send for UIEdgeInsets {}

unsafe impl Sync for UIEdgeInsets {}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NSDirectionalEdgeInsets {
    pub top: CGFloat,
    pub leading: CGFloat,
    pub bottom: CGFloat,
    pub trailing: CGFloat,
}

unsafe impl Encode for NSDirectionalEdgeInsets {
    const ENCODING: Encoding = Encoding::Struct(
        "NSDirectionalEdgeInsets",
        &[
            <CGFloat>::ENCODING,
            <CGFloat>::ENCODING,
            <CGFloat>::ENCODING,
            <CGFloat>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for NSDirectionalEdgeInsets {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

unsafe impl Send for NSDirectionalEdgeInsets {}

unsafe impl Sync for NSDirectionalEdgeInsets {}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UIOffset {
    pub horizontal: CGFloat,
    pub vertical: CGFloat,
}

unsafe impl Encode for UIOffset {
    const ENCODING: Encoding =
        Encoding::Struct("UIOffset", &[<CGFloat>::ENCODING, <CGFloat>::ENCODING]);
}

unsafe impl RefEncode for UIOffset {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

unsafe impl Send for UIOffset {}

unsafe impl Sync for UIOffset {}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIRectEdge(pub NSUInteger);
bitflags::bitflags! {
    impl UIRectEdge: NSUInteger {
        #[doc(alias = "UIRectEdgeNone")]
        const None = 0;
        #[doc(alias = "UIRectEdgeTop")]
        const Top = 1<<0;
        #[doc(alias = "UIRectEdgeLeft")]
        const Left = 1<<1;
        #[doc(alias = "UIRectEdgeBottom")]
        const Bottom = 1<<2;
        #[doc(alias = "UIRectEdgeRight")]
        const Right = 1<<3;
        #[doc(alias = "UIRectEdgeAll")]
        const All = UIRectEdge::Top.0|UIRectEdge::Left.0|UIRectEdge::Bottom.0|UIRectEdge::Right.0;
    }
}

unsafe impl Encode for UIRectEdge {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for UIRectEdge {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIRectCorner(pub NSUInteger);
bitflags::bitflags! {
    impl UIRectCorner: NSUInteger {
        #[doc(alias = "UIRectCornerTopLeft")]
        const TopLeft = 1<<0;
        #[doc(alias = "UIRectCornerTopRight")]
        const TopRight = 1<<1;
        #[doc(alias = "UIRectCornerBottomLeft")]
        const BottomLeft = 1<<2;
        #[doc(alias = "UIRectCornerBottomRight")]
        const BottomRight = 1<<3;
        #[doc(alias = "UIRectCornerAllCorners")]
        const AllCorners = !0;
    }
}

unsafe impl Encode for UIRectCorner {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for UIRectCorner {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIAxis(pub NSUInteger);
bitflags::bitflags! {
    impl UIAxis: NSUInteger {
        #[doc(alias = "UIAxisNeither")]
        const Neither = 0;
        #[doc(alias = "UIAxisHorizontal")]
        const Horizontal = 1<<0;
        #[doc(alias = "UIAxisVertical")]
        const Vertical = 1<<1;
        #[doc(alias = "UIAxisBoth")]
        const Both = UIAxis::Horizontal.0|UIAxis::Vertical.0;
    }
}

unsafe impl Encode for UIAxis {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for UIAxis {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSDirectionalRectEdge(pub NSUInteger);
bitflags::bitflags! {
    impl NSDirectionalRectEdge: NSUInteger {
        #[doc(alias = "NSDirectionalRectEdgeNone")]
        const None = 0;
        #[doc(alias = "NSDirectionalRectEdgeTop")]
        const Top = 1<<0;
        #[doc(alias = "NSDirectionalRectEdgeLeading")]
        const Leading = 1<<1;
        #[doc(alias = "NSDirectionalRectEdgeBottom")]
        const Bottom = 1<<2;
        #[doc(alias = "NSDirectionalRectEdgeTrailing")]
        const Trailing = 1<<3;
        #[doc(alias = "NSDirectionalRectEdgeAll")]
        const All = NSDirectionalRectEdge::Top.0|NSDirectionalRectEdge::Leading.0|NSDirectionalRectEdge::Bottom.0|NSDirectionalRectEdge::Trailing.0;
    }
}

unsafe impl Encode for NSDirectionalRectEdge {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSDirectionalRectEdge {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_OPTIONS
#[deprecated]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIDirectionalRectEdge(pub NSUInteger);
bitflags::bitflags! {
    impl UIDirectionalRectEdge: NSUInteger {
#[deprecated]
        #[doc(alias = "UIDirectionalRectEdgeNone")]
        const None = 0;
#[deprecated]
        #[doc(alias = "UIDirectionalRectEdgeTop")]
        const Top = 1<<0;
#[deprecated]
        #[doc(alias = "UIDirectionalRectEdgeLeading")]
        const Leading = 1<<1;
#[deprecated]
        #[doc(alias = "UIDirectionalRectEdgeBottom")]
        const Bottom = 1<<2;
#[deprecated]
        #[doc(alias = "UIDirectionalRectEdgeTrailing")]
        const Trailing = 1<<3;
#[deprecated]
        #[doc(alias = "UIDirectionalRectEdgeAll")]
        const All = UIDirectionalRectEdge::Top.0|UIDirectionalRectEdge::Leading.0|UIDirectionalRectEdge::Bottom.0|UIDirectionalRectEdge::Trailing.0;
    }
}

unsafe impl Encode for UIDirectionalRectEdge {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for UIDirectionalRectEdge {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSRectAlignment(pub NSInteger);
impl NSRectAlignment {
    #[doc(alias = "NSRectAlignmentNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "NSRectAlignmentTop")]
    pub const Top: Self = Self(1);
    #[doc(alias = "NSRectAlignmentTopLeading")]
    pub const TopLeading: Self = Self(2);
    #[doc(alias = "NSRectAlignmentLeading")]
    pub const Leading: Self = Self(3);
    #[doc(alias = "NSRectAlignmentBottomLeading")]
    pub const BottomLeading: Self = Self(4);
    #[doc(alias = "NSRectAlignmentBottom")]
    pub const Bottom: Self = Self(5);
    #[doc(alias = "NSRectAlignmentBottomTrailing")]
    pub const BottomTrailing: Self = Self(6);
    #[doc(alias = "NSRectAlignmentTrailing")]
    pub const Trailing: Self = Self(7);
    #[doc(alias = "NSRectAlignmentTopTrailing")]
    pub const TopTrailing: Self = Self(8);
}

unsafe impl Encode for NSRectAlignment {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSRectAlignment {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// TODO: pub fn UIEdgeInsetsMake(top: CGFloat,left: CGFloat,bottom: CGFloat,right: CGFloat,) -> UIEdgeInsets;

// TODO: pub fn NSDirectionalEdgeInsetsMake(top: CGFloat,leading: CGFloat,bottom: CGFloat,trailing: CGFloat,) -> NSDirectionalEdgeInsets;

// TODO: pub fn UIEdgeInsetsInsetRect(rect: CGRect,insets: UIEdgeInsets,) -> CGRect;

// TODO: pub fn UIOffsetMake(horizontal: CGFloat,vertical: CGFloat,) -> UIOffset;

// TODO: pub fn UIEdgeInsetsEqualToEdgeInsets(insets1: UIEdgeInsets,insets2: UIEdgeInsets,) -> Bool;

// TODO: pub fn NSDirectionalEdgeInsetsEqualToDirectionalEdgeInsets(insets1: NSDirectionalEdgeInsets,insets2: NSDirectionalEdgeInsets,) -> Bool;

// TODO: pub fn UIOffsetEqualToOffset(offset1: UIOffset,offset2: UIOffset,) -> Bool;

extern "C" {
    pub static UIEdgeInsetsZero: UIEdgeInsets;
}

extern "C" {
    pub static NSDirectionalEdgeInsetsZero: NSDirectionalEdgeInsets;
}

extern "C" {
    pub static UIOffsetZero: UIOffset;
}

extern "C" {
    pub fn NSStringFromCGPoint(point: CGPoint) -> NonNull<NSString>;
}

extern "C" {
    pub fn NSStringFromCGVector(vector: CGVector) -> NonNull<NSString>;
}

extern "C" {
    pub fn NSStringFromCGSize(size: CGSize) -> NonNull<NSString>;
}

extern "C" {
    pub fn NSStringFromCGRect(rect: CGRect) -> NonNull<NSString>;
}

extern "C" {
    pub fn NSStringFromCGAffineTransform(transform: CGAffineTransform) -> NonNull<NSString>;
}

extern "C" {
    pub fn NSStringFromUIEdgeInsets(insets: UIEdgeInsets) -> NonNull<NSString>;
}

extern "C" {
    pub fn NSStringFromDirectionalEdgeInsets(insets: NSDirectionalEdgeInsets) -> NonNull<NSString>;
}

extern "C" {
    pub fn NSStringFromUIOffset(offset: UIOffset) -> NonNull<NSString>;
}

extern "C" {
    pub fn CGPointFromString(string: &NSString) -> CGPoint;
}

extern "C" {
    pub fn CGVectorFromString(string: &NSString) -> CGVector;
}

extern "C" {
    pub fn CGSizeFromString(string: &NSString) -> CGSize;
}

extern "C" {
    pub fn CGRectFromString(string: &NSString) -> CGRect;
}

extern "C" {
    pub fn CGAffineTransformFromString(string: &NSString) -> CGAffineTransform;
}

extern "C" {
    pub fn UIEdgeInsetsFromString(string: &NSString) -> UIEdgeInsets;
}

extern "C" {
    pub fn NSDirectionalEdgeInsetsFromString(string: &NSString) -> NSDirectionalEdgeInsets;
}

extern "C" {
    pub fn UIOffsetFromString(string: &NSString) -> UIOffset;
}

extern_category!(
    /// Category on [`NSValue`].
    pub unsafe trait NSValueUIGeometryExtensions {
        #[method_id(@__retain_semantics Other valueWithCGPoint:)]
        unsafe fn valueWithCGPoint(point: CGPoint) -> Retained<NSValue>;

        #[method_id(@__retain_semantics Other valueWithCGVector:)]
        unsafe fn valueWithCGVector(vector: CGVector) -> Retained<NSValue>;

        #[method_id(@__retain_semantics Other valueWithCGSize:)]
        unsafe fn valueWithCGSize(size: CGSize) -> Retained<NSValue>;

        #[method_id(@__retain_semantics Other valueWithCGRect:)]
        unsafe fn valueWithCGRect(rect: CGRect) -> Retained<NSValue>;

        #[method_id(@__retain_semantics Other valueWithCGAffineTransform:)]
        unsafe fn valueWithCGAffineTransform(transform: CGAffineTransform) -> Retained<NSValue>;

        #[method_id(@__retain_semantics Other valueWithUIEdgeInsets:)]
        unsafe fn valueWithUIEdgeInsets(insets: UIEdgeInsets) -> Retained<NSValue>;

        #[method_id(@__retain_semantics Other valueWithDirectionalEdgeInsets:)]
        unsafe fn valueWithDirectionalEdgeInsets(
            insets: NSDirectionalEdgeInsets,
        ) -> Retained<NSValue>;

        #[method_id(@__retain_semantics Other valueWithUIOffset:)]
        unsafe fn valueWithUIOffset(insets: UIOffset) -> Retained<NSValue>;

        #[method(CGPointValue)]
        unsafe fn CGPointValue(&self) -> CGPoint;

        #[method(CGVectorValue)]
        unsafe fn CGVectorValue(&self) -> CGVector;

        #[method(CGSizeValue)]
        unsafe fn CGSizeValue(&self) -> CGSize;

        #[method(CGRectValue)]
        unsafe fn CGRectValue(&self) -> CGRect;

        #[method(CGAffineTransformValue)]
        unsafe fn CGAffineTransformValue(&self) -> CGAffineTransform;

        #[method(UIEdgeInsetsValue)]
        unsafe fn UIEdgeInsetsValue(&self) -> UIEdgeInsets;

        #[method(directionalEdgeInsetsValue)]
        unsafe fn directionalEdgeInsetsValue(&self) -> NSDirectionalEdgeInsets;

        #[method(UIOffsetValue)]
        unsafe fn UIOffsetValue(&self) -> UIOffset;
    }

    unsafe impl NSValueUIGeometryExtensions for NSValue {}
);

extern_category!(
    /// Category "UIGeometryKeyedCoding" on [`NSCoder`].
    #[doc(alias = "UIGeometryKeyedCoding")]
    pub unsafe trait NSCoderUIGeometryKeyedCoding {
        #[method(encodeCGPoint:forKey:)]
        unsafe fn encodeCGPoint_forKey(&self, point: CGPoint, key: &NSString);

        #[method(encodeCGVector:forKey:)]
        unsafe fn encodeCGVector_forKey(&self, vector: CGVector, key: &NSString);

        #[method(encodeCGSize:forKey:)]
        unsafe fn encodeCGSize_forKey(&self, size: CGSize, key: &NSString);

        #[method(encodeCGRect:forKey:)]
        unsafe fn encodeCGRect_forKey(&self, rect: CGRect, key: &NSString);

        #[method(encodeCGAffineTransform:forKey:)]
        unsafe fn encodeCGAffineTransform_forKey(
            &self,
            transform: CGAffineTransform,
            key: &NSString,
        );

        #[method(encodeUIEdgeInsets:forKey:)]
        unsafe fn encodeUIEdgeInsets_forKey(&self, insets: UIEdgeInsets, key: &NSString);

        #[method(encodeDirectionalEdgeInsets:forKey:)]
        unsafe fn encodeDirectionalEdgeInsets_forKey(
            &self,
            insets: NSDirectionalEdgeInsets,
            key: &NSString,
        );

        #[method(encodeUIOffset:forKey:)]
        unsafe fn encodeUIOffset_forKey(&self, offset: UIOffset, key: &NSString);

        #[method(decodeCGPointForKey:)]
        unsafe fn decodeCGPointForKey(&self, key: &NSString) -> CGPoint;

        #[method(decodeCGVectorForKey:)]
        unsafe fn decodeCGVectorForKey(&self, key: &NSString) -> CGVector;

        #[method(decodeCGSizeForKey:)]
        unsafe fn decodeCGSizeForKey(&self, key: &NSString) -> CGSize;

        #[method(decodeCGRectForKey:)]
        unsafe fn decodeCGRectForKey(&self, key: &NSString) -> CGRect;

        #[method(decodeCGAffineTransformForKey:)]
        unsafe fn decodeCGAffineTransformForKey(&self, key: &NSString) -> CGAffineTransform;

        #[method(decodeUIEdgeInsetsForKey:)]
        unsafe fn decodeUIEdgeInsetsForKey(&self, key: &NSString) -> UIEdgeInsets;

        #[method(decodeDirectionalEdgeInsetsForKey:)]
        unsafe fn decodeDirectionalEdgeInsetsForKey(
            &self,
            key: &NSString,
        ) -> NSDirectionalEdgeInsets;

        #[method(decodeUIOffsetForKey:)]
        unsafe fn decodeUIOffsetForKey(&self, key: &NSString) -> UIOffset;
    }

    unsafe impl NSCoderUIGeometryKeyedCoding for NSCoder {}
);
