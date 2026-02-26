use super::*;

pub fn spv_back_capability_to_ffi(
    capability: &naga::back::spv::Capability,
) -> ffi::NagaSPVBackCapability {
    match capability {
        naga::back::spv::Capability::Matrix => ffi::NagaSPVBackCapability_NagaSPVBackCapability_Matrix,
        naga::back::spv::Capability::Shader => ffi::NagaSPVBackCapability_NagaSPVBackCapability_Shader,
        naga::back::spv::Capability::Geometry => ffi::NagaSPVBackCapability_NagaSPVBackCapability_Geometry,
        naga::back::spv::Capability::Tessellation => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_Tessellation
        }
        naga::back::spv::Capability::Addresses => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_Addresses
        }
        naga::back::spv::Capability::Linkage => ffi::NagaSPVBackCapability_NagaSPVBackCapability_Linkage,
        naga::back::spv::Capability::Kernel => ffi::NagaSPVBackCapability_NagaSPVBackCapability_Kernel,
        naga::back::spv::Capability::Vector16 => ffi::NagaSPVBackCapability_NagaSPVBackCapability_Vector16,
        naga::back::spv::Capability::Float16Buffer => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_Float16Buffer
        }
        naga::back::spv::Capability::Float16 => ffi::NagaSPVBackCapability_NagaSPVBackCapability_Float16,
        naga::back::spv::Capability::Float64 => ffi::NagaSPVBackCapability_NagaSPVBackCapability_Float64,
        naga::back::spv::Capability::Int64 => ffi::NagaSPVBackCapability_NagaSPVBackCapability_Int64,
        naga::back::spv::Capability::Int64Atomics => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_Int64Atomics
        }
        naga::back::spv::Capability::ImageBasic => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_ImageBasic
        }
        naga::back::spv::Capability::ImageReadWrite => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_ImageReadWrite
        }
        naga::back::spv::Capability::ImageMipmap => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_ImageMipmap
        }
        naga::back::spv::Capability::Pipes => ffi::NagaSPVBackCapability_NagaSPVBackCapability_Pipes,
        naga::back::spv::Capability::Groups => ffi::NagaSPVBackCapability_NagaSPVBackCapability_Groups,
        naga::back::spv::Capability::DeviceEnqueue => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_DeviceEnqueue
        }
        naga::back::spv::Capability::LiteralSampler => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_LiteralSampler
        }
        naga::back::spv::Capability::AtomicStorage => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_AtomicStorage
        }
        naga::back::spv::Capability::Int16 => ffi::NagaSPVBackCapability_NagaSPVBackCapability_Int16,
        naga::back::spv::Capability::TessellationPointSize => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_TessellationPointSize
        }
        naga::back::spv::Capability::GeometryPointSize => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_GeometryPointSize
        }
        naga::back::spv::Capability::ImageGatherExtended => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_ImageGatherExtended
        }
        naga::back::spv::Capability::StorageImageMultisample => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_StorageImageMultisample
        }
        naga::back::spv::Capability::UniformBufferArrayDynamicIndexing => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_UniformBufferArrayDynamicIndexing
        }
        naga::back::spv::Capability::SampledImageArrayDynamicIndexing => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_SampledImageArrayDynamicIndexing
        }
        naga::back::spv::Capability::StorageBufferArrayDynamicIndexing => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_StorageBufferArrayDynamicIndexing
        }
        naga::back::spv::Capability::StorageImageArrayDynamicIndexing => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_StorageImageArrayDynamicIndexing
        }
        naga::back::spv::Capability::ClipDistance => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_ClipDistance
        }
        naga::back::spv::Capability::CullDistance => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_CullDistance
        }
        naga::back::spv::Capability::ImageCubeArray => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_ImageCubeArray
        }
        naga::back::spv::Capability::SampleRateShading => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_SampleRateShading
        }
        naga::back::spv::Capability::ImageRect => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_ImageRect
        }
        naga::back::spv::Capability::SampledRect => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_SampledRect
        }
        naga::back::spv::Capability::GenericPointer => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_GenericPointer
        }
        naga::back::spv::Capability::Int8 => ffi::NagaSPVBackCapability_NagaSPVBackCapability_Int8,
        naga::back::spv::Capability::InputAttachment => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_InputAttachment
        }
        naga::back::spv::Capability::SparseResidency => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_SparseResidency
        }
        naga::back::spv::Capability::MinLod => ffi::NagaSPVBackCapability_NagaSPVBackCapability_MinLod,
        naga::back::spv::Capability::Sampled1D => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_Sampled1D
        }
        naga::back::spv::Capability::Image1D => ffi::NagaSPVBackCapability_NagaSPVBackCapability_Image1D,
        naga::back::spv::Capability::SampledCubeArray => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_SampledCubeArray
        }
        naga::back::spv::Capability::SampledBuffer => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_SampledBuffer
        }
        naga::back::spv::Capability::ImageBuffer => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_ImageBuffer
        }
        naga::back::spv::Capability::ImageMSArray => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_ImageMSArray
        }
        naga::back::spv::Capability::StorageImageExtendedFormats => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_StorageImageExtendedFormats
        }
        naga::back::spv::Capability::ImageQuery => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_ImageQuery
        }
        naga::back::spv::Capability::DerivativeControl => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_DerivativeControl
        }
        naga::back::spv::Capability::InterpolationFunction => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_InterpolationFunction
        }
        naga::back::spv::Capability::TransformFeedback => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_TransformFeedback
        }
        naga::back::spv::Capability::GeometryStreams => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_GeometryStreams
        }
        naga::back::spv::Capability::StorageImageReadWithoutFormat => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_StorageImageReadWithoutFormat
        }
        naga::back::spv::Capability::StorageImageWriteWithoutFormat => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_StorageImageWriteWithoutFormat
        }
        naga::back::spv::Capability::MultiViewport => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_MultiViewport
        }
        naga::back::spv::Capability::SubgroupDispatch => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_SubgroupDispatch
        }
        naga::back::spv::Capability::NamedBarrier => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_NamedBarrier
        }
        naga::back::spv::Capability::PipeStorage => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_PipeStorage
        }
        naga::back::spv::Capability::GroupNonUniform => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_GroupNonUniform
        }
        naga::back::spv::Capability::GroupNonUniformVote => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_GroupNonUniformVote
        }
        naga::back::spv::Capability::GroupNonUniformArithmetic => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_GroupNonUniformArithmetic
        }
        naga::back::spv::Capability::GroupNonUniformBallot => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_GroupNonUniformBallot
        }
        naga::back::spv::Capability::GroupNonUniformShuffle => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_GroupNonUniformShuffle
        }
        naga::back::spv::Capability::GroupNonUniformShuffleRelative => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_GroupNonUniformShuffleRelative
        }
        naga::back::spv::Capability::GroupNonUniformClustered => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_GroupNonUniformClustered
        }
        naga::back::spv::Capability::GroupNonUniformQuad => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_GroupNonUniformQuad
        }
        naga::back::spv::Capability::ShaderLayer => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_ShaderLayer
        }
        naga::back::spv::Capability::ShaderViewportIndex => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_ShaderViewportIndex
        }
        naga::back::spv::Capability::UniformDecoration => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_UniformDecoration
        }
        naga::back::spv::Capability::CoreBuiltinsARM => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_CoreBuiltinsARM
        }
        naga::back::spv::Capability::TileImageColorReadAccessEXT => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_TileImageColorReadAccessEXT
        }
        naga::back::spv::Capability::TileImageDepthReadAccessEXT => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_TileImageDepthReadAccessEXT
        }
        naga::back::spv::Capability::TileImageStencilReadAccessEXT => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_TileImageStencilReadAccessEXT
        }
        naga::back::spv::Capability::FragmentShadingRateKHR => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_FragmentShadingRateKHR
        }
        naga::back::spv::Capability::SubgroupBallotKHR => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_SubgroupBallotKHR
        }
        naga::back::spv::Capability::DrawParameters => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_DrawParameters
        }
        naga::back::spv::Capability::WorkgroupMemoryExplicitLayoutKHR => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_WorkgroupMemoryExplicitLayoutKHR
        }
        naga::back::spv::Capability::WorkgroupMemoryExplicitLayout8BitAccessKHR => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_WorkgroupMemoryExplicitLayout8BitAccessKHR
        }
        naga::back::spv::Capability::WorkgroupMemoryExplicitLayout16BitAccessKHR => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_WorkgroupMemoryExplicitLayout16BitAccessKHR
        }
        naga::back::spv::Capability::SubgroupVoteKHR => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_SubgroupVoteKHR
        }
        naga::back::spv::Capability::StorageBuffer16BitAccess => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_StorageBuffer16BitAccess
        }
        naga::back::spv::Capability::UniformAndStorageBuffer16BitAccess => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_UniformAndStorageBuffer16BitAccess
        }
        naga::back::spv::Capability::StoragePushConstant16 => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_StoragePushConstant16
        }
        naga::back::spv::Capability::StorageInputOutput16 => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_StorageInputOutput16
        }
        naga::back::spv::Capability::DeviceGroup => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_DeviceGroup
        }
        naga::back::spv::Capability::MultiView => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_MultiView
        }
        naga::back::spv::Capability::VariablePointersStorageBuffer => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_VariablePointersStorageBuffer
        }
        naga::back::spv::Capability::VariablePointers => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_VariablePointers
        }
        naga::back::spv::Capability::AtomicStorageOps => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_AtomicStorageOps
        }
        naga::back::spv::Capability::SampleMaskPostDepthCoverage => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_SampleMaskPostDepthCoverage
        }
        naga::back::spv::Capability::StorageBuffer8BitAccess => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_StorageBuffer8BitAccess
        }
        naga::back::spv::Capability::UniformAndStorageBuffer8BitAccess => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_UniformAndStorageBuffer8BitAccess
        }
        naga::back::spv::Capability::StoragePushConstant8 => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_StoragePushConstant8
        }
        naga::back::spv::Capability::DenormPreserve => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_DenormPreserve
        }
        naga::back::spv::Capability::DenormFlushToZero => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_DenormFlushToZero
        }
        naga::back::spv::Capability::SignedZeroInfNanPreserve => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_SignedZeroInfNanPreserve
        }
        naga::back::spv::Capability::RoundingModeRTE => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_RoundingModeRTE
        }
        naga::back::spv::Capability::RoundingModeRTZ => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_RoundingModeRTZ
        }
        naga::back::spv::Capability::RayQueryProvisionalKHR => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_RayQueryProvisionalKHR
        }
        naga::back::spv::Capability::RayQueryKHR => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_RayQueryKHR
        }
        naga::back::spv::Capability::RayTraversalPrimitiveCullingKHR => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_RayTraversalPrimitiveCullingKHR
        }
        naga::back::spv::Capability::RayTracingKHR => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_RayTracingKHR
        }
        naga::back::spv::Capability::TextureSampleWeightedQCOM => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_TextureSampleWeightedQCOM
        }
        naga::back::spv::Capability::TextureBoxFilterQCOM => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_TextureBoxFilterQCOM
        }
        naga::back::spv::Capability::TextureBlockMatchQCOM => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_TextureBlockMatchQCOM
        }
        naga::back::spv::Capability::Float16ImageAMD => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_Float16ImageAMD
        }
        naga::back::spv::Capability::ImageGatherBiasLodAMD => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_ImageGatherBiasLodAMD
        }
        naga::back::spv::Capability::FragmentMaskAMD => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_FragmentMaskAMD
        }
        naga::back::spv::Capability::StencilExportEXT => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_StencilExportEXT
        }
        naga::back::spv::Capability::ImageReadWriteLodAMD => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_ImageReadWriteLodAMD
        }
        naga::back::spv::Capability::Int64ImageEXT => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_Int64ImageEXT
        }
        naga::back::spv::Capability::ShaderClockKHR => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_ShaderClockKHR
        }
        naga::back::spv::Capability::ShaderEnqueueAMDX => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_ShaderEnqueueAMDX
        }
        naga::back::spv::Capability::SampleMaskOverrideCoverageNV => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_SampleMaskOverrideCoverageNV
        }
        naga::back::spv::Capability::GeometryShaderPassthroughNV => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_GeometryShaderPassthroughNV
        }
        naga::back::spv::Capability::ShaderViewportIndexLayerEXT => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_ShaderViewportIndexLayerEXT
        }
        naga::back::spv::Capability::ShaderViewportMaskNV => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_ShaderViewportMaskNV
        }
        naga::back::spv::Capability::ShaderStereoViewNV => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_ShaderStereoViewNV
        }
        naga::back::spv::Capability::PerViewAttributesNV => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_PerViewAttributesNV
        }
        naga::back::spv::Capability::FragmentFullyCoveredEXT => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_FragmentFullyCoveredEXT
        }
        naga::back::spv::Capability::MeshShadingNV => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_MeshShadingNV
        }
        naga::back::spv::Capability::ImageFootprintNV => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_ImageFootprintNV
        }
        naga::back::spv::Capability::MeshShadingEXT => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_MeshShadingEXT
        }
        naga::back::spv::Capability::FragmentBarycentricKHR => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_FragmentBarycentricKHR
        }
        naga::back::spv::Capability::FragmentDensityEXT => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_FragmentDensityEXT
        }
        naga::back::spv::Capability::GroupNonUniformPartitionedNV => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_GroupNonUniformPartitionedNV
        }
        naga::back::spv::Capability::ShaderNonUniform => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_ShaderNonUniform
        }
        naga::back::spv::Capability::RuntimeDescriptorArray => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_RuntimeDescriptorArray
        }
        naga::back::spv::Capability::InputAttachmentArrayDynamicIndexing => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_InputAttachmentArrayDynamicIndexing
        }
        naga::back::spv::Capability::UniformTexelBufferArrayDynamicIndexing => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_UniformTexelBufferArrayDynamicIndexing
        }
        naga::back::spv::Capability::StorageTexelBufferArrayDynamicIndexing => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_StorageTexelBufferArrayDynamicIndexing
        }
        naga::back::spv::Capability::UniformBufferArrayNonUniformIndexing => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_UniformBufferArrayNonUniformIndexing
        }
        naga::back::spv::Capability::SampledImageArrayNonUniformIndexing => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_SampledImageArrayNonUniformIndexing
        }
        naga::back::spv::Capability::StorageBufferArrayNonUniformIndexing => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_StorageBufferArrayNonUniformIndexing
        }
        naga::back::spv::Capability::StorageImageArrayNonUniformIndexing => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_StorageImageArrayNonUniformIndexing
        }
        naga::back::spv::Capability::InputAttachmentArrayNonUniformIndexing => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_InputAttachmentArrayNonUniformIndexing
        }
        naga::back::spv::Capability::UniformTexelBufferArrayNonUniformIndexing => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_UniformTexelBufferArrayNonUniformIndexing
        }
        naga::back::spv::Capability::StorageTexelBufferArrayNonUniformIndexing => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_StorageTexelBufferArrayNonUniformIndexing
        }
        naga::back::spv::Capability::RayTracingPositionFetchKHR => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_RayTracingPositionFetchKHR
        }
        naga::back::spv::Capability::RayTracingNV => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_RayTracingNV
        }
        naga::back::spv::Capability::RayTracingMotionBlurNV => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_RayTracingMotionBlurNV
        }
        naga::back::spv::Capability::VulkanMemoryModel => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_VulkanMemoryModel
        }
        naga::back::spv::Capability::VulkanMemoryModelDeviceScope => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_VulkanMemoryModelDeviceScope
        }
        naga::back::spv::Capability::PhysicalStorageBufferAddresses => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_PhysicalStorageBufferAddresses
        }
        naga::back::spv::Capability::RayTracingProvisionalKHR => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_RayTracingProvisionalKHR
        }
        naga::back::spv::Capability::CooperativeMatrixNV => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_CooperativeMatrixNV
        }
        naga::back::spv::Capability::FragmentShaderSampleInterlockEXT => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_FragmentShaderSampleInterlockEXT
        }
        naga::back::spv::Capability::FragmentShaderShadingRateInterlockEXT => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_FragmentShaderShadingRateInterlockEXT
        }
        naga::back::spv::Capability::ShaderSMBuiltinsNV => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_ShaderSMBuiltinsNV
        }
        naga::back::spv::Capability::FragmentShaderPixelInterlockEXT => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_FragmentShaderPixelInterlockEXT
        }
        naga::back::spv::Capability::DemoteToHelperInvocation => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_DemoteToHelperInvocation
        }
        naga::back::spv::Capability::DisplacementMicromapNV => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_DisplacementMicromapNV
        }
        naga::back::spv::Capability::RayTracingOpacityMicromapEXT => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_RayTracingOpacityMicromapEXT
        }
        naga::back::spv::Capability::ShaderInvocationReorderNV => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_ShaderInvocationReorderNV
        }
        naga::back::spv::Capability::BindlessTextureNV => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_BindlessTextureNV
        }
        naga::back::spv::Capability::RayQueryPositionFetchKHR => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_RayQueryPositionFetchKHR
        }
        naga::back::spv::Capability::RayTracingDisplacementMicromapNV => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_RayTracingDisplacementMicromapNV
        }
        naga::back::spv::Capability::SubgroupShuffleINTEL => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_SubgroupShuffleINTEL
        }
        naga::back::spv::Capability::SubgroupBufferBlockIOINTEL => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_SubgroupBufferBlockIOINTEL
        }
        naga::back::spv::Capability::SubgroupImageBlockIOINTEL => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_SubgroupImageBlockIOINTEL
        }
        naga::back::spv::Capability::SubgroupImageMediaBlockIOINTEL => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_SubgroupImageMediaBlockIOINTEL
        }
        naga::back::spv::Capability::RoundToInfinityINTEL => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_RoundToInfinityINTEL
        }
        naga::back::spv::Capability::FloatingPointModeINTEL => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_FloatingPointModeINTEL
        }
        naga::back::spv::Capability::IntegerFunctions2INTEL => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_IntegerFunctions2INTEL
        }
        naga::back::spv::Capability::FunctionPointersINTEL => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_FunctionPointersINTEL
        }
        naga::back::spv::Capability::IndirectReferencesINTEL => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_IndirectReferencesINTEL
        }
        naga::back::spv::Capability::AsmINTEL => ffi::NagaSPVBackCapability_NagaSPVBackCapability_AsmINTEL,
        naga::back::spv::Capability::AtomicFloat32MinMaxEXT => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_AtomicFloat32MinMaxEXT
        }
        naga::back::spv::Capability::AtomicFloat64MinMaxEXT => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_AtomicFloat64MinMaxEXT
        }
        naga::back::spv::Capability::AtomicFloat16MinMaxEXT => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_AtomicFloat16MinMaxEXT
        }
        naga::back::spv::Capability::VectorComputeINTEL => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_VectorComputeINTEL
        }
        naga::back::spv::Capability::VectorAnyINTEL => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_VectorAnyINTEL
        }
        naga::back::spv::Capability::ExpectAssumeKHR => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_ExpectAssumeKHR
        }
        naga::back::spv::Capability::SubgroupAvcMotionEstimationINTEL => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_SubgroupAvcMotionEstimationINTEL
        }
        naga::back::spv::Capability::SubgroupAvcMotionEstimationIntraINTEL => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_SubgroupAvcMotionEstimationIntraINTEL
        }
        naga::back::spv::Capability::SubgroupAvcMotionEstimationChromaINTEL => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_SubgroupAvcMotionEstimationChromaINTEL
        }
        naga::back::spv::Capability::VariableLengthArrayINTEL => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_VariableLengthArrayINTEL
        }
        naga::back::spv::Capability::FunctionFloatControlINTEL => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_FunctionFloatControlINTEL
        }
        naga::back::spv::Capability::FPGAMemoryAttributesINTEL => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_FPGAMemoryAttributesINTEL
        }
        naga::back::spv::Capability::FPFastMathModeINTEL => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_FPFastMathModeINTEL
        }
        naga::back::spv::Capability::ArbitraryPrecisionIntegersINTEL => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_ArbitraryPrecisionIntegersINTEL
        }
        naga::back::spv::Capability::ArbitraryPrecisionFloatingPointINTEL => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_ArbitraryPrecisionFloatingPointINTEL
        }
        naga::back::spv::Capability::UnstructuredLoopControlsINTEL => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_UnstructuredLoopControlsINTEL
        }
        naga::back::spv::Capability::FPGALoopControlsINTEL => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_FPGALoopControlsINTEL
        }
        naga::back::spv::Capability::KernelAttributesINTEL => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_KernelAttributesINTEL
        }
        naga::back::spv::Capability::FPGAKernelAttributesINTEL => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_FPGAKernelAttributesINTEL
        }
        naga::back::spv::Capability::FPGAMemoryAccessesINTEL => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_FPGAMemoryAccessesINTEL
        }
        naga::back::spv::Capability::FPGAClusterAttributesINTEL => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_FPGAClusterAttributesINTEL
        }
        naga::back::spv::Capability::LoopFuseINTEL => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_LoopFuseINTEL
        }
        naga::back::spv::Capability::FPGADSPControlINTEL => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_FPGADSPControlINTEL
        }
        naga::back::spv::Capability::MemoryAccessAliasingINTEL => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_MemoryAccessAliasingINTEL
        }
        naga::back::spv::Capability::FPGAInvocationPipeliningAttributesINTEL => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_FPGAInvocationPipeliningAttributesINTEL
        }
        naga::back::spv::Capability::FPGABufferLocationINTEL => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_FPGABufferLocationINTEL
        }
        naga::back::spv::Capability::ArbitraryPrecisionFixedPointINTEL => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_ArbitraryPrecisionFixedPointINTEL
        }
        naga::back::spv::Capability::USMStorageClassesINTEL => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_USMStorageClassesINTEL
        }
        naga::back::spv::Capability::RuntimeAlignedAttributeINTEL => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_RuntimeAlignedAttributeINTEL
        }
        naga::back::spv::Capability::IOPipesINTEL => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_IOPipesINTEL
        }
        naga::back::spv::Capability::BlockingPipesINTEL => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_BlockingPipesINTEL
        }
        naga::back::spv::Capability::FPGARegINTEL => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_FPGARegINTEL
        }
        naga::back::spv::Capability::DotProductInputAll => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_DotProductInputAll
        }
        naga::back::spv::Capability::DotProductInput4x8Bit => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_DotProductInput4x8Bit
        }
        naga::back::spv::Capability::DotProductInput4x8BitPacked => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_DotProductInput4x8BitPacked
        }
        naga::back::spv::Capability::DotProduct => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_DotProduct
        }
        naga::back::spv::Capability::RayCullMaskKHR => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_RayCullMaskKHR
        }
        naga::back::spv::Capability::CooperativeMatrixKHR => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_CooperativeMatrixKHR
        }
        naga::back::spv::Capability::BitInstructions => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_BitInstructions
        }
        naga::back::spv::Capability::GroupNonUniformRotateKHR => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_GroupNonUniformRotateKHR
        }
        naga::back::spv::Capability::AtomicFloat32AddEXT => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_AtomicFloat32AddEXT
        }
        naga::back::spv::Capability::AtomicFloat64AddEXT => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_AtomicFloat64AddEXT
        }
        naga::back::spv::Capability::AtomicFloat16AddEXT => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_AtomicFloat16AddEXT
        }
        naga::back::spv::Capability::DebugInfoModuleINTEL => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_DebugInfoModuleINTEL
        }
        naga::back::spv::Capability::BFloat16ConversionINTEL => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_BFloat16ConversionINTEL
        }
        naga::back::spv::Capability::SplitBarrierINTEL => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_SplitBarrierINTEL
        }
        naga::back::spv::Capability::GlobalVariableFPGADecorationsINTEL => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_GlobalVariableFPGADecorationsINTEL
        }
        naga::back::spv::Capability::FPGAKernelAttributesv2INTEL => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_FPGAKernelAttributesv2INTEL
        }
        naga::back::spv::Capability::GlobalVariableHostAccessINTEL => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_GlobalVariableHostAccessINTEL
        }
        naga::back::spv::Capability::FPMaxErrorINTEL => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_FPMaxErrorINTEL
        }
        naga::back::spv::Capability::FPGALatencyControlINTEL => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_FPGALatencyControlINTEL
        }
        naga::back::spv::Capability::FPGAArgumentInterfacesINTEL => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_FPGAArgumentInterfacesINTEL
        }
        naga::back::spv::Capability::GroupUniformArithmeticKHR => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_GroupUniformArithmeticKHR
        }
        naga::back::spv::Capability::CacheControlsINTEL => {
            ffi::NagaSPVBackCapability_NagaSPVBackCapability_CacheControlsINTEL
        }
        naga::back::spv::Capability::CooperativeMatrixLayoutsARM => todo!(),
        naga::back::spv::Capability::UntypedPointersKHR => todo!(),
        naga::back::spv::Capability::TextureBlockMatch2QCOM => todo!(),
        naga::back::spv::Capability::QuadControlKHR => todo!(),
        naga::back::spv::Capability::ComputeDerivativeGroupQuadsKHR => todo!(),
        naga::back::spv::Capability::ComputeDerivativeGroupLinearKHR => todo!(),
        naga::back::spv::Capability::CooperativeVectorNV => todo!(),
        naga::back::spv::Capability::AtomicFloat16VectorNV => todo!(),
        naga::back::spv::Capability::RawAccessChainsNV => todo!(),
        naga::back::spv::Capability::RayTracingSpheresGeometryNV => todo!(),
        naga::back::spv::Capability::RayTracingLinearSweptSpheresGeometryNV => todo!(),
        naga::back::spv::Capability::CooperativeMatrixReductionsNV => todo!(),
        naga::back::spv::Capability::CooperativeMatrixConversionsNV => todo!(),
        naga::back::spv::Capability::CooperativeMatrixPerElementOperationsNV => todo!(),
        naga::back::spv::Capability::CooperativeMatrixTensorAddressingNV => todo!(),
        naga::back::spv::Capability::CooperativeMatrixBlockLoadsNV => todo!(),
        naga::back::spv::Capability::CooperativeVectorTrainingNV => todo!(),
        naga::back::spv::Capability::RayTracingClusterAccelerationStructureNV => todo!(),
        naga::back::spv::Capability::TensorAddressingNV => todo!(),
        naga::back::spv::Capability::ReplicatedCompositesEXT => todo!(),
        naga::back::spv::Capability::FloatControls2 => todo!(),
        naga::back::spv::Capability::LongCompositesINTEL => todo!(),
        naga::back::spv::Capability::OptNoneEXT => todo!(),
        naga::back::spv::Capability::ArithmeticFenceEXT => todo!(),
        naga::back::spv::Capability::FPGAClusterAttributesV2INTEL => todo!(),
        naga::back::spv::Capability::SubgroupBufferPrefetchINTEL => todo!(),
        naga::back::spv::Capability::Subgroup2DBlockIOINTEL => todo!(),
        naga::back::spv::Capability::Subgroup2DBlockTransformINTEL => todo!(),
        naga::back::spv::Capability::Subgroup2DBlockTransposeINTEL => todo!(),
        naga::back::spv::Capability::SubgroupMatrixMultiplyAccumulateINTEL => todo!(),
        naga::back::spv::Capability::MaskedGatherScatterINTEL => todo!(),
        naga::back::spv::Capability::RegisterLimitsINTEL => todo!(),
    }
}

pub fn spv_back_capability_to_naga(
    capability: ffi::NagaSPVBackCapability,
) -> naga::back::spv::Capability {
    match capability {
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_Matrix => naga::back::spv::Capability::Matrix,
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_Shader => naga::back::spv::Capability::Shader,
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_Geometry => naga::back::spv::Capability::Geometry,
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_Tessellation => {
            naga::back::spv::Capability::Tessellation
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_Addresses => {
            naga::back::spv::Capability::Addresses
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_Linkage => naga::back::spv::Capability::Linkage,
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_Kernel => naga::back::spv::Capability::Kernel,
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_Vector16 => naga::back::spv::Capability::Vector16,
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_Float16Buffer => {
            naga::back::spv::Capability::Float16Buffer
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_Float16 => naga::back::spv::Capability::Float16,
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_Float64 => naga::back::spv::Capability::Float64,
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_Int64 => naga::back::spv::Capability::Int64,
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_Int64Atomics => {
            naga::back::spv::Capability::Int64Atomics
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_ImageBasic => {
            naga::back::spv::Capability::ImageBasic
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_ImageReadWrite => {
            naga::back::spv::Capability::ImageReadWrite
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_ImageMipmap => {
            naga::back::spv::Capability::ImageMipmap
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_Pipes => naga::back::spv::Capability::Pipes,
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_Groups => naga::back::spv::Capability::Groups,
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_DeviceEnqueue => {
            naga::back::spv::Capability::DeviceEnqueue
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_LiteralSampler => {
            naga::back::spv::Capability::LiteralSampler
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_AtomicStorage => {
            naga::back::spv::Capability::AtomicStorage
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_Int16 => naga::back::spv::Capability::Int16,
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_TessellationPointSize => {
            naga::back::spv::Capability::TessellationPointSize
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_GeometryPointSize => {
            naga::back::spv::Capability::GeometryPointSize
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_ImageGatherExtended => {
            naga::back::spv::Capability::ImageGatherExtended
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_StorageImageMultisample => {
            naga::back::spv::Capability::StorageImageMultisample
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_UniformBufferArrayDynamicIndexing => {
            naga::back::spv::Capability::UniformBufferArrayDynamicIndexing
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_SampledImageArrayDynamicIndexing => {
            naga::back::spv::Capability::SampledImageArrayDynamicIndexing
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_StorageBufferArrayDynamicIndexing => {
            naga::back::spv::Capability::StorageBufferArrayDynamicIndexing
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_StorageImageArrayDynamicIndexing => {
            naga::back::spv::Capability::StorageImageArrayDynamicIndexing
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_ClipDistance => {
            naga::back::spv::Capability::ClipDistance
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_CullDistance => {
            naga::back::spv::Capability::CullDistance
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_ImageCubeArray => {
            naga::back::spv::Capability::ImageCubeArray
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_SampleRateShading => {
            naga::back::spv::Capability::SampleRateShading
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_ImageRect => {
            naga::back::spv::Capability::ImageRect
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_SampledRect => {
            naga::back::spv::Capability::SampledRect
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_GenericPointer => {
            naga::back::spv::Capability::GenericPointer
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_Int8 => naga::back::spv::Capability::Int8,
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_InputAttachment => {
            naga::back::spv::Capability::InputAttachment
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_SparseResidency => {
            naga::back::spv::Capability::SparseResidency
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_MinLod => naga::back::spv::Capability::MinLod,
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_Sampled1D => {
            naga::back::spv::Capability::Sampled1D
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_Image1D => naga::back::spv::Capability::Image1D,
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_SampledCubeArray => {
            naga::back::spv::Capability::SampledCubeArray
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_SampledBuffer => {
            naga::back::spv::Capability::SampledBuffer
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_ImageBuffer => {
            naga::back::spv::Capability::ImageBuffer
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_ImageMSArray => {
            naga::back::spv::Capability::ImageMSArray
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_StorageImageExtendedFormats => {
            naga::back::spv::Capability::StorageImageExtendedFormats
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_ImageQuery => {
            naga::back::spv::Capability::ImageQuery
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_DerivativeControl => {
            naga::back::spv::Capability::DerivativeControl
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_InterpolationFunction => {
            naga::back::spv::Capability::InterpolationFunction
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_TransformFeedback => {
            naga::back::spv::Capability::TransformFeedback
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_GeometryStreams => {
            naga::back::spv::Capability::GeometryStreams
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_StorageImageReadWithoutFormat => {
            naga::back::spv::Capability::StorageImageReadWithoutFormat
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_StorageImageWriteWithoutFormat => {
            naga::back::spv::Capability::StorageImageWriteWithoutFormat
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_MultiViewport => {
            naga::back::spv::Capability::MultiViewport
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_SubgroupDispatch => {
            naga::back::spv::Capability::SubgroupDispatch
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_NamedBarrier => {
            naga::back::spv::Capability::NamedBarrier
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_PipeStorage => {
            naga::back::spv::Capability::PipeStorage
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_GroupNonUniform => {
            naga::back::spv::Capability::GroupNonUniform
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_GroupNonUniformVote => {
            naga::back::spv::Capability::GroupNonUniformVote
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_GroupNonUniformArithmetic => {
            naga::back::spv::Capability::GroupNonUniformArithmetic
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_GroupNonUniformBallot => {
            naga::back::spv::Capability::GroupNonUniformBallot
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_GroupNonUniformShuffle => {
            naga::back::spv::Capability::GroupNonUniformShuffle
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_GroupNonUniformShuffleRelative => {
            naga::back::spv::Capability::GroupNonUniformShuffleRelative
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_GroupNonUniformClustered => {
            naga::back::spv::Capability::GroupNonUniformClustered
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_GroupNonUniformQuad => {
            naga::back::spv::Capability::GroupNonUniformQuad
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_ShaderLayer => {
            naga::back::spv::Capability::ShaderLayer
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_ShaderViewportIndex => {
            naga::back::spv::Capability::ShaderViewportIndex
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_UniformDecoration => {
            naga::back::spv::Capability::UniformDecoration
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_CoreBuiltinsARM => {
            naga::back::spv::Capability::CoreBuiltinsARM
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_TileImageColorReadAccessEXT => {
            naga::back::spv::Capability::TileImageColorReadAccessEXT
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_TileImageDepthReadAccessEXT => {
            naga::back::spv::Capability::TileImageDepthReadAccessEXT
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_TileImageStencilReadAccessEXT => {
            naga::back::spv::Capability::TileImageStencilReadAccessEXT
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_FragmentShadingRateKHR => {
            naga::back::spv::Capability::FragmentShadingRateKHR
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_SubgroupBallotKHR => {
            naga::back::spv::Capability::SubgroupBallotKHR
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_DrawParameters => {
            naga::back::spv::Capability::DrawParameters
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_WorkgroupMemoryExplicitLayoutKHR => {
            naga::back::spv::Capability::WorkgroupMemoryExplicitLayoutKHR
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_WorkgroupMemoryExplicitLayout8BitAccessKHR => {
            naga::back::spv::Capability::WorkgroupMemoryExplicitLayout8BitAccessKHR
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_WorkgroupMemoryExplicitLayout16BitAccessKHR => {
            naga::back::spv::Capability::WorkgroupMemoryExplicitLayout16BitAccessKHR
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_SubgroupVoteKHR => {
            naga::back::spv::Capability::SubgroupVoteKHR
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_StorageBuffer16BitAccess => {
            naga::back::spv::Capability::StorageBuffer16BitAccess
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_UniformAndStorageBuffer16BitAccess => {
            naga::back::spv::Capability::UniformAndStorageBuffer16BitAccess
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_StoragePushConstant16 => {
            naga::back::spv::Capability::StoragePushConstant16
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_StorageInputOutput16 => {
            naga::back::spv::Capability::StorageInputOutput16
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_DeviceGroup => {
            naga::back::spv::Capability::DeviceGroup
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_MultiView => {
            naga::back::spv::Capability::MultiView
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_VariablePointersStorageBuffer => {
            naga::back::spv::Capability::VariablePointersStorageBuffer
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_VariablePointers => {
            naga::back::spv::Capability::VariablePointers
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_AtomicStorageOps => {
            naga::back::spv::Capability::AtomicStorageOps
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_SampleMaskPostDepthCoverage => {
            naga::back::spv::Capability::SampleMaskPostDepthCoverage
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_StorageBuffer8BitAccess => {
            naga::back::spv::Capability::StorageBuffer8BitAccess
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_UniformAndStorageBuffer8BitAccess => {
            naga::back::spv::Capability::UniformAndStorageBuffer8BitAccess
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_StoragePushConstant8 => {
            naga::back::spv::Capability::StoragePushConstant8
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_DenormPreserve => {
            naga::back::spv::Capability::DenormPreserve
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_DenormFlushToZero => {
            naga::back::spv::Capability::DenormFlushToZero
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_SignedZeroInfNanPreserve => {
            naga::back::spv::Capability::SignedZeroInfNanPreserve
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_RoundingModeRTE => {
            naga::back::spv::Capability::RoundingModeRTE
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_RoundingModeRTZ => {
            naga::back::spv::Capability::RoundingModeRTZ
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_RayQueryProvisionalKHR => {
            naga::back::spv::Capability::RayQueryProvisionalKHR
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_RayQueryKHR => {
            naga::back::spv::Capability::RayQueryKHR
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_RayTraversalPrimitiveCullingKHR => {
            naga::back::spv::Capability::RayTraversalPrimitiveCullingKHR
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_RayTracingKHR => {
            naga::back::spv::Capability::RayTracingKHR
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_TextureSampleWeightedQCOM => {
            naga::back::spv::Capability::TextureSampleWeightedQCOM
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_TextureBoxFilterQCOM => {
            naga::back::spv::Capability::TextureBoxFilterQCOM
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_TextureBlockMatchQCOM => {
            naga::back::spv::Capability::TextureBlockMatchQCOM
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_Float16ImageAMD => {
            naga::back::spv::Capability::Float16ImageAMD
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_ImageGatherBiasLodAMD => {
            naga::back::spv::Capability::ImageGatherBiasLodAMD
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_FragmentMaskAMD => {
            naga::back::spv::Capability::FragmentMaskAMD
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_StencilExportEXT => {
            naga::back::spv::Capability::StencilExportEXT
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_ImageReadWriteLodAMD => {
            naga::back::spv::Capability::ImageReadWriteLodAMD
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_Int64ImageEXT => {
            naga::back::spv::Capability::Int64ImageEXT
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_ShaderClockKHR => {
            naga::back::spv::Capability::ShaderClockKHR
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_ShaderEnqueueAMDX => {
            naga::back::spv::Capability::ShaderEnqueueAMDX
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_SampleMaskOverrideCoverageNV => {
            naga::back::spv::Capability::SampleMaskOverrideCoverageNV
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_GeometryShaderPassthroughNV => {
            naga::back::spv::Capability::GeometryShaderPassthroughNV
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_ShaderViewportIndexLayerEXT => {
            naga::back::spv::Capability::ShaderViewportIndexLayerEXT
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_ShaderViewportMaskNV => {
            naga::back::spv::Capability::ShaderViewportMaskNV
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_ShaderStereoViewNV => {
            naga::back::spv::Capability::ShaderStereoViewNV
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_PerViewAttributesNV => {
            naga::back::spv::Capability::PerViewAttributesNV
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_FragmentFullyCoveredEXT => {
            naga::back::spv::Capability::FragmentFullyCoveredEXT
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_MeshShadingNV => {
            naga::back::spv::Capability::MeshShadingNV
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_ImageFootprintNV => {
            naga::back::spv::Capability::ImageFootprintNV
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_MeshShadingEXT => {
            naga::back::spv::Capability::MeshShadingEXT
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_FragmentBarycentricKHR => {
            naga::back::spv::Capability::FragmentBarycentricKHR
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_ComputeDerivativeGroupQuadsNV => {
            naga::back::spv::Capability::ComputeDerivativeGroupQuadsNV
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_FragmentDensityEXT => {
            naga::back::spv::Capability::FragmentDensityEXT
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_GroupNonUniformPartitionedNV => {
            naga::back::spv::Capability::GroupNonUniformPartitionedNV
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_ShaderNonUniform => {
            naga::back::spv::Capability::ShaderNonUniform
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_RuntimeDescriptorArray => {
            naga::back::spv::Capability::RuntimeDescriptorArray
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_InputAttachmentArrayDynamicIndexing => {
            naga::back::spv::Capability::InputAttachmentArrayDynamicIndexing
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_UniformTexelBufferArrayDynamicIndexing => {
            naga::back::spv::Capability::UniformTexelBufferArrayDynamicIndexing
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_StorageTexelBufferArrayDynamicIndexing => {
            naga::back::spv::Capability::StorageTexelBufferArrayDynamicIndexing
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_UniformBufferArrayNonUniformIndexing => {
            naga::back::spv::Capability::UniformBufferArrayNonUniformIndexing
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_SampledImageArrayNonUniformIndexing => {
            naga::back::spv::Capability::SampledImageArrayNonUniformIndexing
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_StorageBufferArrayNonUniformIndexing => {
            naga::back::spv::Capability::StorageBufferArrayNonUniformIndexing
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_StorageImageArrayNonUniformIndexing => {
            naga::back::spv::Capability::StorageImageArrayNonUniformIndexing
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_InputAttachmentArrayNonUniformIndexing => {
            naga::back::spv::Capability::InputAttachmentArrayNonUniformIndexing
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_UniformTexelBufferArrayNonUniformIndexing => {
            naga::back::spv::Capability::UniformTexelBufferArrayNonUniformIndexing
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_StorageTexelBufferArrayNonUniformIndexing => {
            naga::back::spv::Capability::StorageTexelBufferArrayNonUniformIndexing
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_RayTracingPositionFetchKHR => {
            naga::back::spv::Capability::RayTracingPositionFetchKHR
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_RayTracingNV => {
            naga::back::spv::Capability::RayTracingNV
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_RayTracingMotionBlurNV => {
            naga::back::spv::Capability::RayTracingMotionBlurNV
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_VulkanMemoryModel => {
            naga::back::spv::Capability::VulkanMemoryModel
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_VulkanMemoryModelDeviceScope => {
            naga::back::spv::Capability::VulkanMemoryModelDeviceScope
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_PhysicalStorageBufferAddresses => {
            naga::back::spv::Capability::PhysicalStorageBufferAddresses
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_ComputeDerivativeGroupLinearNV => {
            naga::back::spv::Capability::ComputeDerivativeGroupLinearNV
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_RayTracingProvisionalKHR => {
            naga::back::spv::Capability::RayTracingProvisionalKHR
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_CooperativeMatrixNV => {
            naga::back::spv::Capability::CooperativeMatrixNV
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_FragmentShaderSampleInterlockEXT => {
            naga::back::spv::Capability::FragmentShaderSampleInterlockEXT
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_FragmentShaderShadingRateInterlockEXT => {
            naga::back::spv::Capability::FragmentShaderShadingRateInterlockEXT
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_ShaderSMBuiltinsNV => {
            naga::back::spv::Capability::ShaderSMBuiltinsNV
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_FragmentShaderPixelInterlockEXT => {
            naga::back::spv::Capability::FragmentShaderPixelInterlockEXT
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_DemoteToHelperInvocation => {
            naga::back::spv::Capability::DemoteToHelperInvocation
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_DisplacementMicromapNV => {
            naga::back::spv::Capability::DisplacementMicromapNV
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_RayTracingOpacityMicromapEXT => {
            naga::back::spv::Capability::RayTracingOpacityMicromapEXT
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_ShaderInvocationReorderNV => {
            naga::back::spv::Capability::ShaderInvocationReorderNV
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_BindlessTextureNV => {
            naga::back::spv::Capability::BindlessTextureNV
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_RayQueryPositionFetchKHR => {
            naga::back::spv::Capability::RayQueryPositionFetchKHR
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_RayTracingDisplacementMicromapNV => {
            naga::back::spv::Capability::RayTracingDisplacementMicromapNV
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_SubgroupShuffleINTEL => {
            naga::back::spv::Capability::SubgroupShuffleINTEL
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_SubgroupBufferBlockIOINTEL => {
            naga::back::spv::Capability::SubgroupBufferBlockIOINTEL
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_SubgroupImageBlockIOINTEL => {
            naga::back::spv::Capability::SubgroupImageBlockIOINTEL
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_SubgroupImageMediaBlockIOINTEL => {
            naga::back::spv::Capability::SubgroupImageMediaBlockIOINTEL
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_RoundToInfinityINTEL => {
            naga::back::spv::Capability::RoundToInfinityINTEL
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_FloatingPointModeINTEL => {
            naga::back::spv::Capability::FloatingPointModeINTEL
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_IntegerFunctions2INTEL => {
            naga::back::spv::Capability::IntegerFunctions2INTEL
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_FunctionPointersINTEL => {
            naga::back::spv::Capability::FunctionPointersINTEL
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_IndirectReferencesINTEL => {
            naga::back::spv::Capability::IndirectReferencesINTEL
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_AsmINTEL => naga::back::spv::Capability::AsmINTEL,
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_AtomicFloat32MinMaxEXT => {
            naga::back::spv::Capability::AtomicFloat32MinMaxEXT
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_AtomicFloat64MinMaxEXT => {
            naga::back::spv::Capability::AtomicFloat64MinMaxEXT
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_AtomicFloat16MinMaxEXT => {
            naga::back::spv::Capability::AtomicFloat16MinMaxEXT
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_VectorComputeINTEL => {
            naga::back::spv::Capability::VectorComputeINTEL
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_VectorAnyINTEL => {
            naga::back::spv::Capability::VectorAnyINTEL
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_ExpectAssumeKHR => {
            naga::back::spv::Capability::ExpectAssumeKHR
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_SubgroupAvcMotionEstimationINTEL => {
            naga::back::spv::Capability::SubgroupAvcMotionEstimationINTEL
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_SubgroupAvcMotionEstimationIntraINTEL => {
            naga::back::spv::Capability::SubgroupAvcMotionEstimationIntraINTEL
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_SubgroupAvcMotionEstimationChromaINTEL => {
            naga::back::spv::Capability::SubgroupAvcMotionEstimationChromaINTEL
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_VariableLengthArrayINTEL => {
            naga::back::spv::Capability::VariableLengthArrayINTEL
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_FunctionFloatControlINTEL => {
            naga::back::spv::Capability::FunctionFloatControlINTEL
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_FPGAMemoryAttributesINTEL => {
            naga::back::spv::Capability::FPGAMemoryAttributesINTEL
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_FPFastMathModeINTEL => {
            naga::back::spv::Capability::FPFastMathModeINTEL
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_ArbitraryPrecisionIntegersINTEL => {
            naga::back::spv::Capability::ArbitraryPrecisionIntegersINTEL
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_ArbitraryPrecisionFloatingPointINTEL => {
            naga::back::spv::Capability::ArbitraryPrecisionFloatingPointINTEL
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_UnstructuredLoopControlsINTEL => {
            naga::back::spv::Capability::UnstructuredLoopControlsINTEL
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_FPGALoopControlsINTEL => {
            naga::back::spv::Capability::FPGALoopControlsINTEL
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_KernelAttributesINTEL => {
            naga::back::spv::Capability::KernelAttributesINTEL
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_FPGAKernelAttributesINTEL => {
            naga::back::spv::Capability::FPGAKernelAttributesINTEL
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_FPGAMemoryAccessesINTEL => {
            naga::back::spv::Capability::FPGAMemoryAccessesINTEL
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_FPGAClusterAttributesINTEL => {
            naga::back::spv::Capability::FPGAClusterAttributesINTEL
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_LoopFuseINTEL => {
            naga::back::spv::Capability::LoopFuseINTEL
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_FPGADSPControlINTEL => {
            naga::back::spv::Capability::FPGADSPControlINTEL
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_MemoryAccessAliasingINTEL => {
            naga::back::spv::Capability::MemoryAccessAliasingINTEL
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_FPGAInvocationPipeliningAttributesINTEL => {
            naga::back::spv::Capability::FPGAInvocationPipeliningAttributesINTEL
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_FPGABufferLocationINTEL => {
            naga::back::spv::Capability::FPGABufferLocationINTEL
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_ArbitraryPrecisionFixedPointINTEL => {
            naga::back::spv::Capability::ArbitraryPrecisionFixedPointINTEL
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_USMStorageClassesINTEL => {
            naga::back::spv::Capability::USMStorageClassesINTEL
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_RuntimeAlignedAttributeINTEL => {
            naga::back::spv::Capability::RuntimeAlignedAttributeINTEL
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_IOPipesINTEL => {
            naga::back::spv::Capability::IOPipesINTEL
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_BlockingPipesINTEL => {
            naga::back::spv::Capability::BlockingPipesINTEL
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_FPGARegINTEL => {
            naga::back::spv::Capability::FPGARegINTEL
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_DotProductInputAll => {
            naga::back::spv::Capability::DotProductInputAll
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_DotProductInput4x8Bit => {
            naga::back::spv::Capability::DotProductInput4x8Bit
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_DotProductInput4x8BitPacked => {
            naga::back::spv::Capability::DotProductInput4x8BitPacked
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_DotProduct => {
            naga::back::spv::Capability::DotProduct
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_RayCullMaskKHR => {
            naga::back::spv::Capability::RayCullMaskKHR
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_CooperativeMatrixKHR => {
            naga::back::spv::Capability::CooperativeMatrixKHR
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_BitInstructions => {
            naga::back::spv::Capability::BitInstructions
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_GroupNonUniformRotateKHR => {
            naga::back::spv::Capability::GroupNonUniformRotateKHR
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_AtomicFloat32AddEXT => {
            naga::back::spv::Capability::AtomicFloat32AddEXT
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_AtomicFloat64AddEXT => {
            naga::back::spv::Capability::AtomicFloat64AddEXT
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_LongConstantCompositeINTEL => {
            todo!()
            // naga::back::spv::Capability::LongConstantCompositeINTEL
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_OptNoneINTEL => {
            naga::back::spv::Capability::OptNoneINTEL
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_AtomicFloat16AddEXT => {
            naga::back::spv::Capability::AtomicFloat16AddEXT
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_DebugInfoModuleINTEL => {
            naga::back::spv::Capability::DebugInfoModuleINTEL
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_BFloat16ConversionINTEL => {
            naga::back::spv::Capability::BFloat16ConversionINTEL
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_SplitBarrierINTEL => {
            naga::back::spv::Capability::SplitBarrierINTEL
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_GlobalVariableFPGADecorationsINTEL => {
            naga::back::spv::Capability::GlobalVariableFPGADecorationsINTEL
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_FPGAKernelAttributesv2INTEL => {
            naga::back::spv::Capability::FPGAKernelAttributesv2INTEL
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_GlobalVariableHostAccessINTEL => {
            naga::back::spv::Capability::GlobalVariableHostAccessINTEL
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_FPMaxErrorINTEL => {
            naga::back::spv::Capability::FPMaxErrorINTEL
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_FPGALatencyControlINTEL => {
            naga::back::spv::Capability::FPGALatencyControlINTEL
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_FPGAArgumentInterfacesINTEL => {
            naga::back::spv::Capability::FPGAArgumentInterfacesINTEL
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_GroupUniformArithmeticKHR => {
            naga::back::spv::Capability::GroupUniformArithmeticKHR
        }
        ffi::NagaSPVBackCapability_NagaSPVBackCapability_CacheControlsINTEL => {
            naga::back::spv::Capability::CacheControlsINTEL
        }
        _ => panic!("Unknown SPVBackCapability"),
    }
}

pub fn spv_back_capability_set_to_naga(
    capability_set: &ffi::NagaSPVBackCapabilitySet,
) -> naga::FastHashSet<naga::back::spv::Capability> {
    unsafe {
        std::slice::from_raw_parts(capability_set.capabilities, capability_set.capabilities_len)
            .iter()
            .map(|capability| spv_back_capability_to_naga(*capability))
            .collect()
    }
}

pub fn spv_back_writer_flags_to_naga(
    flags: ffi::NagaSPVBackWriterFlags,
) -> naga::back::spv::WriterFlags {
    let mut result = naga::back::spv::WriterFlags::empty();

    if flags & ffi::NagaSPVBackWriterFlags_NagaSPVBackWriterFlags_DEBUG != 0 {
        result |= naga::back::spv::WriterFlags::DEBUG;
    }
    if flags & ffi::NagaSPVBackWriterFlags_NagaSPVBackWriterFlags_ADJUST_COORDINATE_SPACE != 0 {
        result |= naga::back::spv::WriterFlags::ADJUST_COORDINATE_SPACE;
    }
    if flags & ffi::NagaSPVBackWriterFlags_NagaSPVBackWriterFlags_LABEL_VARYINGS != 0 {
        result |= naga::back::spv::WriterFlags::LABEL_VARYINGS;
    }
    if flags & ffi::NagaSPVBackWriterFlags_NagaSPVBackWriterFlags_FORCE_POINT_SIZE != 0 {
        result |= naga::back::spv::WriterFlags::FORCE_POINT_SIZE;
    }
    if flags & ffi::NagaSPVBackWriterFlags_NagaSPVBackWriterFlags_CLAMP_FRAG_DEPTH != 0 {
        result |= naga::back::spv::WriterFlags::CLAMP_FRAG_DEPTH;
    }
    if flags
        & ffi::NagaSPVBackWriterFlags_NagaSPVBackWriterFlags_PRINT_ON_RAY_QUERY_INITIALIZATION_FAIL
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
    info: &ffi::NagaSPVBackBindingInfo,
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

pub fn spv_back_binding_map_to_naga(
    map: &ffi::NagaSPVBackBindingMap,
) -> naga::back::spv::BindingMap {
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
    mode: ffi::NagaSPVBackZeroInitializeWorkgroupMemoryMode,
) -> naga::back::spv::ZeroInitializeWorkgroupMemoryMode {
    match mode {
        ffi::NagaSPVBackZeroInitializeWorkgroupMemoryMode_NagaSPVBackZeroInitializeWorkgroupMemoryMode_Native => {
            naga::back::spv::ZeroInitializeWorkgroupMemoryMode::Native
        }
        ffi::NagaSPVBackZeroInitializeWorkgroupMemoryMode_NagaSPVBackZeroInitializeWorkgroupMemoryMode_Polyfill => {
            naga::back::spv::ZeroInitializeWorkgroupMemoryMode::Polyfill
        }
        ffi::NagaSPVBackZeroInitializeWorkgroupMemoryMode_NagaSPVBackZeroInitializeWorkgroupMemoryMode_None => {
            naga::back::spv::ZeroInitializeWorkgroupMemoryMode::None
        }
        _ => panic!("Unknown SPVBackZeroInitializeWorkgroupMemoryMode"),
    }
}

pub fn spv_back_source_language_to_naga(
    lang: ffi::NagaSPVBackSourceLanguage,
) -> naga::back::spv::SourceLanguage {
    match lang {
        ffi::NagaSPVBackSourceLanguage_NagaSPVBackSourceLanguage_Unknown => {
            naga::back::spv::SourceLanguage::Unknown
        }
        ffi::NagaSPVBackSourceLanguage_NagaSPVBackSourceLanguage_ESSL => {
            naga::back::spv::SourceLanguage::ESSL
        }
        ffi::NagaSPVBackSourceLanguage_NagaSPVBackSourceLanguage_GLSL => {
            naga::back::spv::SourceLanguage::GLSL
        }
        ffi::NagaSPVBackSourceLanguage_NagaSPVBackSourceLanguage_OpenCL_C => {
            naga::back::spv::SourceLanguage::OpenCL_C
        }
        ffi::NagaSPVBackSourceLanguage_NagaSPVBackSourceLanguage_OpenCL_CPP => {
            naga::back::spv::SourceLanguage::OpenCL_CPP
        }
        ffi::NagaSPVBackSourceLanguage_NagaSPVBackSourceLanguage_HLSL => {
            naga::back::spv::SourceLanguage::HLSL
        }
        ffi::NagaSPVBackSourceLanguage_NagaSPVBackSourceLanguage_CPP_for_OpenCL => {
            naga::back::spv::SourceLanguage::CPP_for_OpenCL
        }
        ffi::NagaSPVBackSourceLanguage_NagaSPVBackSourceLanguage_SYCL => {
            naga::back::spv::SourceLanguage::SYCL
        }
        ffi::NagaSPVBackSourceLanguage_NagaSPVBackSourceLanguage_HERO_C => {
            naga::back::spv::SourceLanguage::HERO_C
        }
        ffi::NagaSPVBackSourceLanguage_NagaSPVBackSourceLanguage_NZSL => {
            naga::back::spv::SourceLanguage::NZSL
        }
        ffi::NagaSPVBackSourceLanguage_NagaSPVBackSourceLanguage_WGSL => {
            naga::back::spv::SourceLanguage::WGSL
        }
        ffi::NagaSPVBackSourceLanguage_NagaSPVBackSourceLanguage_Slang => {
            naga::back::spv::SourceLanguage::Slang
        }
        _ => panic!("Unknown SPVBackSourceLanguage"),
    }
}

pub fn spv_back_options_to_naga(options: &ffi::NagaSPVBackOptions) -> naga::back::spv::Options<'_> {
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
    options: &ffi::NagaSPVBackPipelineOptions,
) -> naga::back::spv::PipelineOptions {
    naga::back::spv::PipelineOptions {
        shader_stage: shader_stage_to_naga(&options.shader_stage),
        entry_point: unsafe { string_to_naga(options.entry_point) },
    }
}

pub fn spv_back_error_to_ffi(error: &naga::back::spv::Error) -> ffi::NagaSPVBackError {
    let default_data = ffi::NagaSPVBackError__bindgen_ty_1::default();

    match error {
        naga::back::spv::Error::EntryPointNotFound => ffi::NagaSPVBackError {
            tag: ffi::NagaSPVBackErrorTag_NagaSPVBackErrorTag_EntryPointNotFound,
            data: default_data,
        },
        naga::back::spv::Error::UnsupportedVersion(v0, v1) => ffi::NagaSPVBackError {
            tag: ffi::NagaSPVBackErrorTag_NagaSPVBackErrorTag_UnsupportedVersion,
            data: ffi::NagaSPVBackError__bindgen_ty_1 {
                unsupported_version: [*v0, *v1],
            },
        },
        naga::back::spv::Error::MissingCapabilities(error, capabilities) => ffi::NagaSPVBackError {
            tag: ffi::NagaSPVBackErrorTag_NagaSPVBackErrorTag_MissingCapabilities,
            data: ffi::NagaSPVBackError__bindgen_ty_1 {
                missing_capabilities: ffi::NagaSPVBackError__bindgen_ty_1__bindgen_ty_1 {
                    error: unsafe { string_to_ffi(error) },
                    capabilities: unsafe {
                        slice_to_ffi(capabilities.as_slice(), spv_back_capability_to_ffi)
                    },
                    capabilities_len: capabilities.len() as u32,
                },
            },
        },
        naga::back::spv::Error::FeatureNotImplemented(feature) => ffi::NagaSPVBackError {
            tag: ffi::NagaSPVBackErrorTag_NagaSPVBackErrorTag_FeatureNotImplemented,
            data: ffi::NagaSPVBackError__bindgen_ty_1 {
                feature_not_implemented: unsafe { string_to_ffi(feature) },
            },
        },
        naga::back::spv::Error::Validation(error) => ffi::NagaSPVBackError {
            tag: ffi::NagaSPVBackErrorTag_NagaSPVBackErrorTag_Validation,
            data: ffi::NagaSPVBackError__bindgen_ty_1 {
                validation: unsafe { string_to_ffi(error) },
            },
        },
        naga::back::spv::Error::Override => ffi::NagaSPVBackError {
            tag: ffi::NagaSPVBackErrorTag_NagaSPVBackErrorTag_Override,
            data: default_data,
        },
        naga::back::spv::Error::ResolveArraySizeError(resolve_array_size_error) => {
            ffi::NagaSPVBackError {
                tag: ffi::NagaSPVBackErrorTag_NagaSPVBackErrorTag_ResolveArraySizeError,
                data: ffi::NagaSPVBackError__bindgen_ty_1 {
                    resolve_array_size_error: resolve_array_size_error_to_ffi(
                        resolve_array_size_error,
                    ),
                },
            }
        }
        naga::back::spv::Error::SpirvVersionTooLow(v0, v1) => ffi::NagaSPVBackError {
            tag: ffi::NagaSPVBackErrorTag_NagaSPVBackErrorTag_SpirvVersionTooLow,
            data: ffi::NagaSPVBackError__bindgen_ty_1 {
                spirv_version_too_low: [*v0, *v1],
            },
        },
        naga::back::spv::Error::MissingBinding(resource_binding) => ffi::NagaSPVBackError {
            tag: ffi::NagaSPVBackErrorTag_NagaSPVBackErrorTag_MissingBinding,
            data: ffi::NagaSPVBackError__bindgen_ty_1 {
                missing_binding: resource_binding_to_ffi(resource_binding),
            },
        },
    }
}
