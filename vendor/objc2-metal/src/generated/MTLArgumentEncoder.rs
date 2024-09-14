//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

pub static MTLAttributeStrideStatic: NSUInteger = NSUIntegerMax as _;

extern_protocol!(
    pub unsafe trait MTLArgumentEncoder: NSObjectProtocol + IsRetainable {
        #[cfg(feature = "MTLDevice")]
        #[method_id(@__retain_semantics Other device)]
        unsafe fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;

        #[method_id(@__retain_semantics Other label)]
        unsafe fn label(&self) -> Option<Retained<NSString>>;

        #[method(setLabel:)]
        unsafe fn setLabel(&self, label: Option<&NSString>);

        #[method(encodedLength)]
        fn encodedLength(&self) -> NSUInteger;

        #[method(alignment)]
        fn alignment(&self) -> NSUInteger;

        #[cfg(all(feature = "MTLBuffer", feature = "MTLResource"))]
        #[method(setArgumentBuffer:offset:)]
        unsafe fn setArgumentBuffer_offset(
            &self,
            argument_buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
            offset: NSUInteger,
        );

        #[cfg(all(feature = "MTLBuffer", feature = "MTLResource"))]
        #[method(setArgumentBuffer:startOffset:arrayElement:)]
        unsafe fn setArgumentBuffer_startOffset_arrayElement(
            &self,
            argument_buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
            start_offset: NSUInteger,
            array_element: NSUInteger,
        );

        #[cfg(all(feature = "MTLBuffer", feature = "MTLResource"))]
        #[method(setBuffer:offset:atIndex:)]
        unsafe fn setBuffer_offset_atIndex(
            &self,
            buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
            offset: NSUInteger,
            index: NSUInteger,
        );

        #[cfg(all(feature = "MTLBuffer", feature = "MTLResource"))]
        #[method(setBuffers:offsets:withRange:)]
        unsafe fn setBuffers_offsets_withRange(
            &self,
            buffers: NonNull<*const ProtocolObject<dyn MTLBuffer>>,
            offsets: NonNull<NSUInteger>,
            range: NSRange,
        );

        #[cfg(all(feature = "MTLResource", feature = "MTLTexture"))]
        #[method(setTexture:atIndex:)]
        unsafe fn setTexture_atIndex(
            &self,
            texture: Option<&ProtocolObject<dyn MTLTexture>>,
            index: NSUInteger,
        );

        #[cfg(all(feature = "MTLResource", feature = "MTLTexture"))]
        #[method(setTextures:withRange:)]
        unsafe fn setTextures_withRange(
            &self,
            textures: NonNull<*const ProtocolObject<dyn MTLTexture>>,
            range: NSRange,
        );

        #[cfg(feature = "MTLSampler")]
        #[method(setSamplerState:atIndex:)]
        unsafe fn setSamplerState_atIndex(
            &self,
            sampler: Option<&ProtocolObject<dyn MTLSamplerState>>,
            index: NSUInteger,
        );

        #[cfg(feature = "MTLSampler")]
        #[method(setSamplerStates:withRange:)]
        unsafe fn setSamplerStates_withRange(
            &self,
            samplers: NonNull<*const ProtocolObject<dyn MTLSamplerState>>,
            range: NSRange,
        );

        #[method(constantDataAtIndex:)]
        unsafe fn constantDataAtIndex(&self, index: NSUInteger) -> NonNull<c_void>;

        #[cfg(feature = "MTLRenderPipeline")]
        #[method(setRenderPipelineState:atIndex:)]
        unsafe fn setRenderPipelineState_atIndex(
            &self,
            pipeline: Option<&ProtocolObject<dyn MTLRenderPipelineState>>,
            index: NSUInteger,
        );

        #[cfg(feature = "MTLRenderPipeline")]
        #[method(setRenderPipelineStates:withRange:)]
        unsafe fn setRenderPipelineStates_withRange(
            &self,
            pipelines: NonNull<*const ProtocolObject<dyn MTLRenderPipelineState>>,
            range: NSRange,
        );

        #[cfg(feature = "MTLComputePipeline")]
        #[method(setComputePipelineState:atIndex:)]
        unsafe fn setComputePipelineState_atIndex(
            &self,
            pipeline: Option<&ProtocolObject<dyn MTLComputePipelineState>>,
            index: NSUInteger,
        );

        #[cfg(feature = "MTLComputePipeline")]
        #[method(setComputePipelineStates:withRange:)]
        unsafe fn setComputePipelineStates_withRange(
            &self,
            pipelines: NonNull<*const ProtocolObject<dyn MTLComputePipelineState>>,
            range: NSRange,
        );

        #[cfg(all(feature = "MTLIndirectCommandBuffer", feature = "MTLResource"))]
        #[method(setIndirectCommandBuffer:atIndex:)]
        unsafe fn setIndirectCommandBuffer_atIndex(
            &self,
            indirect_command_buffer: Option<&ProtocolObject<dyn MTLIndirectCommandBuffer>>,
            index: NSUInteger,
        );

        #[cfg(all(feature = "MTLIndirectCommandBuffer", feature = "MTLResource"))]
        #[method(setIndirectCommandBuffers:withRange:)]
        unsafe fn setIndirectCommandBuffers_withRange(
            &self,
            buffers: NonNull<*const ProtocolObject<dyn MTLIndirectCommandBuffer>>,
            range: NSRange,
        );

        #[cfg(all(feature = "MTLAccelerationStructure", feature = "MTLResource"))]
        #[method(setAccelerationStructure:atIndex:)]
        unsafe fn setAccelerationStructure_atIndex(
            &self,
            acceleration_structure: Option<&ProtocolObject<dyn MTLAccelerationStructure>>,
            index: NSUInteger,
        );

        #[method_id(@__retain_semantics New newArgumentEncoderForBufferAtIndex:)]
        unsafe fn newArgumentEncoderForBufferAtIndex(
            &self,
            index: NSUInteger,
        ) -> Option<Retained<ProtocolObject<dyn MTLArgumentEncoder>>>;

        #[cfg(all(feature = "MTLResource", feature = "MTLVisibleFunctionTable"))]
        #[method(setVisibleFunctionTable:atIndex:)]
        unsafe fn setVisibleFunctionTable_atIndex(
            &self,
            visible_function_table: Option<&ProtocolObject<dyn MTLVisibleFunctionTable>>,
            index: NSUInteger,
        );

        #[cfg(all(feature = "MTLResource", feature = "MTLVisibleFunctionTable"))]
        #[method(setVisibleFunctionTables:withRange:)]
        unsafe fn setVisibleFunctionTables_withRange(
            &self,
            visible_function_tables: NonNull<*const ProtocolObject<dyn MTLVisibleFunctionTable>>,
            range: NSRange,
        );

        #[cfg(all(feature = "MTLIntersectionFunctionTable", feature = "MTLResource"))]
        #[method(setIntersectionFunctionTable:atIndex:)]
        unsafe fn setIntersectionFunctionTable_atIndex(
            &self,
            intersection_function_table: Option<&ProtocolObject<dyn MTLIntersectionFunctionTable>>,
            index: NSUInteger,
        );

        #[cfg(all(feature = "MTLIntersectionFunctionTable", feature = "MTLResource"))]
        #[method(setIntersectionFunctionTables:withRange:)]
        unsafe fn setIntersectionFunctionTables_withRange(
            &self,
            intersection_function_tables: NonNull<
                *const ProtocolObject<dyn MTLIntersectionFunctionTable>,
            >,
            range: NSRange,
        );
    }

    unsafe impl ProtocolType for dyn MTLArgumentEncoder {}
);
