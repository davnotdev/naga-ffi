use super::*;

pub fn spv_back_capability_to_ffi(
    capability: &naga::back::spv::Capability,
) -> ffi::SPVBackCapability {
    match capability {
        naga::back::spv::Capability::Matrix => ffi::SPVBackCapability_SPVBackCapability_Matrix,
        naga::back::spv::Capability::Shader => ffi::SPVBackCapability_SPVBackCapability_Shader,
        naga::back::spv::Capability::Geometry => ffi::SPVBackCapability_SPVBackCapability_Geometry,
        naga::back::spv::Capability::Tessellation => {
            ffi::SPVBackCapability_SPVBackCapability_Tessellation
        }
        naga::back::spv::Capability::Addresses => {
            ffi::SPVBackCapability_SPVBackCapability_Addresses
        }
        naga::back::spv::Capability::Linkage => ffi::SPVBackCapability_SPVBackCapability_Linkage,
        naga::back::spv::Capability::Kernel => ffi::SPVBackCapability_SPVBackCapability_Kernel,
        naga::back::spv::Capability::Vector16 => ffi::SPVBackCapability_SPVBackCapability_Vector16,
        naga::back::spv::Capability::Float16Buffer => {
            ffi::SPVBackCapability_SPVBackCapability_Float16Buffer
        }
        naga::back::spv::Capability::Float16 => ffi::SPVBackCapability_SPVBackCapability_Float16,
        naga::back::spv::Capability::Float64 => ffi::SPVBackCapability_SPVBackCapability_Float64,
        naga::back::spv::Capability::Int64 => ffi::SPVBackCapability_SPVBackCapability_Int64,
        naga::back::spv::Capability::Int64Atomics => {
            ffi::SPVBackCapability_SPVBackCapability_Int64Atomics
        }
        naga::back::spv::Capability::ImageBasic => {
            ffi::SPVBackCapability_SPVBackCapability_ImageBasic
        }
        naga::back::spv::Capability::ImageReadWrite => {
            ffi::SPVBackCapability_SPVBackCapability_ImageReadWrite
        }
        naga::back::spv::Capability::ImageMipmap => {
            ffi::SPVBackCapability_SPVBackCapability_ImageMipmap
        }
        naga::back::spv::Capability::Pipes => ffi::SPVBackCapability_SPVBackCapability_Pipes,
        naga::back::spv::Capability::Groups => ffi::SPVBackCapability_SPVBackCapability_Groups,
        naga::back::spv::Capability::DeviceEnqueue => {
            ffi::SPVBackCapability_SPVBackCapability_DeviceEnqueue
        }
        naga::back::spv::Capability::LiteralSampler => {
            ffi::SPVBackCapability_SPVBackCapability_LiteralSampler
        }
        naga::back::spv::Capability::AtomicStorage => {
            ffi::SPVBackCapability_SPVBackCapability_AtomicStorage
        }
        naga::back::spv::Capability::Int16 => ffi::SPVBackCapability_SPVBackCapability_Int16,
        naga::back::spv::Capability::TessellationPointSize => {
            ffi::SPVBackCapability_SPVBackCapability_TessellationPointSize
        }
        naga::back::spv::Capability::GeometryPointSize => {
            ffi::SPVBackCapability_SPVBackCapability_GeometryPointSize
        }
        naga::back::spv::Capability::ImageGatherExtended => {
            ffi::SPVBackCapability_SPVBackCapability_ImageGatherExtended
        }
        naga::back::spv::Capability::StorageImageMultisample => {
            ffi::SPVBackCapability_SPVBackCapability_StorageImageMultisample
        }
        naga::back::spv::Capability::UniformBufferArrayDynamicIndexing => {
            ffi::SPVBackCapability_SPVBackCapability_UniformBufferArrayDynamicIndexing
        }
        naga::back::spv::Capability::SampledImageArrayDynamicIndexing => {
            ffi::SPVBackCapability_SPVBackCapability_SampledImageArrayDynamicIndexing
        }
        naga::back::spv::Capability::StorageBufferArrayDynamicIndexing => {
            ffi::SPVBackCapability_SPVBackCapability_StorageBufferArrayDynamicIndexing
        }
        naga::back::spv::Capability::StorageImageArrayDynamicIndexing => {
            ffi::SPVBackCapability_SPVBackCapability_StorageImageArrayDynamicIndexing
        }
        naga::back::spv::Capability::ClipDistance => {
            ffi::SPVBackCapability_SPVBackCapability_ClipDistance
        }
        naga::back::spv::Capability::CullDistance => {
            ffi::SPVBackCapability_SPVBackCapability_CullDistance
        }
        naga::back::spv::Capability::ImageCubeArray => {
            ffi::SPVBackCapability_SPVBackCapability_ImageCubeArray
        }
        naga::back::spv::Capability::SampleRateShading => {
            ffi::SPVBackCapability_SPVBackCapability_SampleRateShading
        }
        naga::back::spv::Capability::ImageRect => {
            ffi::SPVBackCapability_SPVBackCapability_ImageRect
        }
        naga::back::spv::Capability::SampledRect => {
            ffi::SPVBackCapability_SPVBackCapability_SampledRect
        }
        naga::back::spv::Capability::GenericPointer => {
            ffi::SPVBackCapability_SPVBackCapability_GenericPointer
        }
        naga::back::spv::Capability::Int8 => ffi::SPVBackCapability_SPVBackCapability_Int8,
        naga::back::spv::Capability::InputAttachment => {
            ffi::SPVBackCapability_SPVBackCapability_InputAttachment
        }
        naga::back::spv::Capability::SparseResidency => {
            ffi::SPVBackCapability_SPVBackCapability_SparseResidency
        }
        naga::back::spv::Capability::MinLod => ffi::SPVBackCapability_SPVBackCapability_MinLod,
        naga::back::spv::Capability::Sampled1D => {
            ffi::SPVBackCapability_SPVBackCapability_Sampled1D
        }
        naga::back::spv::Capability::Image1D => ffi::SPVBackCapability_SPVBackCapability_Image1D,
        naga::back::spv::Capability::SampledCubeArray => {
            ffi::SPVBackCapability_SPVBackCapability_SampledCubeArray
        }
        naga::back::spv::Capability::SampledBuffer => {
            ffi::SPVBackCapability_SPVBackCapability_SampledBuffer
        }
        naga::back::spv::Capability::ImageBuffer => {
            ffi::SPVBackCapability_SPVBackCapability_ImageBuffer
        }
        naga::back::spv::Capability::ImageMSArray => {
            ffi::SPVBackCapability_SPVBackCapability_ImageMSArray
        }
        naga::back::spv::Capability::StorageImageExtendedFormats => {
            ffi::SPVBackCapability_SPVBackCapability_StorageImageExtendedFormats
        }
        naga::back::spv::Capability::ImageQuery => {
            ffi::SPVBackCapability_SPVBackCapability_ImageQuery
        }
        naga::back::spv::Capability::DerivativeControl => {
            ffi::SPVBackCapability_SPVBackCapability_DerivativeControl
        }
        naga::back::spv::Capability::InterpolationFunction => {
            ffi::SPVBackCapability_SPVBackCapability_InterpolationFunction
        }
        naga::back::spv::Capability::TransformFeedback => {
            ffi::SPVBackCapability_SPVBackCapability_TransformFeedback
        }
        naga::back::spv::Capability::GeometryStreams => {
            ffi::SPVBackCapability_SPVBackCapability_GeometryStreams
        }
        naga::back::spv::Capability::StorageImageReadWithoutFormat => {
            ffi::SPVBackCapability_SPVBackCapability_StorageImageReadWithoutFormat
        }
        naga::back::spv::Capability::StorageImageWriteWithoutFormat => {
            ffi::SPVBackCapability_SPVBackCapability_StorageImageWriteWithoutFormat
        }
        naga::back::spv::Capability::MultiViewport => {
            ffi::SPVBackCapability_SPVBackCapability_MultiViewport
        }
        naga::back::spv::Capability::SubgroupDispatch => {
            ffi::SPVBackCapability_SPVBackCapability_SubgroupDispatch
        }
        naga::back::spv::Capability::NamedBarrier => {
            ffi::SPVBackCapability_SPVBackCapability_NamedBarrier
        }
        naga::back::spv::Capability::PipeStorage => {
            ffi::SPVBackCapability_SPVBackCapability_PipeStorage
        }
        naga::back::spv::Capability::GroupNonUniform => {
            ffi::SPVBackCapability_SPVBackCapability_GroupNonUniform
        }
        naga::back::spv::Capability::GroupNonUniformVote => {
            ffi::SPVBackCapability_SPVBackCapability_GroupNonUniformVote
        }
        naga::back::spv::Capability::GroupNonUniformArithmetic => {
            ffi::SPVBackCapability_SPVBackCapability_GroupNonUniformArithmetic
        }
        naga::back::spv::Capability::GroupNonUniformBallot => {
            ffi::SPVBackCapability_SPVBackCapability_GroupNonUniformBallot
        }
        naga::back::spv::Capability::GroupNonUniformShuffle => {
            ffi::SPVBackCapability_SPVBackCapability_GroupNonUniformShuffle
        }
        naga::back::spv::Capability::GroupNonUniformShuffleRelative => {
            ffi::SPVBackCapability_SPVBackCapability_GroupNonUniformShuffleRelative
        }
        naga::back::spv::Capability::GroupNonUniformClustered => {
            ffi::SPVBackCapability_SPVBackCapability_GroupNonUniformClustered
        }
        naga::back::spv::Capability::GroupNonUniformQuad => {
            ffi::SPVBackCapability_SPVBackCapability_GroupNonUniformQuad
        }
        naga::back::spv::Capability::ShaderLayer => {
            ffi::SPVBackCapability_SPVBackCapability_ShaderLayer
        }
        naga::back::spv::Capability::ShaderViewportIndex => {
            ffi::SPVBackCapability_SPVBackCapability_ShaderViewportIndex
        }
        naga::back::spv::Capability::UniformDecoration => {
            ffi::SPVBackCapability_SPVBackCapability_UniformDecoration
        }
        naga::back::spv::Capability::CoreBuiltinsARM => {
            ffi::SPVBackCapability_SPVBackCapability_CoreBuiltinsARM
        }
        naga::back::spv::Capability::TileImageColorReadAccessEXT => {
            ffi::SPVBackCapability_SPVBackCapability_TileImageColorReadAccessEXT
        }
        naga::back::spv::Capability::TileImageDepthReadAccessEXT => {
            ffi::SPVBackCapability_SPVBackCapability_TileImageDepthReadAccessEXT
        }
        naga::back::spv::Capability::TileImageStencilReadAccessEXT => {
            ffi::SPVBackCapability_SPVBackCapability_TileImageStencilReadAccessEXT
        }
        naga::back::spv::Capability::FragmentShadingRateKHR => {
            ffi::SPVBackCapability_SPVBackCapability_FragmentShadingRateKHR
        }
        naga::back::spv::Capability::SubgroupBallotKHR => {
            ffi::SPVBackCapability_SPVBackCapability_SubgroupBallotKHR
        }
        naga::back::spv::Capability::DrawParameters => {
            ffi::SPVBackCapability_SPVBackCapability_DrawParameters
        }
        naga::back::spv::Capability::WorkgroupMemoryExplicitLayoutKHR => {
            ffi::SPVBackCapability_SPVBackCapability_WorkgroupMemoryExplicitLayoutKHR
        }
        naga::back::spv::Capability::WorkgroupMemoryExplicitLayout8BitAccessKHR => {
            ffi::SPVBackCapability_SPVBackCapability_WorkgroupMemoryExplicitLayout8BitAccessKHR
        }
        naga::back::spv::Capability::WorkgroupMemoryExplicitLayout16BitAccessKHR => {
            ffi::SPVBackCapability_SPVBackCapability_WorkgroupMemoryExplicitLayout16BitAccessKHR
        }
        naga::back::spv::Capability::SubgroupVoteKHR => {
            ffi::SPVBackCapability_SPVBackCapability_SubgroupVoteKHR
        }
        naga::back::spv::Capability::StorageBuffer16BitAccess => {
            ffi::SPVBackCapability_SPVBackCapability_StorageBuffer16BitAccess
        }
        naga::back::spv::Capability::UniformAndStorageBuffer16BitAccess => {
            ffi::SPVBackCapability_SPVBackCapability_UniformAndStorageBuffer16BitAccess
        }
        naga::back::spv::Capability::StoragePushConstant16 => {
            ffi::SPVBackCapability_SPVBackCapability_StoragePushConstant16
        }
        naga::back::spv::Capability::StorageInputOutput16 => {
            ffi::SPVBackCapability_SPVBackCapability_StorageInputOutput16
        }
        naga::back::spv::Capability::DeviceGroup => {
            ffi::SPVBackCapability_SPVBackCapability_DeviceGroup
        }
        naga::back::spv::Capability::MultiView => {
            ffi::SPVBackCapability_SPVBackCapability_MultiView
        }
        naga::back::spv::Capability::VariablePointersStorageBuffer => {
            ffi::SPVBackCapability_SPVBackCapability_VariablePointersStorageBuffer
        }
        naga::back::spv::Capability::VariablePointers => {
            ffi::SPVBackCapability_SPVBackCapability_VariablePointers
        }
        naga::back::spv::Capability::AtomicStorageOps => {
            ffi::SPVBackCapability_SPVBackCapability_AtomicStorageOps
        }
        naga::back::spv::Capability::SampleMaskPostDepthCoverage => {
            ffi::SPVBackCapability_SPVBackCapability_SampleMaskPostDepthCoverage
        }
        naga::back::spv::Capability::StorageBuffer8BitAccess => {
            ffi::SPVBackCapability_SPVBackCapability_StorageBuffer8BitAccess
        }
        naga::back::spv::Capability::UniformAndStorageBuffer8BitAccess => {
            ffi::SPVBackCapability_SPVBackCapability_UniformAndStorageBuffer8BitAccess
        }
        naga::back::spv::Capability::StoragePushConstant8 => {
            ffi::SPVBackCapability_SPVBackCapability_StoragePushConstant8
        }
        naga::back::spv::Capability::DenormPreserve => {
            ffi::SPVBackCapability_SPVBackCapability_DenormPreserve
        }
        naga::back::spv::Capability::DenormFlushToZero => {
            ffi::SPVBackCapability_SPVBackCapability_DenormFlushToZero
        }
        naga::back::spv::Capability::SignedZeroInfNanPreserve => {
            ffi::SPVBackCapability_SPVBackCapability_SignedZeroInfNanPreserve
        }
        naga::back::spv::Capability::RoundingModeRTE => {
            ffi::SPVBackCapability_SPVBackCapability_RoundingModeRTE
        }
        naga::back::spv::Capability::RoundingModeRTZ => {
            ffi::SPVBackCapability_SPVBackCapability_RoundingModeRTZ
        }
        naga::back::spv::Capability::RayQueryProvisionalKHR => {
            ffi::SPVBackCapability_SPVBackCapability_RayQueryProvisionalKHR
        }
        naga::back::spv::Capability::RayQueryKHR => {
            ffi::SPVBackCapability_SPVBackCapability_RayQueryKHR
        }
        naga::back::spv::Capability::RayTraversalPrimitiveCullingKHR => {
            ffi::SPVBackCapability_SPVBackCapability_RayTraversalPrimitiveCullingKHR
        }
        naga::back::spv::Capability::RayTracingKHR => {
            ffi::SPVBackCapability_SPVBackCapability_RayTracingKHR
        }
        naga::back::spv::Capability::TextureSampleWeightedQCOM => {
            ffi::SPVBackCapability_SPVBackCapability_TextureSampleWeightedQCOM
        }
        naga::back::spv::Capability::TextureBoxFilterQCOM => {
            ffi::SPVBackCapability_SPVBackCapability_TextureBoxFilterQCOM
        }
        naga::back::spv::Capability::TextureBlockMatchQCOM => {
            ffi::SPVBackCapability_SPVBackCapability_TextureBlockMatchQCOM
        }
        naga::back::spv::Capability::Float16ImageAMD => {
            ffi::SPVBackCapability_SPVBackCapability_Float16ImageAMD
        }
        naga::back::spv::Capability::ImageGatherBiasLodAMD => {
            ffi::SPVBackCapability_SPVBackCapability_ImageGatherBiasLodAMD
        }
        naga::back::spv::Capability::FragmentMaskAMD => {
            ffi::SPVBackCapability_SPVBackCapability_FragmentMaskAMD
        }
        naga::back::spv::Capability::StencilExportEXT => {
            ffi::SPVBackCapability_SPVBackCapability_StencilExportEXT
        }
        naga::back::spv::Capability::ImageReadWriteLodAMD => {
            ffi::SPVBackCapability_SPVBackCapability_ImageReadWriteLodAMD
        }
        naga::back::spv::Capability::Int64ImageEXT => {
            ffi::SPVBackCapability_SPVBackCapability_Int64ImageEXT
        }
        naga::back::spv::Capability::ShaderClockKHR => {
            ffi::SPVBackCapability_SPVBackCapability_ShaderClockKHR
        }
        naga::back::spv::Capability::ShaderEnqueueAMDX => {
            ffi::SPVBackCapability_SPVBackCapability_ShaderEnqueueAMDX
        }
        naga::back::spv::Capability::SampleMaskOverrideCoverageNV => {
            ffi::SPVBackCapability_SPVBackCapability_SampleMaskOverrideCoverageNV
        }
        naga::back::spv::Capability::GeometryShaderPassthroughNV => {
            ffi::SPVBackCapability_SPVBackCapability_GeometryShaderPassthroughNV
        }
        naga::back::spv::Capability::ShaderViewportIndexLayerEXT => {
            ffi::SPVBackCapability_SPVBackCapability_ShaderViewportIndexLayerEXT
        }
        naga::back::spv::Capability::ShaderViewportMaskNV => {
            ffi::SPVBackCapability_SPVBackCapability_ShaderViewportMaskNV
        }
        naga::back::spv::Capability::ShaderStereoViewNV => {
            ffi::SPVBackCapability_SPVBackCapability_ShaderStereoViewNV
        }
        naga::back::spv::Capability::PerViewAttributesNV => {
            ffi::SPVBackCapability_SPVBackCapability_PerViewAttributesNV
        }
        naga::back::spv::Capability::FragmentFullyCoveredEXT => {
            ffi::SPVBackCapability_SPVBackCapability_FragmentFullyCoveredEXT
        }
        naga::back::spv::Capability::MeshShadingNV => {
            ffi::SPVBackCapability_SPVBackCapability_MeshShadingNV
        }
        naga::back::spv::Capability::ImageFootprintNV => {
            ffi::SPVBackCapability_SPVBackCapability_ImageFootprintNV
        }
        naga::back::spv::Capability::MeshShadingEXT => {
            ffi::SPVBackCapability_SPVBackCapability_MeshShadingEXT
        }
        naga::back::spv::Capability::FragmentBarycentricKHR => {
            ffi::SPVBackCapability_SPVBackCapability_FragmentBarycentricKHR
        }
        naga::back::spv::Capability::ComputeDerivativeGroupQuadsNV => {
            ffi::SPVBackCapability_SPVBackCapability_ComputeDerivativeGroupQuadsNV
        }
        naga::back::spv::Capability::FragmentDensityEXT => {
            ffi::SPVBackCapability_SPVBackCapability_FragmentDensityEXT
        }
        naga::back::spv::Capability::GroupNonUniformPartitionedNV => {
            ffi::SPVBackCapability_SPVBackCapability_GroupNonUniformPartitionedNV
        }
        naga::back::spv::Capability::ShaderNonUniform => {
            ffi::SPVBackCapability_SPVBackCapability_ShaderNonUniform
        }
        naga::back::spv::Capability::RuntimeDescriptorArray => {
            ffi::SPVBackCapability_SPVBackCapability_RuntimeDescriptorArray
        }
        naga::back::spv::Capability::InputAttachmentArrayDynamicIndexing => {
            ffi::SPVBackCapability_SPVBackCapability_InputAttachmentArrayDynamicIndexing
        }
        naga::back::spv::Capability::UniformTexelBufferArrayDynamicIndexing => {
            ffi::SPVBackCapability_SPVBackCapability_UniformTexelBufferArrayDynamicIndexing
        }
        naga::back::spv::Capability::StorageTexelBufferArrayDynamicIndexing => {
            ffi::SPVBackCapability_SPVBackCapability_StorageTexelBufferArrayDynamicIndexing
        }
        naga::back::spv::Capability::UniformBufferArrayNonUniformIndexing => {
            ffi::SPVBackCapability_SPVBackCapability_UniformBufferArrayNonUniformIndexing
        }
        naga::back::spv::Capability::SampledImageArrayNonUniformIndexing => {
            ffi::SPVBackCapability_SPVBackCapability_SampledImageArrayNonUniformIndexing
        }
        naga::back::spv::Capability::StorageBufferArrayNonUniformIndexing => {
            ffi::SPVBackCapability_SPVBackCapability_StorageBufferArrayNonUniformIndexing
        }
        naga::back::spv::Capability::StorageImageArrayNonUniformIndexing => {
            ffi::SPVBackCapability_SPVBackCapability_StorageImageArrayNonUniformIndexing
        }
        naga::back::spv::Capability::InputAttachmentArrayNonUniformIndexing => {
            ffi::SPVBackCapability_SPVBackCapability_InputAttachmentArrayNonUniformIndexing
        }
        naga::back::spv::Capability::UniformTexelBufferArrayNonUniformIndexing => {
            ffi::SPVBackCapability_SPVBackCapability_UniformTexelBufferArrayNonUniformIndexing
        }
        naga::back::spv::Capability::StorageTexelBufferArrayNonUniformIndexing => {
            ffi::SPVBackCapability_SPVBackCapability_StorageTexelBufferArrayNonUniformIndexing
        }
        naga::back::spv::Capability::RayTracingPositionFetchKHR => {
            ffi::SPVBackCapability_SPVBackCapability_RayTracingPositionFetchKHR
        }
        naga::back::spv::Capability::RayTracingNV => {
            ffi::SPVBackCapability_SPVBackCapability_RayTracingNV
        }
        naga::back::spv::Capability::RayTracingMotionBlurNV => {
            ffi::SPVBackCapability_SPVBackCapability_RayTracingMotionBlurNV
        }
        naga::back::spv::Capability::VulkanMemoryModel => {
            ffi::SPVBackCapability_SPVBackCapability_VulkanMemoryModel
        }
        naga::back::spv::Capability::VulkanMemoryModelDeviceScope => {
            ffi::SPVBackCapability_SPVBackCapability_VulkanMemoryModelDeviceScope
        }
        naga::back::spv::Capability::PhysicalStorageBufferAddresses => {
            ffi::SPVBackCapability_SPVBackCapability_PhysicalStorageBufferAddresses
        }
        naga::back::spv::Capability::ComputeDerivativeGroupLinearNV => {
            ffi::SPVBackCapability_SPVBackCapability_ComputeDerivativeGroupLinearNV
        }
        naga::back::spv::Capability::RayTracingProvisionalKHR => {
            ffi::SPVBackCapability_SPVBackCapability_RayTracingProvisionalKHR
        }
        naga::back::spv::Capability::CooperativeMatrixNV => {
            ffi::SPVBackCapability_SPVBackCapability_CooperativeMatrixNV
        }
        naga::back::spv::Capability::FragmentShaderSampleInterlockEXT => {
            ffi::SPVBackCapability_SPVBackCapability_FragmentShaderSampleInterlockEXT
        }
        naga::back::spv::Capability::FragmentShaderShadingRateInterlockEXT => {
            ffi::SPVBackCapability_SPVBackCapability_FragmentShaderShadingRateInterlockEXT
        }
        naga::back::spv::Capability::ShaderSMBuiltinsNV => {
            ffi::SPVBackCapability_SPVBackCapability_ShaderSMBuiltinsNV
        }
        naga::back::spv::Capability::FragmentShaderPixelInterlockEXT => {
            ffi::SPVBackCapability_SPVBackCapability_FragmentShaderPixelInterlockEXT
        }
        naga::back::spv::Capability::DemoteToHelperInvocation => {
            ffi::SPVBackCapability_SPVBackCapability_DemoteToHelperInvocation
        }
        naga::back::spv::Capability::DisplacementMicromapNV => {
            ffi::SPVBackCapability_SPVBackCapability_DisplacementMicromapNV
        }
        naga::back::spv::Capability::RayTracingOpacityMicromapEXT => {
            ffi::SPVBackCapability_SPVBackCapability_RayTracingOpacityMicromapEXT
        }
        naga::back::spv::Capability::ShaderInvocationReorderNV => {
            ffi::SPVBackCapability_SPVBackCapability_ShaderInvocationReorderNV
        }
        naga::back::spv::Capability::BindlessTextureNV => {
            ffi::SPVBackCapability_SPVBackCapability_BindlessTextureNV
        }
        naga::back::spv::Capability::RayQueryPositionFetchKHR => {
            ffi::SPVBackCapability_SPVBackCapability_RayQueryPositionFetchKHR
        }
        naga::back::spv::Capability::RayTracingDisplacementMicromapNV => {
            ffi::SPVBackCapability_SPVBackCapability_RayTracingDisplacementMicromapNV
        }
        naga::back::spv::Capability::SubgroupShuffleINTEL => {
            ffi::SPVBackCapability_SPVBackCapability_SubgroupShuffleINTEL
        }
        naga::back::spv::Capability::SubgroupBufferBlockIOINTEL => {
            ffi::SPVBackCapability_SPVBackCapability_SubgroupBufferBlockIOINTEL
        }
        naga::back::spv::Capability::SubgroupImageBlockIOINTEL => {
            ffi::SPVBackCapability_SPVBackCapability_SubgroupImageBlockIOINTEL
        }
        naga::back::spv::Capability::SubgroupImageMediaBlockIOINTEL => {
            ffi::SPVBackCapability_SPVBackCapability_SubgroupImageMediaBlockIOINTEL
        }
        naga::back::spv::Capability::RoundToInfinityINTEL => {
            ffi::SPVBackCapability_SPVBackCapability_RoundToInfinityINTEL
        }
        naga::back::spv::Capability::FloatingPointModeINTEL => {
            ffi::SPVBackCapability_SPVBackCapability_FloatingPointModeINTEL
        }
        naga::back::spv::Capability::IntegerFunctions2INTEL => {
            ffi::SPVBackCapability_SPVBackCapability_IntegerFunctions2INTEL
        }
        naga::back::spv::Capability::FunctionPointersINTEL => {
            ffi::SPVBackCapability_SPVBackCapability_FunctionPointersINTEL
        }
        naga::back::spv::Capability::IndirectReferencesINTEL => {
            ffi::SPVBackCapability_SPVBackCapability_IndirectReferencesINTEL
        }
        naga::back::spv::Capability::AsmINTEL => ffi::SPVBackCapability_SPVBackCapability_AsmINTEL,
        naga::back::spv::Capability::AtomicFloat32MinMaxEXT => {
            ffi::SPVBackCapability_SPVBackCapability_AtomicFloat32MinMaxEXT
        }
        naga::back::spv::Capability::AtomicFloat64MinMaxEXT => {
            ffi::SPVBackCapability_SPVBackCapability_AtomicFloat64MinMaxEXT
        }
        naga::back::spv::Capability::AtomicFloat16MinMaxEXT => {
            ffi::SPVBackCapability_SPVBackCapability_AtomicFloat16MinMaxEXT
        }
        naga::back::spv::Capability::VectorComputeINTEL => {
            ffi::SPVBackCapability_SPVBackCapability_VectorComputeINTEL
        }
        naga::back::spv::Capability::VectorAnyINTEL => {
            ffi::SPVBackCapability_SPVBackCapability_VectorAnyINTEL
        }
        naga::back::spv::Capability::ExpectAssumeKHR => {
            ffi::SPVBackCapability_SPVBackCapability_ExpectAssumeKHR
        }
        naga::back::spv::Capability::SubgroupAvcMotionEstimationINTEL => {
            ffi::SPVBackCapability_SPVBackCapability_SubgroupAvcMotionEstimationINTEL
        }
        naga::back::spv::Capability::SubgroupAvcMotionEstimationIntraINTEL => {
            ffi::SPVBackCapability_SPVBackCapability_SubgroupAvcMotionEstimationIntraINTEL
        }
        naga::back::spv::Capability::SubgroupAvcMotionEstimationChromaINTEL => {
            ffi::SPVBackCapability_SPVBackCapability_SubgroupAvcMotionEstimationChromaINTEL
        }
        naga::back::spv::Capability::VariableLengthArrayINTEL => {
            ffi::SPVBackCapability_SPVBackCapability_VariableLengthArrayINTEL
        }
        naga::back::spv::Capability::FunctionFloatControlINTEL => {
            ffi::SPVBackCapability_SPVBackCapability_FunctionFloatControlINTEL
        }
        naga::back::spv::Capability::FPGAMemoryAttributesINTEL => {
            ffi::SPVBackCapability_SPVBackCapability_FPGAMemoryAttributesINTEL
        }
        naga::back::spv::Capability::FPFastMathModeINTEL => {
            ffi::SPVBackCapability_SPVBackCapability_FPFastMathModeINTEL
        }
        naga::back::spv::Capability::ArbitraryPrecisionIntegersINTEL => {
            ffi::SPVBackCapability_SPVBackCapability_ArbitraryPrecisionIntegersINTEL
        }
        naga::back::spv::Capability::ArbitraryPrecisionFloatingPointINTEL => {
            ffi::SPVBackCapability_SPVBackCapability_ArbitraryPrecisionFloatingPointINTEL
        }
        naga::back::spv::Capability::UnstructuredLoopControlsINTEL => {
            ffi::SPVBackCapability_SPVBackCapability_UnstructuredLoopControlsINTEL
        }
        naga::back::spv::Capability::FPGALoopControlsINTEL => {
            ffi::SPVBackCapability_SPVBackCapability_FPGALoopControlsINTEL
        }
        naga::back::spv::Capability::KernelAttributesINTEL => {
            ffi::SPVBackCapability_SPVBackCapability_KernelAttributesINTEL
        }
        naga::back::spv::Capability::FPGAKernelAttributesINTEL => {
            ffi::SPVBackCapability_SPVBackCapability_FPGAKernelAttributesINTEL
        }
        naga::back::spv::Capability::FPGAMemoryAccessesINTEL => {
            ffi::SPVBackCapability_SPVBackCapability_FPGAMemoryAccessesINTEL
        }
        naga::back::spv::Capability::FPGAClusterAttributesINTEL => {
            ffi::SPVBackCapability_SPVBackCapability_FPGAClusterAttributesINTEL
        }
        naga::back::spv::Capability::LoopFuseINTEL => {
            ffi::SPVBackCapability_SPVBackCapability_LoopFuseINTEL
        }
        naga::back::spv::Capability::FPGADSPControlINTEL => {
            ffi::SPVBackCapability_SPVBackCapability_FPGADSPControlINTEL
        }
        naga::back::spv::Capability::MemoryAccessAliasingINTEL => {
            ffi::SPVBackCapability_SPVBackCapability_MemoryAccessAliasingINTEL
        }
        naga::back::spv::Capability::FPGAInvocationPipeliningAttributesINTEL => {
            ffi::SPVBackCapability_SPVBackCapability_FPGAInvocationPipeliningAttributesINTEL
        }
        naga::back::spv::Capability::FPGABufferLocationINTEL => {
            ffi::SPVBackCapability_SPVBackCapability_FPGABufferLocationINTEL
        }
        naga::back::spv::Capability::ArbitraryPrecisionFixedPointINTEL => {
            ffi::SPVBackCapability_SPVBackCapability_ArbitraryPrecisionFixedPointINTEL
        }
        naga::back::spv::Capability::USMStorageClassesINTEL => {
            ffi::SPVBackCapability_SPVBackCapability_USMStorageClassesINTEL
        }
        naga::back::spv::Capability::RuntimeAlignedAttributeINTEL => {
            ffi::SPVBackCapability_SPVBackCapability_RuntimeAlignedAttributeINTEL
        }
        naga::back::spv::Capability::IOPipesINTEL => {
            ffi::SPVBackCapability_SPVBackCapability_IOPipesINTEL
        }
        naga::back::spv::Capability::BlockingPipesINTEL => {
            ffi::SPVBackCapability_SPVBackCapability_BlockingPipesINTEL
        }
        naga::back::spv::Capability::FPGARegINTEL => {
            ffi::SPVBackCapability_SPVBackCapability_FPGARegINTEL
        }
        naga::back::spv::Capability::DotProductInputAll => {
            ffi::SPVBackCapability_SPVBackCapability_DotProductInputAll
        }
        naga::back::spv::Capability::DotProductInput4x8Bit => {
            ffi::SPVBackCapability_SPVBackCapability_DotProductInput4x8Bit
        }
        naga::back::spv::Capability::DotProductInput4x8BitPacked => {
            ffi::SPVBackCapability_SPVBackCapability_DotProductInput4x8BitPacked
        }
        naga::back::spv::Capability::DotProduct => {
            ffi::SPVBackCapability_SPVBackCapability_DotProduct
        }
        naga::back::spv::Capability::RayCullMaskKHR => {
            ffi::SPVBackCapability_SPVBackCapability_RayCullMaskKHR
        }
        naga::back::spv::Capability::CooperativeMatrixKHR => {
            ffi::SPVBackCapability_SPVBackCapability_CooperativeMatrixKHR
        }
        naga::back::spv::Capability::BitInstructions => {
            ffi::SPVBackCapability_SPVBackCapability_BitInstructions
        }
        naga::back::spv::Capability::GroupNonUniformRotateKHR => {
            ffi::SPVBackCapability_SPVBackCapability_GroupNonUniformRotateKHR
        }
        naga::back::spv::Capability::AtomicFloat32AddEXT => {
            ffi::SPVBackCapability_SPVBackCapability_AtomicFloat32AddEXT
        }
        naga::back::spv::Capability::AtomicFloat64AddEXT => {
            ffi::SPVBackCapability_SPVBackCapability_AtomicFloat64AddEXT
        }
        naga::back::spv::Capability::LongConstantCompositeINTEL => {
            ffi::SPVBackCapability_SPVBackCapability_LongConstantCompositeINTEL
        }
        naga::back::spv::Capability::OptNoneINTEL => {
            ffi::SPVBackCapability_SPVBackCapability_OptNoneINTEL
        }
        naga::back::spv::Capability::AtomicFloat16AddEXT => {
            ffi::SPVBackCapability_SPVBackCapability_AtomicFloat16AddEXT
        }
        naga::back::spv::Capability::DebugInfoModuleINTEL => {
            ffi::SPVBackCapability_SPVBackCapability_DebugInfoModuleINTEL
        }
        naga::back::spv::Capability::BFloat16ConversionINTEL => {
            ffi::SPVBackCapability_SPVBackCapability_BFloat16ConversionINTEL
        }
        naga::back::spv::Capability::SplitBarrierINTEL => {
            ffi::SPVBackCapability_SPVBackCapability_SplitBarrierINTEL
        }
        naga::back::spv::Capability::GlobalVariableFPGADecorationsINTEL => {
            ffi::SPVBackCapability_SPVBackCapability_GlobalVariableFPGADecorationsINTEL
        }
        naga::back::spv::Capability::FPGAKernelAttributesv2INTEL => {
            ffi::SPVBackCapability_SPVBackCapability_FPGAKernelAttributesv2INTEL
        }
        naga::back::spv::Capability::GlobalVariableHostAccessINTEL => {
            ffi::SPVBackCapability_SPVBackCapability_GlobalVariableHostAccessINTEL
        }
        naga::back::spv::Capability::FPMaxErrorINTEL => {
            ffi::SPVBackCapability_SPVBackCapability_FPMaxErrorINTEL
        }
        naga::back::spv::Capability::FPGALatencyControlINTEL => {
            ffi::SPVBackCapability_SPVBackCapability_FPGALatencyControlINTEL
        }
        naga::back::spv::Capability::FPGAArgumentInterfacesINTEL => {
            ffi::SPVBackCapability_SPVBackCapability_FPGAArgumentInterfacesINTEL
        }
        naga::back::spv::Capability::GroupUniformArithmeticKHR => {
            ffi::SPVBackCapability_SPVBackCapability_GroupUniformArithmeticKHR
        }
        naga::back::spv::Capability::CacheControlsINTEL => {
            ffi::SPVBackCapability_SPVBackCapability_CacheControlsINTEL
        }
    }
}

pub fn spv_back_capability_to_naga(
    capability: ffi::SPVBackCapability,
) -> naga::back::spv::Capability {
    match capability {
        ffi::SPVBackCapability_SPVBackCapability_Matrix => naga::back::spv::Capability::Matrix,
        ffi::SPVBackCapability_SPVBackCapability_Shader => naga::back::spv::Capability::Shader,
        ffi::SPVBackCapability_SPVBackCapability_Geometry => naga::back::spv::Capability::Geometry,
        ffi::SPVBackCapability_SPVBackCapability_Tessellation => {
            naga::back::spv::Capability::Tessellation
        }
        ffi::SPVBackCapability_SPVBackCapability_Addresses => {
            naga::back::spv::Capability::Addresses
        }
        ffi::SPVBackCapability_SPVBackCapability_Linkage => naga::back::spv::Capability::Linkage,
        ffi::SPVBackCapability_SPVBackCapability_Kernel => naga::back::spv::Capability::Kernel,
        ffi::SPVBackCapability_SPVBackCapability_Vector16 => naga::back::spv::Capability::Vector16,
        ffi::SPVBackCapability_SPVBackCapability_Float16Buffer => {
            naga::back::spv::Capability::Float16Buffer
        }
        ffi::SPVBackCapability_SPVBackCapability_Float16 => naga::back::spv::Capability::Float16,
        ffi::SPVBackCapability_SPVBackCapability_Float64 => naga::back::spv::Capability::Float64,
        ffi::SPVBackCapability_SPVBackCapability_Int64 => naga::back::spv::Capability::Int64,
        ffi::SPVBackCapability_SPVBackCapability_Int64Atomics => {
            naga::back::spv::Capability::Int64Atomics
        }
        ffi::SPVBackCapability_SPVBackCapability_ImageBasic => {
            naga::back::spv::Capability::ImageBasic
        }
        ffi::SPVBackCapability_SPVBackCapability_ImageReadWrite => {
            naga::back::spv::Capability::ImageReadWrite
        }
        ffi::SPVBackCapability_SPVBackCapability_ImageMipmap => {
            naga::back::spv::Capability::ImageMipmap
        }
        ffi::SPVBackCapability_SPVBackCapability_Pipes => naga::back::spv::Capability::Pipes,
        ffi::SPVBackCapability_SPVBackCapability_Groups => naga::back::spv::Capability::Groups,
        ffi::SPVBackCapability_SPVBackCapability_DeviceEnqueue => {
            naga::back::spv::Capability::DeviceEnqueue
        }
        ffi::SPVBackCapability_SPVBackCapability_LiteralSampler => {
            naga::back::spv::Capability::LiteralSampler
        }
        ffi::SPVBackCapability_SPVBackCapability_AtomicStorage => {
            naga::back::spv::Capability::AtomicStorage
        }
        ffi::SPVBackCapability_SPVBackCapability_Int16 => naga::back::spv::Capability::Int16,
        ffi::SPVBackCapability_SPVBackCapability_TessellationPointSize => {
            naga::back::spv::Capability::TessellationPointSize
        }
        ffi::SPVBackCapability_SPVBackCapability_GeometryPointSize => {
            naga::back::spv::Capability::GeometryPointSize
        }
        ffi::SPVBackCapability_SPVBackCapability_ImageGatherExtended => {
            naga::back::spv::Capability::ImageGatherExtended
        }
        ffi::SPVBackCapability_SPVBackCapability_StorageImageMultisample => {
            naga::back::spv::Capability::StorageImageMultisample
        }
        ffi::SPVBackCapability_SPVBackCapability_UniformBufferArrayDynamicIndexing => {
            naga::back::spv::Capability::UniformBufferArrayDynamicIndexing
        }
        ffi::SPVBackCapability_SPVBackCapability_SampledImageArrayDynamicIndexing => {
            naga::back::spv::Capability::SampledImageArrayDynamicIndexing
        }
        ffi::SPVBackCapability_SPVBackCapability_StorageBufferArrayDynamicIndexing => {
            naga::back::spv::Capability::StorageBufferArrayDynamicIndexing
        }
        ffi::SPVBackCapability_SPVBackCapability_StorageImageArrayDynamicIndexing => {
            naga::back::spv::Capability::StorageImageArrayDynamicIndexing
        }
        ffi::SPVBackCapability_SPVBackCapability_ClipDistance => {
            naga::back::spv::Capability::ClipDistance
        }
        ffi::SPVBackCapability_SPVBackCapability_CullDistance => {
            naga::back::spv::Capability::CullDistance
        }
        ffi::SPVBackCapability_SPVBackCapability_ImageCubeArray => {
            naga::back::spv::Capability::ImageCubeArray
        }
        ffi::SPVBackCapability_SPVBackCapability_SampleRateShading => {
            naga::back::spv::Capability::SampleRateShading
        }
        ffi::SPVBackCapability_SPVBackCapability_ImageRect => {
            naga::back::spv::Capability::ImageRect
        }
        ffi::SPVBackCapability_SPVBackCapability_SampledRect => {
            naga::back::spv::Capability::SampledRect
        }
        ffi::SPVBackCapability_SPVBackCapability_GenericPointer => {
            naga::back::spv::Capability::GenericPointer
        }
        ffi::SPVBackCapability_SPVBackCapability_Int8 => naga::back::spv::Capability::Int8,
        ffi::SPVBackCapability_SPVBackCapability_InputAttachment => {
            naga::back::spv::Capability::InputAttachment
        }
        ffi::SPVBackCapability_SPVBackCapability_SparseResidency => {
            naga::back::spv::Capability::SparseResidency
        }
        ffi::SPVBackCapability_SPVBackCapability_MinLod => naga::back::spv::Capability::MinLod,
        ffi::SPVBackCapability_SPVBackCapability_Sampled1D => {
            naga::back::spv::Capability::Sampled1D
        }
        ffi::SPVBackCapability_SPVBackCapability_Image1D => naga::back::spv::Capability::Image1D,
        ffi::SPVBackCapability_SPVBackCapability_SampledCubeArray => {
            naga::back::spv::Capability::SampledCubeArray
        }
        ffi::SPVBackCapability_SPVBackCapability_SampledBuffer => {
            naga::back::spv::Capability::SampledBuffer
        }
        ffi::SPVBackCapability_SPVBackCapability_ImageBuffer => {
            naga::back::spv::Capability::ImageBuffer
        }
        ffi::SPVBackCapability_SPVBackCapability_ImageMSArray => {
            naga::back::spv::Capability::ImageMSArray
        }
        ffi::SPVBackCapability_SPVBackCapability_StorageImageExtendedFormats => {
            naga::back::spv::Capability::StorageImageExtendedFormats
        }
        ffi::SPVBackCapability_SPVBackCapability_ImageQuery => {
            naga::back::spv::Capability::ImageQuery
        }
        ffi::SPVBackCapability_SPVBackCapability_DerivativeControl => {
            naga::back::spv::Capability::DerivativeControl
        }
        ffi::SPVBackCapability_SPVBackCapability_InterpolationFunction => {
            naga::back::spv::Capability::InterpolationFunction
        }
        ffi::SPVBackCapability_SPVBackCapability_TransformFeedback => {
            naga::back::spv::Capability::TransformFeedback
        }
        ffi::SPVBackCapability_SPVBackCapability_GeometryStreams => {
            naga::back::spv::Capability::GeometryStreams
        }
        ffi::SPVBackCapability_SPVBackCapability_StorageImageReadWithoutFormat => {
            naga::back::spv::Capability::StorageImageReadWithoutFormat
        }
        ffi::SPVBackCapability_SPVBackCapability_StorageImageWriteWithoutFormat => {
            naga::back::spv::Capability::StorageImageWriteWithoutFormat
        }
        ffi::SPVBackCapability_SPVBackCapability_MultiViewport => {
            naga::back::spv::Capability::MultiViewport
        }
        ffi::SPVBackCapability_SPVBackCapability_SubgroupDispatch => {
            naga::back::spv::Capability::SubgroupDispatch
        }
        ffi::SPVBackCapability_SPVBackCapability_NamedBarrier => {
            naga::back::spv::Capability::NamedBarrier
        }
        ffi::SPVBackCapability_SPVBackCapability_PipeStorage => {
            naga::back::spv::Capability::PipeStorage
        }
        ffi::SPVBackCapability_SPVBackCapability_GroupNonUniform => {
            naga::back::spv::Capability::GroupNonUniform
        }
        ffi::SPVBackCapability_SPVBackCapability_GroupNonUniformVote => {
            naga::back::spv::Capability::GroupNonUniformVote
        }
        ffi::SPVBackCapability_SPVBackCapability_GroupNonUniformArithmetic => {
            naga::back::spv::Capability::GroupNonUniformArithmetic
        }
        ffi::SPVBackCapability_SPVBackCapability_GroupNonUniformBallot => {
            naga::back::spv::Capability::GroupNonUniformBallot
        }
        ffi::SPVBackCapability_SPVBackCapability_GroupNonUniformShuffle => {
            naga::back::spv::Capability::GroupNonUniformShuffle
        }
        ffi::SPVBackCapability_SPVBackCapability_GroupNonUniformShuffleRelative => {
            naga::back::spv::Capability::GroupNonUniformShuffleRelative
        }
        ffi::SPVBackCapability_SPVBackCapability_GroupNonUniformClustered => {
            naga::back::spv::Capability::GroupNonUniformClustered
        }
        ffi::SPVBackCapability_SPVBackCapability_GroupNonUniformQuad => {
            naga::back::spv::Capability::GroupNonUniformQuad
        }
        ffi::SPVBackCapability_SPVBackCapability_ShaderLayer => {
            naga::back::spv::Capability::ShaderLayer
        }
        ffi::SPVBackCapability_SPVBackCapability_ShaderViewportIndex => {
            naga::back::spv::Capability::ShaderViewportIndex
        }
        ffi::SPVBackCapability_SPVBackCapability_UniformDecoration => {
            naga::back::spv::Capability::UniformDecoration
        }
        ffi::SPVBackCapability_SPVBackCapability_CoreBuiltinsARM => {
            naga::back::spv::Capability::CoreBuiltinsARM
        }
        ffi::SPVBackCapability_SPVBackCapability_TileImageColorReadAccessEXT => {
            naga::back::spv::Capability::TileImageColorReadAccessEXT
        }
        ffi::SPVBackCapability_SPVBackCapability_TileImageDepthReadAccessEXT => {
            naga::back::spv::Capability::TileImageDepthReadAccessEXT
        }
        ffi::SPVBackCapability_SPVBackCapability_TileImageStencilReadAccessEXT => {
            naga::back::spv::Capability::TileImageStencilReadAccessEXT
        }
        ffi::SPVBackCapability_SPVBackCapability_FragmentShadingRateKHR => {
            naga::back::spv::Capability::FragmentShadingRateKHR
        }
        ffi::SPVBackCapability_SPVBackCapability_SubgroupBallotKHR => {
            naga::back::spv::Capability::SubgroupBallotKHR
        }
        ffi::SPVBackCapability_SPVBackCapability_DrawParameters => {
            naga::back::spv::Capability::DrawParameters
        }
        ffi::SPVBackCapability_SPVBackCapability_WorkgroupMemoryExplicitLayoutKHR => {
            naga::back::spv::Capability::WorkgroupMemoryExplicitLayoutKHR
        }
        ffi::SPVBackCapability_SPVBackCapability_WorkgroupMemoryExplicitLayout8BitAccessKHR => {
            naga::back::spv::Capability::WorkgroupMemoryExplicitLayout8BitAccessKHR
        }
        ffi::SPVBackCapability_SPVBackCapability_WorkgroupMemoryExplicitLayout16BitAccessKHR => {
            naga::back::spv::Capability::WorkgroupMemoryExplicitLayout16BitAccessKHR
        }
        ffi::SPVBackCapability_SPVBackCapability_SubgroupVoteKHR => {
            naga::back::spv::Capability::SubgroupVoteKHR
        }
        ffi::SPVBackCapability_SPVBackCapability_StorageBuffer16BitAccess => {
            naga::back::spv::Capability::StorageBuffer16BitAccess
        }
        ffi::SPVBackCapability_SPVBackCapability_UniformAndStorageBuffer16BitAccess => {
            naga::back::spv::Capability::UniformAndStorageBuffer16BitAccess
        }
        ffi::SPVBackCapability_SPVBackCapability_StoragePushConstant16 => {
            naga::back::spv::Capability::StoragePushConstant16
        }
        ffi::SPVBackCapability_SPVBackCapability_StorageInputOutput16 => {
            naga::back::spv::Capability::StorageInputOutput16
        }
        ffi::SPVBackCapability_SPVBackCapability_DeviceGroup => {
            naga::back::spv::Capability::DeviceGroup
        }
        ffi::SPVBackCapability_SPVBackCapability_MultiView => {
            naga::back::spv::Capability::MultiView
        }
        ffi::SPVBackCapability_SPVBackCapability_VariablePointersStorageBuffer => {
            naga::back::spv::Capability::VariablePointersStorageBuffer
        }
        ffi::SPVBackCapability_SPVBackCapability_VariablePointers => {
            naga::back::spv::Capability::VariablePointers
        }
        ffi::SPVBackCapability_SPVBackCapability_AtomicStorageOps => {
            naga::back::spv::Capability::AtomicStorageOps
        }
        ffi::SPVBackCapability_SPVBackCapability_SampleMaskPostDepthCoverage => {
            naga::back::spv::Capability::SampleMaskPostDepthCoverage
        }
        ffi::SPVBackCapability_SPVBackCapability_StorageBuffer8BitAccess => {
            naga::back::spv::Capability::StorageBuffer8BitAccess
        }
        ffi::SPVBackCapability_SPVBackCapability_UniformAndStorageBuffer8BitAccess => {
            naga::back::spv::Capability::UniformAndStorageBuffer8BitAccess
        }
        ffi::SPVBackCapability_SPVBackCapability_StoragePushConstant8 => {
            naga::back::spv::Capability::StoragePushConstant8
        }
        ffi::SPVBackCapability_SPVBackCapability_DenormPreserve => {
            naga::back::spv::Capability::DenormPreserve
        }
        ffi::SPVBackCapability_SPVBackCapability_DenormFlushToZero => {
            naga::back::spv::Capability::DenormFlushToZero
        }
        ffi::SPVBackCapability_SPVBackCapability_SignedZeroInfNanPreserve => {
            naga::back::spv::Capability::SignedZeroInfNanPreserve
        }
        ffi::SPVBackCapability_SPVBackCapability_RoundingModeRTE => {
            naga::back::spv::Capability::RoundingModeRTE
        }
        ffi::SPVBackCapability_SPVBackCapability_RoundingModeRTZ => {
            naga::back::spv::Capability::RoundingModeRTZ
        }
        ffi::SPVBackCapability_SPVBackCapability_RayQueryProvisionalKHR => {
            naga::back::spv::Capability::RayQueryProvisionalKHR
        }
        ffi::SPVBackCapability_SPVBackCapability_RayQueryKHR => {
            naga::back::spv::Capability::RayQueryKHR
        }
        ffi::SPVBackCapability_SPVBackCapability_RayTraversalPrimitiveCullingKHR => {
            naga::back::spv::Capability::RayTraversalPrimitiveCullingKHR
        }
        ffi::SPVBackCapability_SPVBackCapability_RayTracingKHR => {
            naga::back::spv::Capability::RayTracingKHR
        }
        ffi::SPVBackCapability_SPVBackCapability_TextureSampleWeightedQCOM => {
            naga::back::spv::Capability::TextureSampleWeightedQCOM
        }
        ffi::SPVBackCapability_SPVBackCapability_TextureBoxFilterQCOM => {
            naga::back::spv::Capability::TextureBoxFilterQCOM
        }
        ffi::SPVBackCapability_SPVBackCapability_TextureBlockMatchQCOM => {
            naga::back::spv::Capability::TextureBlockMatchQCOM
        }
        ffi::SPVBackCapability_SPVBackCapability_Float16ImageAMD => {
            naga::back::spv::Capability::Float16ImageAMD
        }
        ffi::SPVBackCapability_SPVBackCapability_ImageGatherBiasLodAMD => {
            naga::back::spv::Capability::ImageGatherBiasLodAMD
        }
        ffi::SPVBackCapability_SPVBackCapability_FragmentMaskAMD => {
            naga::back::spv::Capability::FragmentMaskAMD
        }
        ffi::SPVBackCapability_SPVBackCapability_StencilExportEXT => {
            naga::back::spv::Capability::StencilExportEXT
        }
        ffi::SPVBackCapability_SPVBackCapability_ImageReadWriteLodAMD => {
            naga::back::spv::Capability::ImageReadWriteLodAMD
        }
        ffi::SPVBackCapability_SPVBackCapability_Int64ImageEXT => {
            naga::back::spv::Capability::Int64ImageEXT
        }
        ffi::SPVBackCapability_SPVBackCapability_ShaderClockKHR => {
            naga::back::spv::Capability::ShaderClockKHR
        }
        ffi::SPVBackCapability_SPVBackCapability_ShaderEnqueueAMDX => {
            naga::back::spv::Capability::ShaderEnqueueAMDX
        }
        ffi::SPVBackCapability_SPVBackCapability_SampleMaskOverrideCoverageNV => {
            naga::back::spv::Capability::SampleMaskOverrideCoverageNV
        }
        ffi::SPVBackCapability_SPVBackCapability_GeometryShaderPassthroughNV => {
            naga::back::spv::Capability::GeometryShaderPassthroughNV
        }
        ffi::SPVBackCapability_SPVBackCapability_ShaderViewportIndexLayerEXT => {
            naga::back::spv::Capability::ShaderViewportIndexLayerEXT
        }
        ffi::SPVBackCapability_SPVBackCapability_ShaderViewportMaskNV => {
            naga::back::spv::Capability::ShaderViewportMaskNV
        }
        ffi::SPVBackCapability_SPVBackCapability_ShaderStereoViewNV => {
            naga::back::spv::Capability::ShaderStereoViewNV
        }
        ffi::SPVBackCapability_SPVBackCapability_PerViewAttributesNV => {
            naga::back::spv::Capability::PerViewAttributesNV
        }
        ffi::SPVBackCapability_SPVBackCapability_FragmentFullyCoveredEXT => {
            naga::back::spv::Capability::FragmentFullyCoveredEXT
        }
        ffi::SPVBackCapability_SPVBackCapability_MeshShadingNV => {
            naga::back::spv::Capability::MeshShadingNV
        }
        ffi::SPVBackCapability_SPVBackCapability_ImageFootprintNV => {
            naga::back::spv::Capability::ImageFootprintNV
        }
        ffi::SPVBackCapability_SPVBackCapability_MeshShadingEXT => {
            naga::back::spv::Capability::MeshShadingEXT
        }
        ffi::SPVBackCapability_SPVBackCapability_FragmentBarycentricKHR => {
            naga::back::spv::Capability::FragmentBarycentricKHR
        }
        ffi::SPVBackCapability_SPVBackCapability_ComputeDerivativeGroupQuadsNV => {
            naga::back::spv::Capability::ComputeDerivativeGroupQuadsNV
        }
        ffi::SPVBackCapability_SPVBackCapability_FragmentDensityEXT => {
            naga::back::spv::Capability::FragmentDensityEXT
        }
        ffi::SPVBackCapability_SPVBackCapability_GroupNonUniformPartitionedNV => {
            naga::back::spv::Capability::GroupNonUniformPartitionedNV
        }
        ffi::SPVBackCapability_SPVBackCapability_ShaderNonUniform => {
            naga::back::spv::Capability::ShaderNonUniform
        }
        ffi::SPVBackCapability_SPVBackCapability_RuntimeDescriptorArray => {
            naga::back::spv::Capability::RuntimeDescriptorArray
        }
        ffi::SPVBackCapability_SPVBackCapability_InputAttachmentArrayDynamicIndexing => {
            naga::back::spv::Capability::InputAttachmentArrayDynamicIndexing
        }
        ffi::SPVBackCapability_SPVBackCapability_UniformTexelBufferArrayDynamicIndexing => {
            naga::back::spv::Capability::UniformTexelBufferArrayDynamicIndexing
        }
        ffi::SPVBackCapability_SPVBackCapability_StorageTexelBufferArrayDynamicIndexing => {
            naga::back::spv::Capability::StorageTexelBufferArrayDynamicIndexing
        }
        ffi::SPVBackCapability_SPVBackCapability_UniformBufferArrayNonUniformIndexing => {
            naga::back::spv::Capability::UniformBufferArrayNonUniformIndexing
        }
        ffi::SPVBackCapability_SPVBackCapability_SampledImageArrayNonUniformIndexing => {
            naga::back::spv::Capability::SampledImageArrayNonUniformIndexing
        }
        ffi::SPVBackCapability_SPVBackCapability_StorageBufferArrayNonUniformIndexing => {
            naga::back::spv::Capability::StorageBufferArrayNonUniformIndexing
        }
        ffi::SPVBackCapability_SPVBackCapability_StorageImageArrayNonUniformIndexing => {
            naga::back::spv::Capability::StorageImageArrayNonUniformIndexing
        }
        ffi::SPVBackCapability_SPVBackCapability_InputAttachmentArrayNonUniformIndexing => {
            naga::back::spv::Capability::InputAttachmentArrayNonUniformIndexing
        }
        ffi::SPVBackCapability_SPVBackCapability_UniformTexelBufferArrayNonUniformIndexing => {
            naga::back::spv::Capability::UniformTexelBufferArrayNonUniformIndexing
        }
        ffi::SPVBackCapability_SPVBackCapability_StorageTexelBufferArrayNonUniformIndexing => {
            naga::back::spv::Capability::StorageTexelBufferArrayNonUniformIndexing
        }
        ffi::SPVBackCapability_SPVBackCapability_RayTracingPositionFetchKHR => {
            naga::back::spv::Capability::RayTracingPositionFetchKHR
        }
        ffi::SPVBackCapability_SPVBackCapability_RayTracingNV => {
            naga::back::spv::Capability::RayTracingNV
        }
        ffi::SPVBackCapability_SPVBackCapability_RayTracingMotionBlurNV => {
            naga::back::spv::Capability::RayTracingMotionBlurNV
        }
        ffi::SPVBackCapability_SPVBackCapability_VulkanMemoryModel => {
            naga::back::spv::Capability::VulkanMemoryModel
        }
        ffi::SPVBackCapability_SPVBackCapability_VulkanMemoryModelDeviceScope => {
            naga::back::spv::Capability::VulkanMemoryModelDeviceScope
        }
        ffi::SPVBackCapability_SPVBackCapability_PhysicalStorageBufferAddresses => {
            naga::back::spv::Capability::PhysicalStorageBufferAddresses
        }
        ffi::SPVBackCapability_SPVBackCapability_ComputeDerivativeGroupLinearNV => {
            naga::back::spv::Capability::ComputeDerivativeGroupLinearNV
        }
        ffi::SPVBackCapability_SPVBackCapability_RayTracingProvisionalKHR => {
            naga::back::spv::Capability::RayTracingProvisionalKHR
        }
        ffi::SPVBackCapability_SPVBackCapability_CooperativeMatrixNV => {
            naga::back::spv::Capability::CooperativeMatrixNV
        }
        ffi::SPVBackCapability_SPVBackCapability_FragmentShaderSampleInterlockEXT => {
            naga::back::spv::Capability::FragmentShaderSampleInterlockEXT
        }
        ffi::SPVBackCapability_SPVBackCapability_FragmentShaderShadingRateInterlockEXT => {
            naga::back::spv::Capability::FragmentShaderShadingRateInterlockEXT
        }
        ffi::SPVBackCapability_SPVBackCapability_ShaderSMBuiltinsNV => {
            naga::back::spv::Capability::ShaderSMBuiltinsNV
        }
        ffi::SPVBackCapability_SPVBackCapability_FragmentShaderPixelInterlockEXT => {
            naga::back::spv::Capability::FragmentShaderPixelInterlockEXT
        }
        ffi::SPVBackCapability_SPVBackCapability_DemoteToHelperInvocation => {
            naga::back::spv::Capability::DemoteToHelperInvocation
        }
        ffi::SPVBackCapability_SPVBackCapability_DisplacementMicromapNV => {
            naga::back::spv::Capability::DisplacementMicromapNV
        }
        ffi::SPVBackCapability_SPVBackCapability_RayTracingOpacityMicromapEXT => {
            naga::back::spv::Capability::RayTracingOpacityMicromapEXT
        }
        ffi::SPVBackCapability_SPVBackCapability_ShaderInvocationReorderNV => {
            naga::back::spv::Capability::ShaderInvocationReorderNV
        }
        ffi::SPVBackCapability_SPVBackCapability_BindlessTextureNV => {
            naga::back::spv::Capability::BindlessTextureNV
        }
        ffi::SPVBackCapability_SPVBackCapability_RayQueryPositionFetchKHR => {
            naga::back::spv::Capability::RayQueryPositionFetchKHR
        }
        ffi::SPVBackCapability_SPVBackCapability_RayTracingDisplacementMicromapNV => {
            naga::back::spv::Capability::RayTracingDisplacementMicromapNV
        }
        ffi::SPVBackCapability_SPVBackCapability_SubgroupShuffleINTEL => {
            naga::back::spv::Capability::SubgroupShuffleINTEL
        }
        ffi::SPVBackCapability_SPVBackCapability_SubgroupBufferBlockIOINTEL => {
            naga::back::spv::Capability::SubgroupBufferBlockIOINTEL
        }
        ffi::SPVBackCapability_SPVBackCapability_SubgroupImageBlockIOINTEL => {
            naga::back::spv::Capability::SubgroupImageBlockIOINTEL
        }
        ffi::SPVBackCapability_SPVBackCapability_SubgroupImageMediaBlockIOINTEL => {
            naga::back::spv::Capability::SubgroupImageMediaBlockIOINTEL
        }
        ffi::SPVBackCapability_SPVBackCapability_RoundToInfinityINTEL => {
            naga::back::spv::Capability::RoundToInfinityINTEL
        }
        ffi::SPVBackCapability_SPVBackCapability_FloatingPointModeINTEL => {
            naga::back::spv::Capability::FloatingPointModeINTEL
        }
        ffi::SPVBackCapability_SPVBackCapability_IntegerFunctions2INTEL => {
            naga::back::spv::Capability::IntegerFunctions2INTEL
        }
        ffi::SPVBackCapability_SPVBackCapability_FunctionPointersINTEL => {
            naga::back::spv::Capability::FunctionPointersINTEL
        }
        ffi::SPVBackCapability_SPVBackCapability_IndirectReferencesINTEL => {
            naga::back::spv::Capability::IndirectReferencesINTEL
        }
        ffi::SPVBackCapability_SPVBackCapability_AsmINTEL => naga::back::spv::Capability::AsmINTEL,
        ffi::SPVBackCapability_SPVBackCapability_AtomicFloat32MinMaxEXT => {
            naga::back::spv::Capability::AtomicFloat32MinMaxEXT
        }
        ffi::SPVBackCapability_SPVBackCapability_AtomicFloat64MinMaxEXT => {
            naga::back::spv::Capability::AtomicFloat64MinMaxEXT
        }
        ffi::SPVBackCapability_SPVBackCapability_AtomicFloat16MinMaxEXT => {
            naga::back::spv::Capability::AtomicFloat16MinMaxEXT
        }
        ffi::SPVBackCapability_SPVBackCapability_VectorComputeINTEL => {
            naga::back::spv::Capability::VectorComputeINTEL
        }
        ffi::SPVBackCapability_SPVBackCapability_VectorAnyINTEL => {
            naga::back::spv::Capability::VectorAnyINTEL
        }
        ffi::SPVBackCapability_SPVBackCapability_ExpectAssumeKHR => {
            naga::back::spv::Capability::ExpectAssumeKHR
        }
        ffi::SPVBackCapability_SPVBackCapability_SubgroupAvcMotionEstimationINTEL => {
            naga::back::spv::Capability::SubgroupAvcMotionEstimationINTEL
        }
        ffi::SPVBackCapability_SPVBackCapability_SubgroupAvcMotionEstimationIntraINTEL => {
            naga::back::spv::Capability::SubgroupAvcMotionEstimationIntraINTEL
        }
        ffi::SPVBackCapability_SPVBackCapability_SubgroupAvcMotionEstimationChromaINTEL => {
            naga::back::spv::Capability::SubgroupAvcMotionEstimationChromaINTEL
        }
        ffi::SPVBackCapability_SPVBackCapability_VariableLengthArrayINTEL => {
            naga::back::spv::Capability::VariableLengthArrayINTEL
        }
        ffi::SPVBackCapability_SPVBackCapability_FunctionFloatControlINTEL => {
            naga::back::spv::Capability::FunctionFloatControlINTEL
        }
        ffi::SPVBackCapability_SPVBackCapability_FPGAMemoryAttributesINTEL => {
            naga::back::spv::Capability::FPGAMemoryAttributesINTEL
        }
        ffi::SPVBackCapability_SPVBackCapability_FPFastMathModeINTEL => {
            naga::back::spv::Capability::FPFastMathModeINTEL
        }
        ffi::SPVBackCapability_SPVBackCapability_ArbitraryPrecisionIntegersINTEL => {
            naga::back::spv::Capability::ArbitraryPrecisionIntegersINTEL
        }
        ffi::SPVBackCapability_SPVBackCapability_ArbitraryPrecisionFloatingPointINTEL => {
            naga::back::spv::Capability::ArbitraryPrecisionFloatingPointINTEL
        }
        ffi::SPVBackCapability_SPVBackCapability_UnstructuredLoopControlsINTEL => {
            naga::back::spv::Capability::UnstructuredLoopControlsINTEL
        }
        ffi::SPVBackCapability_SPVBackCapability_FPGALoopControlsINTEL => {
            naga::back::spv::Capability::FPGALoopControlsINTEL
        }
        ffi::SPVBackCapability_SPVBackCapability_KernelAttributesINTEL => {
            naga::back::spv::Capability::KernelAttributesINTEL
        }
        ffi::SPVBackCapability_SPVBackCapability_FPGAKernelAttributesINTEL => {
            naga::back::spv::Capability::FPGAKernelAttributesINTEL
        }
        ffi::SPVBackCapability_SPVBackCapability_FPGAMemoryAccessesINTEL => {
            naga::back::spv::Capability::FPGAMemoryAccessesINTEL
        }
        ffi::SPVBackCapability_SPVBackCapability_FPGAClusterAttributesINTEL => {
            naga::back::spv::Capability::FPGAClusterAttributesINTEL
        }
        ffi::SPVBackCapability_SPVBackCapability_LoopFuseINTEL => {
            naga::back::spv::Capability::LoopFuseINTEL
        }
        ffi::SPVBackCapability_SPVBackCapability_FPGADSPControlINTEL => {
            naga::back::spv::Capability::FPGADSPControlINTEL
        }
        ffi::SPVBackCapability_SPVBackCapability_MemoryAccessAliasingINTEL => {
            naga::back::spv::Capability::MemoryAccessAliasingINTEL
        }
        ffi::SPVBackCapability_SPVBackCapability_FPGAInvocationPipeliningAttributesINTEL => {
            naga::back::spv::Capability::FPGAInvocationPipeliningAttributesINTEL
        }
        ffi::SPVBackCapability_SPVBackCapability_FPGABufferLocationINTEL => {
            naga::back::spv::Capability::FPGABufferLocationINTEL
        }
        ffi::SPVBackCapability_SPVBackCapability_ArbitraryPrecisionFixedPointINTEL => {
            naga::back::spv::Capability::ArbitraryPrecisionFixedPointINTEL
        }
        ffi::SPVBackCapability_SPVBackCapability_USMStorageClassesINTEL => {
            naga::back::spv::Capability::USMStorageClassesINTEL
        }
        ffi::SPVBackCapability_SPVBackCapability_RuntimeAlignedAttributeINTEL => {
            naga::back::spv::Capability::RuntimeAlignedAttributeINTEL
        }
        ffi::SPVBackCapability_SPVBackCapability_IOPipesINTEL => {
            naga::back::spv::Capability::IOPipesINTEL
        }
        ffi::SPVBackCapability_SPVBackCapability_BlockingPipesINTEL => {
            naga::back::spv::Capability::BlockingPipesINTEL
        }
        ffi::SPVBackCapability_SPVBackCapability_FPGARegINTEL => {
            naga::back::spv::Capability::FPGARegINTEL
        }
        ffi::SPVBackCapability_SPVBackCapability_DotProductInputAll => {
            naga::back::spv::Capability::DotProductInputAll
        }
        ffi::SPVBackCapability_SPVBackCapability_DotProductInput4x8Bit => {
            naga::back::spv::Capability::DotProductInput4x8Bit
        }
        ffi::SPVBackCapability_SPVBackCapability_DotProductInput4x8BitPacked => {
            naga::back::spv::Capability::DotProductInput4x8BitPacked
        }
        ffi::SPVBackCapability_SPVBackCapability_DotProduct => {
            naga::back::spv::Capability::DotProduct
        }
        ffi::SPVBackCapability_SPVBackCapability_RayCullMaskKHR => {
            naga::back::spv::Capability::RayCullMaskKHR
        }
        ffi::SPVBackCapability_SPVBackCapability_CooperativeMatrixKHR => {
            naga::back::spv::Capability::CooperativeMatrixKHR
        }
        ffi::SPVBackCapability_SPVBackCapability_BitInstructions => {
            naga::back::spv::Capability::BitInstructions
        }
        ffi::SPVBackCapability_SPVBackCapability_GroupNonUniformRotateKHR => {
            naga::back::spv::Capability::GroupNonUniformRotateKHR
        }
        ffi::SPVBackCapability_SPVBackCapability_AtomicFloat32AddEXT => {
            naga::back::spv::Capability::AtomicFloat32AddEXT
        }
        ffi::SPVBackCapability_SPVBackCapability_AtomicFloat64AddEXT => {
            naga::back::spv::Capability::AtomicFloat64AddEXT
        }
        ffi::SPVBackCapability_SPVBackCapability_LongConstantCompositeINTEL => {
            naga::back::spv::Capability::LongConstantCompositeINTEL
        }
        ffi::SPVBackCapability_SPVBackCapability_OptNoneINTEL => {
            naga::back::spv::Capability::OptNoneINTEL
        }
        ffi::SPVBackCapability_SPVBackCapability_AtomicFloat16AddEXT => {
            naga::back::spv::Capability::AtomicFloat16AddEXT
        }
        ffi::SPVBackCapability_SPVBackCapability_DebugInfoModuleINTEL => {
            naga::back::spv::Capability::DebugInfoModuleINTEL
        }
        ffi::SPVBackCapability_SPVBackCapability_BFloat16ConversionINTEL => {
            naga::back::spv::Capability::BFloat16ConversionINTEL
        }
        ffi::SPVBackCapability_SPVBackCapability_SplitBarrierINTEL => {
            naga::back::spv::Capability::SplitBarrierINTEL
        }
        ffi::SPVBackCapability_SPVBackCapability_GlobalVariableFPGADecorationsINTEL => {
            naga::back::spv::Capability::GlobalVariableFPGADecorationsINTEL
        }
        ffi::SPVBackCapability_SPVBackCapability_FPGAKernelAttributesv2INTEL => {
            naga::back::spv::Capability::FPGAKernelAttributesv2INTEL
        }
        ffi::SPVBackCapability_SPVBackCapability_GlobalVariableHostAccessINTEL => {
            naga::back::spv::Capability::GlobalVariableHostAccessINTEL
        }
        ffi::SPVBackCapability_SPVBackCapability_FPMaxErrorINTEL => {
            naga::back::spv::Capability::FPMaxErrorINTEL
        }
        ffi::SPVBackCapability_SPVBackCapability_FPGALatencyControlINTEL => {
            naga::back::spv::Capability::FPGALatencyControlINTEL
        }
        ffi::SPVBackCapability_SPVBackCapability_FPGAArgumentInterfacesINTEL => {
            naga::back::spv::Capability::FPGAArgumentInterfacesINTEL
        }
        ffi::SPVBackCapability_SPVBackCapability_GroupUniformArithmeticKHR => {
            naga::back::spv::Capability::GroupUniformArithmeticKHR
        }
        ffi::SPVBackCapability_SPVBackCapability_CacheControlsINTEL => {
            naga::back::spv::Capability::CacheControlsINTEL
        }
        _ => panic!("Unknown SPVBackCapability"),
    }
}

pub fn spv_back_capability_set_to_naga(
    capability_set: &ffi::SPVBackCapabilitySet,
) -> naga::FastHashSet<naga::back::spv::Capability> {
    unsafe {
        std::slice::from_raw_parts(capability_set.capabilities, capability_set.capabilities_len)
            .iter()
            .map(|capability| spv_back_capability_to_naga(*capability))
            .collect()
    }
}

pub fn spv_back_writer_flags_to_naga(
    flags: ffi::SPVBackWriterFlags,
) -> naga::back::spv::WriterFlags {
    let mut result = naga::back::spv::WriterFlags::empty();

    if flags & ffi::SPVBackWriterFlags_SPVBackWriterFlags_DEBUG != 0 {
        result |= naga::back::spv::WriterFlags::DEBUG;
    }
    if flags & ffi::SPVBackWriterFlags_SPVBackWriterFlags_ADJUST_COORDINATE_SPACE != 0 {
        result |= naga::back::spv::WriterFlags::ADJUST_COORDINATE_SPACE;
    }
    if flags & ffi::SPVBackWriterFlags_SPVBackWriterFlags_LABEL_VARYINGS != 0 {
        result |= naga::back::spv::WriterFlags::LABEL_VARYINGS;
    }
    if flags & ffi::SPVBackWriterFlags_SPVBackWriterFlags_FORCE_POINT_SIZE != 0 {
        result |= naga::back::spv::WriterFlags::FORCE_POINT_SIZE;
    }
    if flags & ffi::SPVBackWriterFlags_SPVBackWriterFlags_CLAMP_FRAG_DEPTH != 0 {
        result |= naga::back::spv::WriterFlags::CLAMP_FRAG_DEPTH;
    }
    if flags & ffi::SPVBackWriterFlags_SPVBackWriterFlags_PRINT_ON_RAY_QUERY_INITIALIZATION_FAIL
        != 0
    {
        result |= naga::back::spv::WriterFlags::PRINT_ON_RAY_QUERY_INITIALIZATION_FAIL;
    }

    sa::const_assert_eq!(
        naga::back::spv::WriterFlags::all().bits(),
        naga::back::spv::WriterFlags::DEBUG.bits()
            | naga::back::spv::WriterFlags::ADJUST_COORDINATE_SPACE.bits()
            | naga::back::spv::WriterFlags::LABEL_VARYINGS.bits()
            | naga::back::spv::WriterFlags::FORCE_POINT_SIZE.bits()
            | naga::back::spv::WriterFlags::CLAMP_FRAG_DEPTH.bits()
            | naga::back::spv::WriterFlags::PRINT_ON_RAY_QUERY_INITIALIZATION_FAIL.bits()
    );

    result
}

pub fn spv_back_binding_info_to_naga(
    info: &ffi::SPVBackBindingInfo,
) -> naga::back::spv::BindingInfo {
    naga::back::spv::BindingInfo {
        descriptor_set: info.descriptor_set,
        binding: info.binding,
        binding_array_size: if bool_to_naga(info.binding_array_size.some) {
            Some(info.binding_array_size.value)
        } else {
            None
        },
    }
}

pub fn spv_back_binding_map_to_naga(map: &ffi::SPVBackBindingMap) -> naga::back::spv::BindingMap {
    unsafe {
        std::slice::from_raw_parts(map.entries, map.entries_len)
            .iter()
            .map(|entry| {
                (
                    resource_binding_to_naga(&entry.key),
                    spv_back_binding_info_to_naga(&entry.value),
                )
            })
            .collect()
    }
}

pub fn spv_back_zero_initialize_workgroup_memory_mode_to_naga(
    mode: ffi::SPVBackZeroInitializeWorkgroupMemoryMode,
) -> naga::back::spv::ZeroInitializeWorkgroupMemoryMode {
    match mode {
        ffi::SPVBackZeroInitializeWorkgroupMemoryMode_SPVBackZeroInitializeWorkgroupMemoryMode_Native => {
            naga::back::spv::ZeroInitializeWorkgroupMemoryMode::Native
        }
        ffi::SPVBackZeroInitializeWorkgroupMemoryMode_SPVBackZeroInitializeWorkgroupMemoryMode_Polyfill => {
            naga::back::spv::ZeroInitializeWorkgroupMemoryMode::Polyfill
        }
        ffi::SPVBackZeroInitializeWorkgroupMemoryMode_SPVBackZeroInitializeWorkgroupMemoryMode_None => {
            naga::back::spv::ZeroInitializeWorkgroupMemoryMode::None
        }
        _ => panic!("Unknown SPVBackZeroInitializeWorkgroupMemoryMode"),
    }
}

pub fn spv_back_source_language_to_naga(
    lang: ffi::SPVBackSourceLanguage,
) -> naga::back::spv::SourceLanguage {
    match lang {
        ffi::SPVBackSourceLanguage_SPVBackSourceLanguage_Unknown => {
            naga::back::spv::SourceLanguage::Unknown
        }
        ffi::SPVBackSourceLanguage_SPVBackSourceLanguage_ESSL => {
            naga::back::spv::SourceLanguage::ESSL
        }
        ffi::SPVBackSourceLanguage_SPVBackSourceLanguage_GLSL => {
            naga::back::spv::SourceLanguage::GLSL
        }
        ffi::SPVBackSourceLanguage_SPVBackSourceLanguage_OpenCL_C => {
            naga::back::spv::SourceLanguage::OpenCL_C
        }
        ffi::SPVBackSourceLanguage_SPVBackSourceLanguage_OpenCL_CPP => {
            naga::back::spv::SourceLanguage::OpenCL_CPP
        }
        ffi::SPVBackSourceLanguage_SPVBackSourceLanguage_HLSL => {
            naga::back::spv::SourceLanguage::HLSL
        }
        ffi::SPVBackSourceLanguage_SPVBackSourceLanguage_CPP_for_OpenCL => {
            naga::back::spv::SourceLanguage::CPP_for_OpenCL
        }
        ffi::SPVBackSourceLanguage_SPVBackSourceLanguage_SYCL => {
            naga::back::spv::SourceLanguage::SYCL
        }
        ffi::SPVBackSourceLanguage_SPVBackSourceLanguage_HERO_C => {
            naga::back::spv::SourceLanguage::HERO_C
        }
        ffi::SPVBackSourceLanguage_SPVBackSourceLanguage_NZSL => {
            naga::back::spv::SourceLanguage::NZSL
        }
        ffi::SPVBackSourceLanguage_SPVBackSourceLanguage_WGSL => {
            naga::back::spv::SourceLanguage::WGSL
        }
        ffi::SPVBackSourceLanguage_SPVBackSourceLanguage_Slang => {
            naga::back::spv::SourceLanguage::Slang
        }
        _ => panic!("Unknown SPVBackSourceLanguage"),
    }
}

pub fn spv_back_options_to_naga(options: &ffi::SPVBackOptions) -> naga::back::spv::Options<'_> {
    naga::back::spv::Options {
        lang_version: (options.lang_version[0], options.lang_version[1]),
        flags: spv_back_writer_flags_to_naga(options.flags),
        fake_missing_bindings: bool_to_naga(options.fake_missing_bindings),
        binding_map: spv_back_binding_map_to_naga(&options.binding_map),
        capabilities: if bool_to_naga(options.capabilities.some) {
            Some(spv_back_capability_set_to_naga(&options.capabilities.value))
        } else {
            None
        },
        bounds_check_policies: bound_check_policies_to_naga(&options.bounds_check_policies),
        zero_initialize_workgroup_memory: spv_back_zero_initialize_workgroup_memory_mode_to_naga(
            options.zero_initialize_workgroup_memory,
        ),
        force_loop_bounding: bool_to_naga(options.force_loop_bounding),
        ray_query_initialization_tracking: bool_to_naga(options.ray_query_initialization_tracking),
        use_storage_input_output_16: bool_to_naga(options.use_storage_input_output_16),
        debug_info: None,
    }
}

pub fn spv_back_pipeline_options_to_naga(
    options: &ffi::SPVBackPipelineOptions,
) -> naga::back::spv::PipelineOptions {
    naga::back::spv::PipelineOptions {
        shader_stage: shader_stage_to_naga(&options.shader_stage),
        entry_point: unsafe { string_to_naga(options.entry_point) },
    }
}

pub fn spv_back_error_to_ffi(error: &naga::back::spv::Error) -> ffi::SPVBackError {
    let default_data = ffi::SPVBackError__bindgen_ty_1::default();

    match error {
        naga::back::spv::Error::EntryPointNotFound => ffi::SPVBackError {
            tag: ffi::SPVBackErrorTag_SPVBackErrorTag_EntryPointNotFound,
            data: default_data,
        },
        naga::back::spv::Error::UnsupportedVersion(v0, v1) => ffi::SPVBackError {
            tag: ffi::SPVBackErrorTag_SPVBackErrorTag_UnsupportedVersion,
            data: ffi::SPVBackError__bindgen_ty_1 {
                unsupported_version: [*v0, *v1],
            },
        },
        naga::back::spv::Error::MissingCapabilities(error, capabilities) => ffi::SPVBackError {
            tag: ffi::SPVBackErrorTag_SPVBackErrorTag_MissingCapabilities,
            data: ffi::SPVBackError__bindgen_ty_1 {
                missing_capabilities: ffi::SPVBackError__bindgen_ty_1__bindgen_ty_1 {
                    error: unsafe { string_to_ffi(error) },
                    capabilities: unsafe {
                        slice_to_ffi(capabilities.as_slice(), spv_back_capability_to_ffi)
                    },
                    capabilities_len: capabilities.len() as u32,
                },
            },
        },
        naga::back::spv::Error::FeatureNotImplemented(feature) => ffi::SPVBackError {
            tag: ffi::SPVBackErrorTag_SPVBackErrorTag_FeatureNotImplemented,
            data: ffi::SPVBackError__bindgen_ty_1 {
                feature_not_implemented: unsafe { string_to_ffi(feature) },
            },
        },
        naga::back::spv::Error::Validation(error) => ffi::SPVBackError {
            tag: ffi::SPVBackErrorTag_SPVBackErrorTag_Validation,
            data: ffi::SPVBackError__bindgen_ty_1 {
                validation: unsafe { string_to_ffi(error) },
            },
        },
        naga::back::spv::Error::Override => ffi::SPVBackError {
            tag: ffi::SPVBackErrorTag_SPVBackErrorTag_Override,
            data: default_data,
        },
        naga::back::spv::Error::ResolveArraySizeError(resolve_array_size_error) => {
            ffi::SPVBackError {
                tag: ffi::SPVBackErrorTag_SPVBackErrorTag_ResolveArraySizeError,
                data: ffi::SPVBackError__bindgen_ty_1 {
                    resolve_array_size_error: resolve_array_size_error_to_ffi(
                        resolve_array_size_error,
                    ),
                },
            }
        }
        naga::back::spv::Error::SpirvVersionTooLow(v0, v1) => ffi::SPVBackError {
            tag: ffi::SPVBackErrorTag_SPVBackErrorTag_SpirvVersionTooLow,
            data: ffi::SPVBackError__bindgen_ty_1 {
                spirv_version_too_low: [*v0, *v1],
            },
        },
        naga::back::spv::Error::MissingBinding(resource_binding) => ffi::SPVBackError {
            tag: ffi::SPVBackErrorTag_SPVBackErrorTag_MissingBinding,
            data: ffi::SPVBackError__bindgen_ty_1 {
                missing_binding: resource_binding_to_ffi(resource_binding),
            },
        },
    }
}
