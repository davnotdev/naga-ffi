#ifndef NAGA_FFI_H
#define NAGA_FFI_H

#include <stdint.h>
#include <stdlib.h>

#define NAGA_FFI_VERSION 28
#define NAGA_NULLABLE
#define NAGA_UNIMPLEMENTED

#define DEFINE_OPTIONAL(T) \
	struct {               \
		bool some;         \
		T value;           \
	}
#define DEFINE_HANDLE_INDEX(T) size_t

struct Empty {
	uint8_t _phantom;
};

#ifndef __cplusplus
typedef uint8_t bool;
#endif

// TODO: desctructors to free everything
// TODO: validator errors
// TODO: implement *::ReflectionInfo

typedef struct Span {
	uint32_t start;
	uint32_t end;
} Span;

typedef struct SpanContext {
	Span span;
	char *message;
} SpanContext;

// --- naga::ir ---

typedef uint8_t Bytes;

typedef struct Type Type;
typedef struct Override Override;

typedef enum ScalarKind {
	ScalarKind_Sint = 0,
	ScalarKind_Uint = 1,
	ScalarKind_Float = 2,
	ScalarKind_Bool = 3,
	ScalarKind_AbstractInt = 4,
	ScalarKind_AbstractFloat = 5,
} ScalarKind;

typedef struct Scalar {
	ScalarKind kind;
	Bytes width;
} Scalar;

typedef enum VectorSize {
	VectorSize_Bi = 2,
	VectorSize_Tri = 3,
	VectorSize_Quad = 4,
} VectorSize;

typedef enum ArraySizeTag {
	ArraySizeTag_Constant,
	ArraySizeTag_Pending,
	ArraySizeTag_Dynamic,
} ArraySizeTag;

typedef struct ArraySize {
	ArraySizeTag tag;
	union {
		uint32_t constant;
		DEFINE_HANDLE_INDEX(Override)
		pending;
	} data;
} ArraySize;

typedef enum StorageFormat {
	StorageFormat_R8Unorm,
	StorageFormat_R8Snorm,
	StorageFormat_R8Uint,
	StorageFormat_R8Sint,
	StorageFormat_R16Uint,
	StorageFormat_R16Sint,
	StorageFormat_R16Float,
	StorageFormat_Rg8Unorm,
	StorageFormat_Rg8Snorm,
	StorageFormat_Rg8Uint,
	StorageFormat_Rg8Sint,
	StorageFormat_R32Uint,
	StorageFormat_R32Sint,
	StorageFormat_R32Float,
	StorageFormat_Rg16Uint,
	StorageFormat_Rg16Sint,
	StorageFormat_Rg16Float,
	StorageFormat_Rgba8Unorm,
	StorageFormat_Rgba8Snorm,
	StorageFormat_Rgba8Uint,
	StorageFormat_Rgba8Sint,
	StorageFormat_Bgra8Unorm,
	StorageFormat_Rgb10a2Uint,
	StorageFormat_Rgb10a2Unorm,
	StorageFormat_Rg11b10Ufloat,
	StorageFormat_R64Uint,
	StorageFormat_Rg32Uint,
	StorageFormat_Rg32Sint,
	StorageFormat_Rg32Float,
	StorageFormat_Rgba16Uint,
	StorageFormat_Rgba16Sint,
	StorageFormat_Rgba16Float,
	StorageFormat_Rgba32Uint,
	StorageFormat_Rgba32Sint,
	StorageFormat_Rgba32Float,
	StorageFormat_R16Unorm,
	StorageFormat_R16Snorm,
	StorageFormat_Rg16Unorm,
	StorageFormat_Rg16Snorm,
	StorageFormat_Rgba16Unorm,
	StorageFormat_Rgba16Snorm,
} StorageFormat;

typedef enum StorageAccess {
	StorageAccess_LOAD = 0x1,
	StorageAccess_STORE = 0x2,
	StorageAccess_ATOMIC = 0x4,
} StorageAccess;

typedef enum RelationalFunction {
	RelationalFunction_All,
	RelationalFunction_Any,
	RelationalFunction_IsNan,
	RelationalFunction_IsInf,
} RelationalFunction;

typedef enum ImageClassTag {
	ImageClassTag_Sampled,
	ImageClassTag_Depth,
	ImageClassTag_External,
	ImageClassTag_Storage,
} ImageClassTag;

typedef struct ImageClass {
	ImageClassTag tag;
	union {
		struct {
			ScalarKind kind;
			bool multi;
		} sampled;
		struct {
			bool multi;
		} depth;
		struct {
			StorageFormat format;
			StorageAccess access;
		} storage;
	} data;
} ImageClass;

typedef enum AddressSpaceTag {
	AddressSpaceTag_Function,
	AddressSpaceTag_Private,
	AddressSpaceTag_WorkGroup,
	AddressSpaceTag_Uniform,
	AddressSpaceTag_Storage,
	AddressSpaceTag_Handle,
	AddressSpaceTag_Immediate,
	AddressSpaceTag_TaskPayload,
} AddressSpaceTag;

typedef struct AddressSpace {
	AddressSpaceTag tag;
	union {
		struct {
			StorageAccess access;
		} storage;
	} data;
} AddressSpace;

typedef enum MathFunction {
	MathFunction_Abs,
	MathFunction_Min,
	MathFunction_Max,
	MathFunction_Clamp,
	MathFunction_Saturate,
	MathFunction_Cos,
	MathFunction_Cosh,
	MathFunction_Sin,
	MathFunction_Sinh,
	MathFunction_Tan,
	MathFunction_Tanh,
	MathFunction_Acos,
	MathFunction_Asin,
	MathFunction_Atan,
	MathFunction_Atan2,
	MathFunction_Asinh,
	MathFunction_Acosh,
	MathFunction_Atanh,
	MathFunction_Radians,
	MathFunction_Degrees,
	MathFunction_Ceil,
	MathFunction_Floor,
	MathFunction_Round,
	MathFunction_Fract,
	MathFunction_Trunc,
	MathFunction_Modf,
	MathFunction_Frexp,
	MathFunction_Ldexp,
	MathFunction_Exp,
	MathFunction_Exp2,
	MathFunction_Log,
	MathFunction_Log2,
	MathFunction_Pow,
	MathFunction_Dot,
	MathFunction_Dot4I8Packed,
	MathFunction_Dot4U8Packed,
	MathFunction_Outer,
	MathFunction_Cross,
	MathFunction_Distance,
	MathFunction_Length,
	MathFunction_Normalize,
	MathFunction_FaceForward,
	MathFunction_Reflect,
	MathFunction_Refract,
	MathFunction_Sign,
	MathFunction_Fma,
	MathFunction_Mix,
	MathFunction_Step,
	MathFunction_SmoothStep,
	MathFunction_Sqrt,
	MathFunction_InverseSqrt,
	MathFunction_Inverse,
	MathFunction_Transpose,
	MathFunction_Determinant,
	MathFunction_QuantizeToF16,
	MathFunction_CountTrailingZeros,
	MathFunction_CountLeadingZeros,
	MathFunction_CountOneBits,
	MathFunction_ReverseBits,
	MathFunction_ExtractBits,
	MathFunction_InsertBits,
	MathFunction_FirstTrailingBit,
	MathFunction_FirstLeadingBit,
	MathFunction_Pack4x8snorm,
	MathFunction_Pack4x8unorm,
	MathFunction_Pack2x16snorm,
	MathFunction_Pack2x16unorm,
	MathFunction_Pack2x16float,
	MathFunction_Pack4xI8,
	MathFunction_Pack4xU8,
	MathFunction_Pack4xI8Clamp,
	MathFunction_Pack4xU8Clamp,
	MathFunction_Unpack4x8snorm,
	MathFunction_Unpack4x8unorm,
	MathFunction_Unpack2x16snorm,
	MathFunction_Unpack2x16unorm,
	MathFunction_Unpack2x16float,
	MathFunction_Unpack4xI8,
	MathFunction_Unpack4xU8,
} MathFunction;

typedef enum Interpolation {
	Interpolation_Perspective,
	Interpolation_Linear,
	Interpolation_Flat,
} Interpolation;

typedef enum Sampling {
	Sampling_Center,
	Sampling_Centroid,
	Sampling_Sample,
	Sampling_First,
	Sampling_Either,
} Sampling;

typedef enum BuiltInTag {
	BuiltInTag_Position,
	BuiltInTag_ViewIndex,
	BuiltInTag_BaseInstance,
	BuiltInTag_BaseVertex,
	BuiltInTag_ClipDistance,
	BuiltInTag_CullDistance,
	BuiltInTag_InstanceIndex,
	BuiltInTag_PointSize,
	BuiltInTag_VertexIndex,
	BuiltInTag_DrawID,
	BuiltInTag_FragDepth,
	BuiltInTag_PointCoord,
	BuiltInTag_FrontFacing,
	BuiltInTag_PrimitiveIndex,
	BuiltInTag_Barycentric,
	BuiltInTag_SampleIndex,
	BuiltInTag_SampleMask,
	BuiltInTag_GlobalInvocationId,
	BuiltInTag_LocalInvocationId,
	BuiltInTag_LocalInvocationIndex,
	BuiltInTag_WorkGroupId,
	BuiltInTag_WorkGroupSize,
	BuiltInTag_NumWorkGroups,
	BuiltInTag_NumSubgroups,
	BuiltInTag_SubgroupId,
	BuiltInTag_SubgroupSize,
	BuiltInTag_SubgroupInvocationId,
	BuiltInTag_MeshTaskSize,
	BuiltInTag_CullPrimitive,
	BuiltInTag_PointIndex,
	BuiltInTag_LineIndices,
	BuiltInTag_TriangleIndices,
	BuiltInTag_VertexCount,
	BuiltInTag_Vertices,
	BuiltInTag_PrimitiveCount,
	BuiltInTag_Primitives,
} BuiltInTag;

typedef struct BuiltIn {
	BuiltInTag tag;
	union {
		struct {
			bool invariant;
		} position;
	} data;
} BuiltIn;

typedef enum BindingTag {
	BindingTag_BuiltIn,
	BindingTag_Location,
} BindingTag;

typedef struct Binding {
	BindingTag tag;
	union {
		BuiltIn built_in;
		struct {
			uint32_t location;
			Interpolation interpolation;
			Sampling sampling;
			uint32_t blend_src;
			bool per_primitive;
		} location;
	} data;
} Binding;

typedef struct StructMember {
	char *NAGA_NULLABLE name;
	DEFINE_HANDLE_INDEX(Type)
	ty;
	DEFINE_OPTIONAL(Binding)
	binding;
	uint32_t offset;
} StructMember;

typedef enum ImageDimension {
	D1,
	D2,
	D3,
	Cube,
} ImageDimension;

typedef enum TypeInnerTag {
	TypeInnerTag_Scalar,
	TypeInnerTag_Vector,
	TypeInnerTag_Matrix,
	TypeInnerTag_Atomic,
	TypeInnerTag_Pointer,
	TypeInnerTag_ValuePointer,
	TypeInnerTag_Array,
	TypeInnerTag_Struct,
	TypeInnerTag_Image,
	TypeInnerTag_Sampler,
	TypeInnerTag_AccelerationStructure,
	TypeInnerTag_RayQuery,
	TypeInnerTag_BindingArray,
} TypeInnerTag;

typedef struct TypeInner {
	TypeInnerTag tag;
	union {
		Scalar scalar;
		struct {
			VectorSize size;
			Scalar scalar;
		} vector;
		struct {
			VectorSize columns;
			VectorSize rows;
			Scalar scalar;
		} matrix;
		Scalar atomic;
		struct {
			DEFINE_HANDLE_INDEX(Type)
			base;
			AddressSpace space;
		} pointer;
		struct {
			VectorSize size;
			Scalar scalar;
			AddressSpace space;
		} value_pointer;
		struct {
			DEFINE_HANDLE_INDEX(Type)
			base;
			ArraySize size;
			uint32_t stride;
		} array;
		struct {
			StructMember *members;
			size_t members_len;
			uint32_t span;
		} struct_;
		struct {
			ImageDimension dim;
			bool arrayed;
			ImageClass class_;
		} image;
		struct {
			bool comparison;
		} sampler;
		struct {
			bool vertex_return;
		} acceleration_structure;
		struct {
			bool vertex_return;
		} ray_query;
		struct {
			DEFINE_HANDLE_INDEX(Type)
			base;
			ArraySize size;
		} binding_array;
	} data;
} TypeInner;

typedef struct Type {
	char *NAGA_NULLABLE name;
	TypeInner inner;
} Type;

typedef struct Constant {
	char *NAGA_NULLABLE name;
	DEFINE_HANDLE_INDEX(Type)
	ty;
	struct Empty *NAGA_UNIMPLEMENTED init;
} Constant;

typedef struct Override {
	char *NAGA_NULLABLE name;
	DEFINE_OPTIONAL(uint16_t)
	id;
	DEFINE_HANDLE_INDEX(Type)
	ty;
	struct Empty *NAGA_UNIMPLEMENTED init;
} Override;

typedef struct ResourceBinding {
	uint32_t group;
	uint32_t binding;
} ResourceBinding;

typedef struct GlobalVariable {
	char *NAGA_NULLABLE name;
	AddressSpace space;
	DEFINE_OPTIONAL(ResourceBinding)
	binding;
	DEFINE_HANDLE_INDEX(Type)
	ty;
	struct Empty *NAGA_UNIMPLEMENTED init;
} GlobalVariable;

typedef enum ConservativeDepth {
	ConservativeDepth_GreaterEqual,
	ConservativeDepth_LessEqual,
	ConservativeDepth_Unchanged,
} ConservativeDepth;

typedef enum EarlyDepthTestTag {
	EarlyDepthTestTag_Force,
	EarlyDepthTestTag_Allow,
} EarlyDepthTestTag;

typedef struct EarlyDepthTest {
	EarlyDepthTestTag tag;
	union {
		struct {
			ConservativeDepth conservative;
		} allow;
	} data;
} EarlyDepthTest;

typedef enum ShaderStage {
	ShaderStage_Vertex,
	ShaderStage_Task,
	ShaderStage_Mesh,
	ShaderStage_Fragment,
	ShaderStage_Compute,
} ShaderStage;

typedef struct EntryPoint {
	char *name;
	ShaderStage stage;
	DEFINE_OPTIONAL(EarlyDepthTest)
	early_depth_test;
	uint32_t workgroup_size[3];
	struct Empty *NAGA_UNIMPLEMENTED workgroup_size_overrides[3];
	struct Empty *NAGA_UNIMPLEMENTED function;
	struct Empty *NAGA_UNIMPLEMENTED mesh_info;
	struct Empty *NAGA_UNIMPLEMENTED task_payload;
} EntryPoint;

typedef enum ModuleFillFlags {
	ModuleFillFlags_Types = 0x1,
	ModuleFillFlags_Constants = 0x2,
	ModuleFillFlags_Overrides = 0x4,
	ModuleFillFlags_GlobalVariables = 0x8,
	ModuleFillFlags_GlobalExpressions = 0x10,
	ModuleFillFlags_Functions = 0x20,
	ModuleFillFlags_EntryPoints = 0x40,
	ModuleFillFlags_DiagnosticFilters = 0x80,
} ModuleFillFlags;

typedef struct Module {
	void *_inner_module;

	Type *types;
	size_t types_len;
	struct Empty *NAGA_UNIMPLEMENTED special_types;
	Constant *constants;
	size_t constants_len;
	Override *overrides;
	size_t overrides_len;
	GlobalVariable *global_variables;
	size_t global_variables_len;
	struct Empty *NAGA_UNIMPLEMENTED global_expressions;
	size_t global_expressions_len;
	struct Empty *NAGA_UNIMPLEMENTED functions;
	size_t functions_len;
	EntryPoint *entry_points;
	size_t entry_points_len;
	struct Empty *NAGA_UNIMPLEMENTED diagnostic_filters;
	size_t diagnostic_filters_len;
	struct Empty *NAGA_UNIMPLEMENTED diagnostic_filter_leaf;
	struct Empty *NAGA_UNIMPLEMENTED doc_comments;
} Module;

typedef enum ModuleInfoFillFlags {
	ModuleInfoFillFlags_Unimplemented = UINT32_MAX
} ModuleInfoFillFlags;

typedef struct ModuleInfo {
	void *_inner_module_info;
} ModuleInfo;

// --- naga::proc ---

typedef enum ConstantEvaluatorErrorTag {
	ConstantEvaluatorErrorTag_FunctionArg,
	ConstantEvaluatorErrorTag_GlobalVariable,
	ConstantEvaluatorErrorTag_LocalVariable,
	ConstantEvaluatorErrorTag_InvalidArrayLengthArg,
	ConstantEvaluatorErrorTag_ArrayLengthDynamic,
	ConstantEvaluatorErrorTag_ArrayLengthOverridden,
	ConstantEvaluatorErrorTag_Call,
	ConstantEvaluatorErrorTag_WorkGroupUniformLoadResult,
	ConstantEvaluatorErrorTag_Atomic,
	ConstantEvaluatorErrorTag_Derivative,
	ConstantEvaluatorErrorTag_Load,
	ConstantEvaluatorErrorTag_ImageExpression,
	ConstantEvaluatorErrorTag_RayQueryExpression,
	ConstantEvaluatorErrorTag_SubgroupExpression,
	ConstantEvaluatorErrorTag_InvalidAccessBase,
	ConstantEvaluatorErrorTag_InvalidAccessIndex,
	ConstantEvaluatorErrorTag_InvalidAccessIndexTy,
	ConstantEvaluatorErrorTag_ArrayLength,
	ConstantEvaluatorErrorTag_InvalidCastArg,
	ConstantEvaluatorErrorTag_InvalidUnaryOpArg,
	ConstantEvaluatorErrorTag_InvalidBinaryOpArgs,
	ConstantEvaluatorErrorTag_InvalidMathArg,
	ConstantEvaluatorErrorTag_InvalidMathArgCount,
	ConstantEvaluatorErrorTag_InvalidRelationalArg,
	ConstantEvaluatorErrorTag_InvalidClamp,
	ConstantEvaluatorErrorTag_InvalidVectorComposeLength,
	ConstantEvaluatorErrorTag_InvalidVectorComposeComponent,
	ConstantEvaluatorErrorTag_SplatScalarOnly,
	ConstantEvaluatorErrorTag_SwizzleVectorOnly,
	ConstantEvaluatorErrorTag_SwizzleOutOfBounds,
	ConstantEvaluatorErrorTag_TypeNotConstructible,
	ConstantEvaluatorErrorTag_SubexpressionsAreNotConstant,
	ConstantEvaluatorErrorTag_NotImplemented,
	ConstantEvaluatorErrorTag_Overflow,
	ConstantEvaluatorErrorTag_AutomaticConversionLossy,
	ConstantEvaluatorErrorTag_DivisionByZero,
	ConstantEvaluatorErrorTag_RemainderByZero,
	ConstantEvaluatorErrorTag_ShiftedMoreThan32Bits,
	ConstantEvaluatorErrorTag_Literal,
	ConstantEvaluatorErrorTag_Override,
	ConstantEvaluatorErrorTag_RuntimeExpr,
	ConstantEvaluatorErrorTag_OverrideExpr,
	ConstantEvaluatorErrorTag_SelectScalarConditionNotABool,
	ConstantEvaluatorErrorTag_SelectVecRejectAcceptSizeMismatch,
	ConstantEvaluatorErrorTag_SelectConditionNotAVecBool,
	ConstantEvaluatorErrorTag_SelectConditionVecSizeMismatch,
	ConstantEvaluatorErrorTag_SelectAcceptRejectTypeMismatch,
} ConstantEvaluatorErrorTag;

typedef struct ConstantEvaluatorError {
	ConstantEvaluatorErrorTag tag;
	union {
		struct {
			char *from;
			char *to;
		} invalid_cast_arg;
		struct {
			MathFunction function;
			size_t expected;
			size_t actual;
		} invalid_math_arg_count;
		struct Empty *NAGA_UNIMPLEMENTED invalid_relational_arg;
		struct {
			size_t expected;
			size_t actual;
		} invalid_vector_compose_length;
		char *not_implemented;
		char *overflow;
		struct {
			char *value;
			char *to_type;
		} automatic_conversion_lossy;
		struct Empty *NAGA_UNIMPLEMENTED literal;
		struct {
			VectorSize reject;
			VectorSize accept;
		} select_vec_reject_accept_size_mismatch;
	} data;
} ConstantEvaluatorError;

typedef enum BoundsCheckPolicy {
	BoundsCheckPolicy_Restrict,
	BoundsCheckPolicy_ReadZeroSkipWrite,
	BoundsCheckPolicy_Unchecked,
} BoundsCheckPolicy;

typedef struct BoundsCheckPolicies {
	BoundsCheckPolicy index;
	BoundsCheckPolicy buffer;
	BoundsCheckPolicy image_load;
	BoundsCheckPolicy binding_array;
} BoundsCheckPolicies;

typedef enum ResolveArraySizeError {
	ResolveArraySizeError_ExpectedPositiveArrayLength,
	ResolveArraySizeError_NonConstArrayLength,
} ResolveArraySizeError;

// --- naga::valid ---

typedef struct Validator {
	void *_inner_validator;
} Validator;

typedef enum Capabilities {
	Capabilities_IMMEDIATES = 0x1,
	Capabilities_FLOAT64 = 0x2,
	Capabilities_PRIMITIVE_INDEX = 0x4,
	Capabilities_TEXTURE_AND_SAMPLER_BINDING_ARRAY = 0x8,
	Capabilities_BUFFER_BINDING_ARRAY = 0x10,
	Capabilities_STORAGE_TEXTURE_BINDING_ARRAY = 0x20,
	Capabilities_STORAGE_BUFFER_BINDING_ARRAY = 0x40,
	Capabilities_CLIP_DISTANCE = 0x80,
	Capabilities_CULL_DISTANCE = 0x100,
	Capabilities_STORAGE_TEXTURE_16BIT_NORM_FORMATS = 0x200,
	Capabilities_MULTIVIEW = 0x400,
	Capabilities_EARLY_DEPTH_TEST = 0x800,
	Capabilities_MULTISAMPLED_SHADING = 0x1000,
	Capabilities_RAY_QUERY = 0x2000,
	Capabilities_DUAL_SOURCE_BLENDING = 0x4000,
	Capabilities_CUBE_ARRAY_TEXTURES = 0x8000,
	Capabilities_SHADER_INT64 = 0x10000,
	Capabilities_SUBGROUP = 0x20000,
	Capabilities_SUBGROUP_BARRIER = 0x40000,
	Capabilities_SUBGROUP_VERTEX_STAGE = 0x80000,
	Capabilities_SHADER_INT64_ATOMIC_MIN_MAX = 0x100000,
	Capabilities_SHADER_INT64_ATOMIC_ALL_OPS = 0x200000,
	Capabilities_SHADER_FLOAT32_ATOMIC = 0x400000,
	Capabilities_TEXTURE_ATOMIC = 0x800000,
	Capabilities_TEXTURE_INT64_ATOMIC = 0x1000000,
	Capabilities_RAY_HIT_VERTEX_POSITION = 0x2000000,
	Capabilities_SHADER_FLOAT16 = 0x4000000,
	Capabilities_TEXTURE_EXTERNAL = 0x8000000,
	Capabilities_SHADER_FLOAT16_IN_FLOAT32 = 0x10000000,
	Capabilities_SHADER_BARYCENTRICS = 0x20000000,
	Capabilities_MESH_SHADER = 0x40000000,
	Capabilities_MESH_SHADER_POINT_TOPOLOGY = 0x80000000,
	Capabilities_TEXTURE_AND_SAMPLER_BINDING_ARRAY_NON_UNIFORM_INDEXING = 0x100000000,
	Capabilities_BUFFER_BINDING_ARRAY_NON_UNIFORM_INDEXING = 0x200000000,
	Capabilities_STORAGE_TEXTURE_BINDING_ARRAY_NON_UNIFORM_INDEXING = 0x400000000,
	Capabilities_STORAGE_BUFFER_BINDING_ARRAY_NON_UNIFORM_INDEXING = 0x800000000,
} Capabilities;

typedef enum ValidationFlags {
	ValidationFlags_EXPRESSIONS = 0x1,
	ValidationFlags_BLOCKS = 0x2,
	ValidationFlags_CONTROL_FLOW_UNIFORMITY = 0x4,
	ValidationFlags_STRUCT_LAYOUTS = 0x8,
	ValidationFlags_CONSTANTS = 0x10,
	ValidationFlags_BINDINGS = 0x20,
} ValidationFlags;

// --- naga::back::dot ---

typedef struct DOTBackOptions {
	bool cfg_only;
} DOTBackOptions;

// --- naga::back::glsl ---

typedef enum GLSLBackVersionTag {
	GLSLBackVersionTag_Desktop,
	GLSLBackVersionTag_Embedded,
} GLSLBackVersionTag;

typedef struct GLSLBackVersion {
	GLSLBackVersionTag tag;
	union {
		uint16_t desktop;
		struct {
			uint16_t version;
			bool is_webgl;
		} embedded;
	} data;
} GLSLBackVersion;

typedef enum GLSLBackWriterFlags {
	GLSLBackWriterFlags_ADJUST_COORDINATE_SPACE = 0x1,
	GLSLBackWriterFlags_TEXTURE_SHADOW_LOD = 0x2,
	GLSLBackWriterFlags_DRAW_PARAMETERS = 0x4,
	GLSLBackWriterFlags_INCLUDE_UNUSED_ITEMS = 0x10,
	GLSLBackWriterFlags_FORCE_POINT_SIZE = 0x20,
} GLSLBackWriterFlags;

typedef struct GLSLBackBindingMapEntry {
	ResourceBinding key;
	uint8_t value;
} GLSLBackBindingMapEntry;

typedef struct GLSLBackBindingMap {
	GLSLBackBindingMapEntry *entries;
	size_t entries_len;
} GLSLBackBindingMap;

typedef struct GLSLBackOptions {
	GLSLBackVersion version;
	GLSLBackWriterFlags writer_flags;
	GLSLBackBindingMap binding_map;
	bool zero_initialize_workgroup_memory;
} GLSLBackOptions;

typedef struct GLSLBackPipelineOptions {
	ShaderStage shader_stage;
	char *entry_point;
	DEFINE_OPTIONAL(uint32_t)
	multiview;
} GLSLBackPipelineOptions;

typedef enum GLSLBackFeatures {
	GLSLBackFeatures_BUFFER_STORAGE = 0x1,
	GLSLBackFeatures_ARRAY_OF_ARRAYS = 0x2,
	GLSLBackFeatures_DOUBLE_TYPE = 0x4,
	GLSLBackFeatures_FULL_IMAGE_FORMATS = 0x8,
	GLSLBackFeatures_MULTISAMPLED_TEXTURES = 0x10,
	GLSLBackFeatures_MULTISAMPLED_TEXTURE_ARRAYS = 0x20,
	GLSLBackFeatures_CUBE_TEXTURES_ARRAY = 0x40,
	GLSLBackFeatures_COMPUTE_SHADER = 0x80,
	GLSLBackFeatures_IMAGE_LOAD_STORE = 0x100,
	GLSLBackFeatures_CONSERVATIVE_DEPTH = 0x200,
	GLSLBackFeatures_NOPERSPECTIVE_QUALIFIER = 0x800,
	GLSLBackFeatures_SAMPLE_QUALIFIER = 0x1000,
	GLSLBackFeatures_CLIP_DISTANCE = 0x2000,
	GLSLBackFeatures_CULL_DISTANCE = 0x4000,
	GLSLBackFeatures_SAMPLE_VARIABLES = 0x8000,
	GLSLBackFeatures_DYNAMIC_ARRAY_SIZE = 0x10000,
	GLSLBackFeatures_MULTI_VIEW = 0x20000,
	GLSLBackFeatures_TEXTURE_SAMPLES = 0x40000,
	GLSLBackFeatures_TEXTURE_LEVELS = 0x80000,
	GLSLBackFeatures_IMAGE_SIZE = 0x100000,
	GLSLBackFeatures_DUAL_SOURCE_BLENDING = 0x200000,
	GLSLBackFeatures_INSTANCE_INDEX = 0x400000,
	GLSLBackFeatures_TEXTURE_SHADOW_LOD = 0x800000,
	GLSLBackFeatures_SUBGROUP_OPERATIONS = 0x1000000,
	GLSLBackFeatures_TEXTURE_ATOMICS = 0x2000000,
	GLSLBackFeatures_SHADER_BARYCENTRICS = 0x4000000,
} GLSLBackFeatures;

typedef enum GLSLBackErrorTag {
	GLSLBackErrorTag_FmtError,
	GLSLBackErrorTag_MissingFeatures,
	GLSLBackErrorTag_MultipleImmediateData,
	GLSLBackErrorTag_VersionNotSupported,
	GLSLBackErrorTag_EntryPointNotFound,
	GLSLBackErrorTag_UnsupportedExternal,
	GLSLBackErrorTag_UnsupportedScalar,
	GLSLBackErrorTag_ImageMultipleSamplers,
	GLSLBackErrorTag_Custom,
	GLSLBackErrorTag_Override,
	GLSLBackErrorTag_FirstSamplingNotSupported,
	GLSLBackErrorTag_ResolveArraySizeError,
} GLSLBackErrorTag;

typedef struct GLSLBackError {
	GLSLBackErrorTag tag;
	union {
		char *fmt_error;
		GLSLBackFeatures missing_features;
		char *unsupported_external;
		Scalar unsupported_scalar;
		char *custom;
		ResolveArraySizeError resolve_array_size_error;
	} data;
} GLSLBackError;

typedef struct Empty GLSLBackReflectionInfo NAGA_UNIMPLEMENTED;

// --- naga::back::hlsl ---

typedef enum HLSLBackShaderModel {
	HLSLBackShaderModel_V5_0,
	HLSLBackShaderModel_V5_1,
	HLSLBackShaderModel_V6_0,
	HLSLBackShaderModel_V6_1,
	HLSLBackShaderModel_V6_2,
	HLSLBackShaderModel_V6_3,
	HLSLBackShaderModel_V6_4,
	HLSLBackShaderModel_V6_5,
	HLSLBackShaderModel_V6_6,
	HLSLBackShaderModel_V6_7,
} HLSLBackShaderModel;

typedef struct HLSLBackBindTarget {
	uint8_t space;
	uint32_t register_;
	DEFINE_OPTIONAL(uint32_t)
	binding_array_size;
	DEFINE_OPTIONAL(uint32_t)
	dynamic_storage_buffer_offsets_index;
	bool restrict_indexing;
} HLSLBackBindTarget;

typedef struct HLSLBackBindingMapEntry {
	ResourceBinding key;
	HLSLBackBindTarget value;
} HLSLBackBindingMapEntry;

typedef struct HLSLBackBindingMap {
	HLSLBackBindingMapEntry *entries;
	size_t entries_len;
} HLSLBackBindingMap;

typedef struct HLSLBackSamplerHeapBindTargets {
	HLSLBackBindTarget standard_samplers;
	HLSLBackBindTarget comparison_samplers;
} HLSLBackSamplerHeapBindTargets;

typedef struct HLSLBackSamplerIndexBufferKey {
	uint32_t group;
} HLSLBackSamplerIndexBufferKey;

typedef struct HLSLBackSamplerIndexBufferBindingMapEntry {
	HLSLBackSamplerIndexBufferKey key;
	HLSLBackBindTarget value;
} HLSLBackSamplerIndexBufferBindingMapEntry;

typedef struct HLSLBackSamplerIndexBufferBindingMap {
	HLSLBackSamplerIndexBufferBindingMapEntry *entries;
	size_t entries_len;
} HLSLBackSamplerIndexBufferBindingMap;

typedef struct HLSLBackOffsetsBindTarget {
	uint8_t space;
	uint32_t register_;
	uint32_t size;
} HLSLBackOffsetsBindTarget;

typedef struct HLSLBackDynamicStorageBufferOffsetsTargetsEntry {
	uint32_t key;
	HLSLBackOffsetsBindTarget value;
} HLSLBackDynamicStorageBufferOffsetsTargetsEntry;

typedef struct HLSLBackDynamicStorageBufferOffsetsTargets {
	HLSLBackDynamicStorageBufferOffsetsTargetsEntry *entries;
	size_t entries_len;
} HLSLBackDynamicStorageBufferOffsetsTargets;

typedef struct HLSLBackExternalTextureBindTarget {
	HLSLBackBindTarget planes[3];
	HLSLBackBindTarget params;
} HLSLBackExternalTextureBindTarget;

typedef struct HLSLBackExternalTextureBindingMapEntry {
	ResourceBinding key;
	HLSLBackExternalTextureBindTarget value;
} HLSLBackExternalTextureBindingMapEntry;

typedef struct HLSLBackExternalTextureBindingMap {
	HLSLBackExternalTextureBindingMapEntry *entries;
	size_t entries_len;
} HLSLBackExternalTextureBindingMap;

typedef struct HLSLBackOptions {
	HLSLBackShaderModel shader_model;
	HLSLBackBindingMap binding_map;
	bool fake_missing_bindings;
	DEFINE_OPTIONAL(HLSLBackBindTarget)
	special_constants_binding;
	DEFINE_OPTIONAL(HLSLBackBindTarget)
	immediates_target;
	HLSLBackSamplerHeapBindTargets sampler_heap_target;
	HLSLBackSamplerIndexBufferBindingMap sampler_buffer_binding_map;
	HLSLBackDynamicStorageBufferOffsetsTargets dynamic_storage_buffer_offsets_targets;
	HLSLBackExternalTextureBindingMap external_texture_binding_map;
	bool zero_initialize_workgroup_memory;
	bool restrict_indexing;
	bool force_loop_bounding;
} HLSLBackOptions;

typedef struct ShaderStageString {
	ShaderStage shader_stage;
	char *string;

} ShaderStageString;

typedef struct HLSLBackPipelineOptions {
	DEFINE_OPTIONAL(ShaderStageString)
	entry_point;
} HLSLBackPipelineOptions;

typedef enum HLSLBackErrorTag {
	HLSLBackErrorTag_IoError,
	HLSLBackErrorTag_UnsupportedScalar,
	HLSLBackErrorTag_Unimplemented,
	HLSLBackErrorTag_Custom,
	HLSLBackErrorTag_Override,
	HLSLBackErrorTag_ResolveArraySizeError,
	HLSLBackErrorTag_EntryPointNotFound,
	HLSLBackErrorTag_ShaderModelTooLow,
} HLSLBackErrorTag;

typedef struct HLSLBackError {
	HLSLBackErrorTag tag;
	union {
		char *io_error;
		Scalar unsupported_scalar;
		char *unimplemented;
		char *custom;
		ResolveArraySizeError resolve_array_size_error;
		struct {
			ShaderStage stage;
			char *name;
		} entry_point_not_found;
		struct {
			char *message;
			HLSLBackShaderModel model;
		} shader_model_too_low;
	} data;
} HLSLBackError;

typedef struct HLSLBackFragmentEntryPoint {
	struct Empty *NAGA_UNIMPLEMENTED module;
	struct Empty *NAGA_UNIMPLEMENTED func;
} HLSLBackFragmentEntryPoint;

typedef struct Empty HLSLBackReflectionInfo NAGA_UNIMPLEMENTED;

// --- naga::back::msl ---

typedef enum MSLBackSamplerAddress {
	MSLBackSamplerAddress_Repeat,
	MSLBackSamplerAddress_MirroredRepeat,
	MSLBackSamplerAddress_ClampToEdge,
	MSLBackSamplerAddress_ClampToZero,
	MSLBackSamplerAddress_ClampToBorder,
} MSLBackSamplerAddress;

typedef enum MSLBackSamplerBorderColor {
	MSLBackSamplerBorderColor_TransparentBlack,
	MSLBackSamplerBorderColor_OpaqueBlack,
	MSLBackSamplerBorderColor_OpaqueWhite,
} MSLBackSamplerBorderColor;

typedef enum MSLBackSamplerCompareFunc {
	MSLBackSamplerCompareFunc_Never,
	MSLBackSamplerCompareFunc_Less,
	MSLBackSamplerCompareFunc_LessEqual,
	MSLBackSamplerCompareFunc_Greater,
	MSLBackSamplerCompareFunc_GreaterEqual,
	MSLBackSamplerCompareFunc_Equal,
	MSLBackSamplerCompareFunc_NotEqual,
	MSLBackSamplerCompareFunc_Always,
} MSLBackSamplerCompareFunc;

typedef enum MSLBackSamplerCoord {
	MSLBackSamplerCoord_Normalized,
	MSLBackSamplerCoord_Pixel,
} MSLBackSamplerCoord;

typedef enum MSLBackSamplerFilter {
	MSLBackSamplerFilter_Nearest,
	MSLBackSamplerFilter_Linear,
} MSLBackSamplerFilter;

typedef struct MSLBackFloatRange {
	float start;
	float end;
} MSLBackFloatFloat;

typedef struct MSLBackInlineSampler {
	MSLBackSamplerCoord coord;
	MSLBackSamplerAddress address[3];
	MSLBackSamplerBorderColor border_color;
	MSLBackSamplerFilter mag_filter;
	MSLBackSamplerFilter min_filter;
	DEFINE_OPTIONAL(MSLBackSamplerFilter)
	mip_filter;
	DEFINE_OPTIONAL(MSLBackFloatFloat)
	lod_clamp;
	DEFINE_OPTIONAL(uint32_t)
	max_anisotropy;
	MSLBackSamplerCompareFunc compare_func;
} MSLBackInlineSampler;

typedef uint8_t MSLBackSlot;

typedef enum MSLBackVertexFormat {
	MSLBackVertexFormat_Uint8 = 0,
	MSLBackVertexFormat_Uint8x2 = 1,
	MSLBackVertexFormat_Uint8x4 = 2,
	MSLBackVertexFormat_Sint8 = 3,
	MSLBackVertexFormat_Sint8x2 = 4,
	MSLBackVertexFormat_Sint8x4 = 5,
	MSLBackVertexFormat_Unorm8 = 6,
	MSLBackVertexFormat_Unorm8x2 = 7,
	MSLBackVertexFormat_Unorm8x4 = 8,
	MSLBackVertexFormat_Snorm8 = 9,
	MSLBackVertexFormat_Snorm8x2 = 10,
	MSLBackVertexFormat_Snorm8x4 = 11,
	MSLBackVertexFormat_Uint16 = 12,
	MSLBackVertexFormat_Uint16x2 = 13,
	MSLBackVertexFormat_Uint16x4 = 14,
	MSLBackVertexFormat_Sint16 = 15,
	MSLBackVertexFormat_Sint16x2 = 16,
	MSLBackVertexFormat_Sint16x4 = 17,
	MSLBackVertexFormat_Unorm16 = 18,
	MSLBackVertexFormat_Unorm16x2 = 19,
	MSLBackVertexFormat_Unorm16x4 = 20,
	MSLBackVertexFormat_Snorm16 = 21,
	MSLBackVertexFormat_Snorm16x2 = 22,
	MSLBackVertexFormat_Snorm16x4 = 23,
	MSLBackVertexFormat_Float16 = 24,
	MSLBackVertexFormat_Float16x2 = 25,
	MSLBackVertexFormat_Float16x4 = 26,
	MSLBackVertexFormat_Float32 = 27,
	MSLBackVertexFormat_Float32x2 = 28,
	MSLBackVertexFormat_Float32x3 = 29,
	MSLBackVertexFormat_Float32x4 = 30,
	MSLBackVertexFormat_Uint32 = 31,
	MSLBackVertexFormat_Uint32x2 = 32,
	MSLBackVertexFormat_Uint32x3 = 33,
	MSLBackVertexFormat_Uint32x4 = 34,
	MSLBackVertexFormat_Sint32 = 35,
	MSLBackVertexFormat_Sint32x2 = 36,
	MSLBackVertexFormat_Sint32x3 = 37,
	MSLBackVertexFormat_Sint32x4 = 38,
	MSLBackVertexFormat_Unorm10_10_10_2 = 43,
	MSLBackVertexFormat_Unorm8x4Bgra = 44,
} MSLBackVertexFormat;

typedef enum MSLBackVertexBufferStepMode {
	MSLBackVertexBufferStepMode_Constant,
	MSLBackVertexBufferStepMode_ByVertex,
	MSLBackVertexBufferStepMode_ByInstance,
} MSLBackVertexBufferStepMode;

typedef struct MSLBackAttributeMapping {
	uint32_t shader_location;
	uint32_t offset;
	MSLBackVertexFormat format;
} MSLBackAttributeMapping;

typedef struct MSLBackBindExternalTextureTarget {
	MSLBackSlot planes[3];
	MSLBackSlot params;
} MSLBackBindExternalTextureTarget;

typedef enum MSLBackBindSamplerTargetTag {
	MSLBackBindSamplerTargetTag_Resource,
	MSLBackBindSamplerTargetTag_Inline,
} MSLBackBindSamplerTargetTag;

typedef struct MSLBackBindSamplerTarget {
	MSLBackBindSamplerTargetTag tag;
	union {
		MSLBackSlot resource;
		uint8_t inline_;
	} data;
} MSLBackBindSamplerTarget;

typedef struct MSLBackBindTarget {
	DEFINE_OPTIONAL(MSLBackSlot)
	buffer;
	DEFINE_OPTIONAL(MSLBackSlot)
	texture;
	DEFINE_OPTIONAL(MSLBackBindSamplerTarget)
	sampler;
	DEFINE_OPTIONAL(MSLBackBindExternalTextureTarget)
	external_texture;
	bool mutable_;
} MSLBackBindTarget;

typedef struct MSLBackBindingMapEntry {
	ResourceBinding key;
	MSLBackBindTarget value;
} MSLBackBindingMapEntry;

typedef struct MSLBackBindingMap {
	MSLBackBindingMapEntry *entries;
	size_t entries_len;
} MSLBackBindingMap;

typedef struct MSLBackEntryPointResources {
	MSLBackBindingMap resources;
	DEFINE_OPTIONAL(MSLBackSlot)
	immediates_buffer;
	DEFINE_OPTIONAL(MSLBackSlot)
	sizes_buffer;
} MSLBackEntryPointResources;

typedef struct MSLBackEntryPointResourceMapEntry {
	char *key;
	MSLBackEntryPointResources value;
} MSLBackEntryPointResourceMapEntry;

typedef struct MSLBackEntryPointResourceMap {
	MSLBackEntryPointResourceMapEntry *entries;
	size_t entries_len;
} MSLBackEntryPointResourceMap;

typedef struct MSLBackOptions {
	uint8_t lang_version[2];
	MSLBackEntryPointResourceMap per_entry_point_map;
	MSLBackInlineSampler *inline_samplers;
	size_t inline_samplers_len;
	bool spirv_cross_compatibility;
	bool fake_missing_bindings;
	BoundsCheckPolicies bounds_check_policies;
	bool zero_initialize_workgroup_memory;
	bool force_loop_bounding;
} MSLBackOptions;

typedef struct MSLBackVertexBufferMapping {
	uint32_t id;
	uint32_t stride;
	MSLBackVertexBufferStepMode step_mode;
	MSLBackAttributeMapping *attributes;
	size_t attributes_len;
} MSLBackVertexBufferMapping;

typedef struct MSLBackPipelineOptions {
	DEFINE_OPTIONAL(ShaderStageString)
	entry_point;
	bool allow_and_force_point_size;
	bool vertex_pulling_transform;
	MSLBackVertexBufferMapping *vertex_buffer_mappings;
	size_t vertex_buffer_mappings_len;
} MSLBackPipelineOptions;

typedef enum MSLBackEntryPointErrorTag {
	MSLBackEntryPointErrorTag_MissingBinding,
	MSLBackEntryPointErrorTag_MissingBindTarget,
	MSLBackEntryPointErrorTag_MissingImmediateData,
	MSLBackEntryPointErrorTag_MissingSizesBuffer,
} MSLBackEntryPointErrorTag;

typedef struct MSLBackEntryPointError {
	MSLBackEntryPointErrorTag tag;
	union {
		char *missing_binding;
		ResourceBinding missing_bind_target;
	} data;
} MSLBackEntryPointError;

typedef enum MSLBackErrorTag {
	MSLBackErrorTag_Format,
	MSLBackErrorTag_UnimplementedBindTarget,
	MSLBackErrorTag_UnsupportedCompose,
	MSLBackErrorTag_UnsupportedBinaryOp,
	MSLBackErrorTag_UnsupportedCall,
	MSLBackErrorTag_FeatureNotImplemented,
	MSLBackErrorTag_GenericValidation,
	MSLBackErrorTag_UnsupportedBuiltIn,
	MSLBackErrorTag_CapabilityNotSupported,
	MSLBackErrorTag_UnsupportedAttribute,
	MSLBackErrorTag_UnsupportedFunction,
	MSLBackErrorTag_UnsupportedWriteableStorageBuffer,
	MSLBackErrorTag_UnsupportedWriteableStorageTexture,
	MSLBackErrorTag_UnsupportedRWStorageTexture,
	MSLBackErrorTag_UnsupportedArrayOf,
	MSLBackErrorTag_UnsupportedArrayOfType,
	MSLBackErrorTag_UnsupportedRayTracing,
	MSLBackErrorTag_Override,
	MSLBackErrorTag_UnsupportedBitCast,
	MSLBackErrorTag_ResolveArraySizeError,
	MSLBackErrorTag_EntryPointNotFound,
} MSLBackErrorTag;

typedef struct MSLBackError {
	MSLBackErrorTag tag;
	union {
		char *format;
		MSLBackBindTarget unimplemented_bind_target;
		DEFINE_HANDLE_INDEX(Type)
		unsupported_compose;
		struct Empty *NAGA_UNIMPLEMENTED unsupported_binary_op;
		char *unsupported_call;
		char *feature_not_implemented;
		char *generic_validation;
		BuiltIn unsupported_built_in;
		Capabilities capability_not_supported;
		char *unsupported_attribute;
		char *unsupported_function;
		ShaderStage unsupported_writeable_storage_texture;
		char *unsupported_array_of;
		DEFINE_HANDLE_INDEX(Type)
		unsupported_array_of_type;
		TypeInner unsupported_bit_cast;
		ResolveArraySizeError resolve_array_size_error;
		ShaderStageString entry_point_not_found;
	} data;
} MSLBackError;

typedef struct Empty MSLBackTranslationInfo NAGA_UNIMPLEMENTED;

// --- naga::back::spv ---

typedef enum SPVBackCapability {
	SPVBackCapability_Matrix = 0,
	SPVBackCapability_Shader = 1,
	SPVBackCapability_Geometry = 2,
	SPVBackCapability_Tessellation = 3,
	SPVBackCapability_Addresses = 4,
	SPVBackCapability_Linkage = 5,
	SPVBackCapability_Kernel = 6,
	SPVBackCapability_Vector16 = 7,
	SPVBackCapability_Float16Buffer = 8,
	SPVBackCapability_Float16 = 9,
	SPVBackCapability_Float64 = 10,
	SPVBackCapability_Int64 = 11,
	SPVBackCapability_Int64Atomics = 12,
	SPVBackCapability_ImageBasic = 13,
	SPVBackCapability_ImageReadWrite = 14,
	SPVBackCapability_ImageMipmap = 15,
	SPVBackCapability_Pipes = 17,
	SPVBackCapability_Groups = 18,
	SPVBackCapability_DeviceEnqueue = 19,
	SPVBackCapability_LiteralSampler = 20,
	SPVBackCapability_AtomicStorage = 21,
	SPVBackCapability_Int16 = 22,
	SPVBackCapability_TessellationPointSize = 23,
	SPVBackCapability_GeometryPointSize = 24,
	SPVBackCapability_ImageGatherExtended = 25,
	SPVBackCapability_StorageImageMultisample = 27,
	SPVBackCapability_UniformBufferArrayDynamicIndexing = 28,
	SPVBackCapability_SampledImageArrayDynamicIndexing = 29,
	SPVBackCapability_StorageBufferArrayDynamicIndexing = 30,
	SPVBackCapability_StorageImageArrayDynamicIndexing = 31,
	SPVBackCapability_ClipDistance = 32,
	SPVBackCapability_CullDistance = 33,
	SPVBackCapability_ImageCubeArray = 34,
	SPVBackCapability_SampleRateShading = 35,
	SPVBackCapability_ImageRect = 36,
	SPVBackCapability_SampledRect = 37,
	SPVBackCapability_GenericPointer = 38,
	SPVBackCapability_Int8 = 39,
	SPVBackCapability_InputAttachment = 40,
	SPVBackCapability_SparseResidency = 41,
	SPVBackCapability_MinLod = 42,
	SPVBackCapability_Sampled1D = 43,
	SPVBackCapability_Image1D = 44,
	SPVBackCapability_SampledCubeArray = 45,
	SPVBackCapability_SampledBuffer = 46,
	SPVBackCapability_ImageBuffer = 47,
	SPVBackCapability_ImageMSArray = 48,
	SPVBackCapability_StorageImageExtendedFormats = 49,
	SPVBackCapability_ImageQuery = 50,
	SPVBackCapability_DerivativeControl = 51,
	SPVBackCapability_InterpolationFunction = 52,
	SPVBackCapability_TransformFeedback = 53,
	SPVBackCapability_GeometryStreams = 54,
	SPVBackCapability_StorageImageReadWithoutFormat = 55,
	SPVBackCapability_StorageImageWriteWithoutFormat = 56,
	SPVBackCapability_MultiViewport = 57,
	SPVBackCapability_SubgroupDispatch = 58,
	SPVBackCapability_NamedBarrier = 59,
	SPVBackCapability_PipeStorage = 60,
	SPVBackCapability_GroupNonUniform = 61,
	SPVBackCapability_GroupNonUniformVote = 62,
	SPVBackCapability_GroupNonUniformArithmetic = 63,
	SPVBackCapability_GroupNonUniformBallot = 64,
	SPVBackCapability_GroupNonUniformShuffle = 65,
	SPVBackCapability_GroupNonUniformShuffleRelative = 66,
	SPVBackCapability_GroupNonUniformClustered = 67,
	SPVBackCapability_GroupNonUniformQuad = 68,
	SPVBackCapability_ShaderLayer = 69,
	SPVBackCapability_ShaderViewportIndex = 70,
	SPVBackCapability_UniformDecoration = 71,
	SPVBackCapability_CoreBuiltinsARM = 4165,
	SPVBackCapability_TileImageColorReadAccessEXT = 4166,
	SPVBackCapability_TileImageDepthReadAccessEXT = 4167,
	SPVBackCapability_TileImageStencilReadAccessEXT = 4168,
	SPVBackCapability_FragmentShadingRateKHR = 4422,
	SPVBackCapability_SubgroupBallotKHR = 4423,
	SPVBackCapability_DrawParameters = 4427,
	SPVBackCapability_WorkgroupMemoryExplicitLayoutKHR = 4428,
	SPVBackCapability_WorkgroupMemoryExplicitLayout8BitAccessKHR = 4429,
	SPVBackCapability_WorkgroupMemoryExplicitLayout16BitAccessKHR = 4430,
	SPVBackCapability_SubgroupVoteKHR = 4431,
	SPVBackCapability_StorageBuffer16BitAccess = 4433,
	SPVBackCapability_UniformAndStorageBuffer16BitAccess = 4434,
	SPVBackCapability_StoragePushConstant16 = 4435,
	SPVBackCapability_StorageInputOutput16 = 4436,
	SPVBackCapability_DeviceGroup = 4437,
	SPVBackCapability_MultiView = 4439,
	SPVBackCapability_VariablePointersStorageBuffer = 4441,
	SPVBackCapability_VariablePointers = 4442,
	SPVBackCapability_AtomicStorageOps = 4445,
	SPVBackCapability_SampleMaskPostDepthCoverage = 4447,
	SPVBackCapability_StorageBuffer8BitAccess = 4448,
	SPVBackCapability_UniformAndStorageBuffer8BitAccess = 4449,
	SPVBackCapability_StoragePushConstant8 = 4450,
	SPVBackCapability_DenormPreserve = 4464,
	SPVBackCapability_DenormFlushToZero = 4465,
	SPVBackCapability_SignedZeroInfNanPreserve = 4466,
	SPVBackCapability_RoundingModeRTE = 4467,
	SPVBackCapability_RoundingModeRTZ = 4468,
	SPVBackCapability_RayQueryProvisionalKHR = 4471,
	SPVBackCapability_RayQueryKHR = 4472,
	SPVBackCapability_RayTraversalPrimitiveCullingKHR = 4478,
	SPVBackCapability_RayTracingKHR = 4479,
	SPVBackCapability_TextureSampleWeightedQCOM = 4484,
	SPVBackCapability_TextureBoxFilterQCOM = 4485,
	SPVBackCapability_TextureBlockMatchQCOM = 4486,
	SPVBackCapability_Float16ImageAMD = 5008,
	SPVBackCapability_ImageGatherBiasLodAMD = 5009,
	SPVBackCapability_FragmentMaskAMD = 5010,
	SPVBackCapability_StencilExportEXT = 5013,
	SPVBackCapability_ImageReadWriteLodAMD = 5015,
	SPVBackCapability_Int64ImageEXT = 5016,
	SPVBackCapability_ShaderClockKHR = 5055,
	SPVBackCapability_ShaderEnqueueAMDX = 5067,
	SPVBackCapability_SampleMaskOverrideCoverageNV = 5249,
	SPVBackCapability_GeometryShaderPassthroughNV = 5251,
	SPVBackCapability_ShaderViewportIndexLayerEXT = 5254,
	SPVBackCapability_ShaderViewportMaskNV = 5255,
	SPVBackCapability_ShaderStereoViewNV = 5259,
	SPVBackCapability_PerViewAttributesNV = 5260,
	SPVBackCapability_FragmentFullyCoveredEXT = 5265,
	SPVBackCapability_MeshShadingNV = 5266,
	SPVBackCapability_ImageFootprintNV = 5282,
	SPVBackCapability_MeshShadingEXT = 5283,
	SPVBackCapability_FragmentBarycentricKHR = 5284,
	SPVBackCapability_ComputeDerivativeGroupQuadsNV = 5288,
	SPVBackCapability_FragmentDensityEXT = 5291,
	SPVBackCapability_GroupNonUniformPartitionedNV = 5297,
	SPVBackCapability_ShaderNonUniform = 5301,
	SPVBackCapability_RuntimeDescriptorArray = 5302,
	SPVBackCapability_InputAttachmentArrayDynamicIndexing = 5303,
	SPVBackCapability_UniformTexelBufferArrayDynamicIndexing = 5304,
	SPVBackCapability_StorageTexelBufferArrayDynamicIndexing = 5305,
	SPVBackCapability_UniformBufferArrayNonUniformIndexing = 5306,
	SPVBackCapability_SampledImageArrayNonUniformIndexing = 5307,
	SPVBackCapability_StorageBufferArrayNonUniformIndexing = 5308,
	SPVBackCapability_StorageImageArrayNonUniformIndexing = 5309,
	SPVBackCapability_InputAttachmentArrayNonUniformIndexing = 5310,
	SPVBackCapability_UniformTexelBufferArrayNonUniformIndexing = 5311,
	SPVBackCapability_StorageTexelBufferArrayNonUniformIndexing = 5312,
	SPVBackCapability_RayTracingPositionFetchKHR = 5336,
	SPVBackCapability_RayTracingNV = 5340,
	SPVBackCapability_RayTracingMotionBlurNV = 5341,
	SPVBackCapability_VulkanMemoryModel = 5345,
	SPVBackCapability_VulkanMemoryModelDeviceScope = 5346,
	SPVBackCapability_PhysicalStorageBufferAddresses = 5347,
	SPVBackCapability_ComputeDerivativeGroupLinearNV = 5350,
	SPVBackCapability_RayTracingProvisionalKHR = 5353,
	SPVBackCapability_CooperativeMatrixNV = 5357,
	SPVBackCapability_FragmentShaderSampleInterlockEXT = 5363,
	SPVBackCapability_FragmentShaderShadingRateInterlockEXT = 5372,
	SPVBackCapability_ShaderSMBuiltinsNV = 5373,
	SPVBackCapability_FragmentShaderPixelInterlockEXT = 5378,
	SPVBackCapability_DemoteToHelperInvocation = 5379,
	SPVBackCapability_DisplacementMicromapNV = 5380,
	SPVBackCapability_RayTracingOpacityMicromapEXT = 5381,
	SPVBackCapability_ShaderInvocationReorderNV = 5383,
	SPVBackCapability_BindlessTextureNV = 5390,
	SPVBackCapability_RayQueryPositionFetchKHR = 5391,
	SPVBackCapability_RayTracingDisplacementMicromapNV = 5409,
	SPVBackCapability_SubgroupShuffleINTEL = 5568,
	SPVBackCapability_SubgroupBufferBlockIOINTEL = 5569,
	SPVBackCapability_SubgroupImageBlockIOINTEL = 5570,
	SPVBackCapability_SubgroupImageMediaBlockIOINTEL = 5579,
	SPVBackCapability_RoundToInfinityINTEL = 5582,
	SPVBackCapability_FloatingPointModeINTEL = 5583,
	SPVBackCapability_IntegerFunctions2INTEL = 5584,
	SPVBackCapability_FunctionPointersINTEL = 5603,
	SPVBackCapability_IndirectReferencesINTEL = 5604,
	SPVBackCapability_AsmINTEL = 5606,
	SPVBackCapability_AtomicFloat32MinMaxEXT = 5612,
	SPVBackCapability_AtomicFloat64MinMaxEXT = 5613,
	SPVBackCapability_AtomicFloat16MinMaxEXT = 5616,
	SPVBackCapability_VectorComputeINTEL = 5617,
	SPVBackCapability_VectorAnyINTEL = 5619,
	SPVBackCapability_ExpectAssumeKHR = 5629,
	SPVBackCapability_SubgroupAvcMotionEstimationINTEL = 5696,
	SPVBackCapability_SubgroupAvcMotionEstimationIntraINTEL = 5697,
	SPVBackCapability_SubgroupAvcMotionEstimationChromaINTEL = 5698,
	SPVBackCapability_VariableLengthArrayINTEL = 5817,
	SPVBackCapability_FunctionFloatControlINTEL = 5821,
	SPVBackCapability_FPGAMemoryAttributesINTEL = 5824,
	SPVBackCapability_FPFastMathModeINTEL = 5837,
	SPVBackCapability_ArbitraryPrecisionIntegersINTEL = 5844,
	SPVBackCapability_ArbitraryPrecisionFloatingPointINTEL = 5845,
	SPVBackCapability_UnstructuredLoopControlsINTEL = 5886,
	SPVBackCapability_FPGALoopControlsINTEL = 5888,
	SPVBackCapability_KernelAttributesINTEL = 5892,
	SPVBackCapability_FPGAKernelAttributesINTEL = 5897,
	SPVBackCapability_FPGAMemoryAccessesINTEL = 5898,
	SPVBackCapability_FPGAClusterAttributesINTEL = 5904,
	SPVBackCapability_LoopFuseINTEL = 5906,
	SPVBackCapability_FPGADSPControlINTEL = 5908,
	SPVBackCapability_MemoryAccessAliasingINTEL = 5910,
	SPVBackCapability_FPGAInvocationPipeliningAttributesINTEL = 5916,
	SPVBackCapability_FPGABufferLocationINTEL = 5920,
	SPVBackCapability_ArbitraryPrecisionFixedPointINTEL = 5922,
	SPVBackCapability_USMStorageClassesINTEL = 5935,
	SPVBackCapability_RuntimeAlignedAttributeINTEL = 5939,
	SPVBackCapability_IOPipesINTEL = 5943,
	SPVBackCapability_BlockingPipesINTEL = 5945,
	SPVBackCapability_FPGARegINTEL = 5948,
	SPVBackCapability_DotProductInputAll = 6016,
	SPVBackCapability_DotProductInput4x8Bit = 6017,
	SPVBackCapability_DotProductInput4x8BitPacked = 6018,
	SPVBackCapability_DotProduct = 6019,
	SPVBackCapability_RayCullMaskKHR = 6020,
	SPVBackCapability_CooperativeMatrixKHR = 6022,
	SPVBackCapability_BitInstructions = 6025,
	SPVBackCapability_GroupNonUniformRotateKHR = 6026,
	SPVBackCapability_AtomicFloat32AddEXT = 6033,
	SPVBackCapability_AtomicFloat64AddEXT = 6034,
	SPVBackCapability_LongConstantCompositeINTEL = 6089,
	SPVBackCapability_OptNoneINTEL = 6094,
	SPVBackCapability_AtomicFloat16AddEXT = 6095,
	SPVBackCapability_DebugInfoModuleINTEL = 6114,
	SPVBackCapability_BFloat16ConversionINTEL = 6115,
	SPVBackCapability_SplitBarrierINTEL = 6141,
	SPVBackCapability_GlobalVariableFPGADecorationsINTEL = 6146,
	SPVBackCapability_FPGAKernelAttributesv2INTEL = 6161,
	SPVBackCapability_GlobalVariableHostAccessINTEL = 6167,
	SPVBackCapability_FPMaxErrorINTEL = 6169,
	SPVBackCapability_FPGALatencyControlINTEL = 6171,
	SPVBackCapability_FPGAArgumentInterfacesINTEL = 6174,
	SPVBackCapability_GroupUniformArithmeticKHR = 6400,
	SPVBackCapability_CacheControlsINTEL = 6441,
} SPVBackCapability;

typedef struct SPVBackCapabilitySet {
	SPVBackCapability *capabilities;
	size_t capabilities_len;
} SPVBackCapabilities;

typedef enum SPVBackWriterFlags {
	SPVBackWriterFlags_DEBUG = 0x1,
	SPVBackWriterFlags_ADJUST_COORDINATE_SPACE = 0x2,
	SPVBackWriterFlags_LABEL_VARYINGS = 0x4,
	SPVBackWriterFlags_FORCE_POINT_SIZE = 0x8,
	SPVBackWriterFlags_CLAMP_FRAG_DEPTH = 0x10,
	SPVBackWriterFlags_PRINT_ON_RAY_QUERY_INITIALIZATION_FAIL = 0x20,
} SPVBackWriterFlags;

typedef struct SPVBackBindingInfo {
	uint32_t descriptor_set;
	uint32_t binding;
	DEFINE_OPTIONAL(uint32_t)
	binding_array_size;
} SPVBackBindingInfo;

typedef struct SPVBackBindingMapEntry {
	ResourceBinding key;
	SPVBackBindingInfo value;
} SPVBackBindingMapEntry;

typedef struct SPVBackBindingMap {
	SPVBackBindingMapEntry *entries;
	size_t entries_len;
} SPVBackBindingMap;

typedef enum SPVBackZeroInitializeWorkgroupMemoryMode {
	SPVBackZeroInitializeWorkgroupMemoryMode_Native,
	SPVBackZeroInitializeWorkgroupMemoryMode_Polyfill,
	SPVBackZeroInitializeWorkgroupMemoryMode_None,
} SPVBackZeroInitializeWorkgroupMemoryMode;

typedef enum SPVBackSourceLanguage {
	SPVBackSourceLanguage_Unknown = 0,
	SPVBackSourceLanguage_ESSL = 1,
	SPVBackSourceLanguage_GLSL = 2,
	SPVBackSourceLanguage_OpenCL_C = 3,
	SPVBackSourceLanguage_OpenCL_CPP = 4,
	SPVBackSourceLanguage_HLSL = 5,
	SPVBackSourceLanguage_CPP_for_OpenCL = 6,
	SPVBackSourceLanguage_SYCL = 7,
	SPVBackSourceLanguage_HERO_C = 8,
	SPVBackSourceLanguage_NZSL = 9,
	SPVBackSourceLanguage_WGSL = 10,
	SPVBackSourceLanguage_Slang = 11,
} SPVBackSourceLanguage;

typedef struct SPVBackDebugInfo {
	char *source_code;
	char *file_name;
	SPVBackSourceLanguage language;
} SPVBackDebugInfo;

typedef struct SPVBackOptions {
	uint8_t lang_version[2];
	SPVBackWriterFlags flags;
	bool fake_missing_bindings;
	SPVBackBindingMap binding_map;
	DEFINE_OPTIONAL(SPVBackCapabilities)
	capabilities;
	BoundsCheckPolicies bounds_check_policies;
	SPVBackZeroInitializeWorkgroupMemoryMode zero_initialize_workgroup_memory;
	bool force_loop_bounding;
	bool ray_query_initialization_tracking;
	bool use_storage_input_output_16;
	// DEFINE_OPTIONAL(SPVBackDebugInfo)
	// debug_info;
    // NOTE: This type has an awkward lifetime on a borrowed string slice.
    struct Empty* debug_info;
} SPVBackOptions;

typedef struct SPVBackPipelineOptions {
	ShaderStage shader_stage;
	char *entry_point;
} SPVBackPipelineOptions;

typedef enum SPVBackErrorTag {
	SPVBackErrorTag_EntryPointNotFound,
	SPVBackErrorTag_UnsupportedVersion,
	SPVBackErrorTag_MissingCapabilities,
	SPVBackErrorTag_FeatureNotImplemented,
	SPVBackErrorTag_Validation,
	SPVBackErrorTag_Override,
	SPVBackErrorTag_ResolveArraySizeError,
	SPVBackErrorTag_SpirvVersionTooLow,
	SPVBackErrorTag_MissingBinding,
} SPVBackErrorTag;

typedef struct SPVBackError {
	SPVBackErrorTag tag;
	union {
		uint8_t unsupported_version[2];
		struct {
			char *error;
			SPVBackCapability *capabilities;
			uint32_t capabilities_len;
		} missing_capabilities;
		char *feature_not_implemented;
		char *validation;
		ResolveArraySizeError resolve_array_size_error;
		uint8_t spirv_version_too_low[2];
		ResourceBinding missing_binding;
	} data;
} SPVBackError;

// --- naga::back::wgsl ---

typedef enum WGSLBackWriterFlags {
	WGSLBackWriterFlags_EXPLICIT_TYPES = 0x1,
} WGSLBackWriterFlags;

typedef enum WGSLBackErrorTag {
	WGSLBackErrorTag_FmtError,
	WGSLBackErrorTag_Custom,
	WGSLBackErrorTag_Unimplemented,
	WGSLBackErrorTag_UnsupportedRelationalFunction,
	WGSLBackErrorTag_Unsupported,
} WGSLBackErrorTag;

typedef struct WGSLBackError {
	WGSLBackErrorTag tag;
	union {
		char *fmt_error;
		char *custom;
		char *unimplemented;
		RelationalFunction unsupported_relational_function;
		struct {
			char *kind;
			char *value;
		} unsupported;
	} data;
} WGSLBackError;

// -- naga::back::pipeline_constants ---

typedef struct PiplineConstant {
	const char *key;
	double value;
} PipelineConstant;

typedef enum PipelineConstantErrorTag {
	PipelineConstantErrorTag_MissingValue,
	PipelineConstantErrorTag_SrcNeedsToBeFinite,
	PipelineConstantErrorTag_DstRangeTooSmall,
	PipelineConstantErrorTag_ConstantEvaluatorError,
	PipelineConstantErrorTag_ValidationError,
	PipelineConstantErrorTag_NegativeWorkgroupSize,
	PipelineConstantErrorTag_NegativeMeshOutputMax,
} PipelineConstantErrorTag;

typedef struct PipelineConstantError {
	PipelineConstantErrorTag tag;
	union {
		char *missing_value;
		ConstantEvaluatorError constant_evaluator_error;
		struct Empty *NAGA_UNIMPLEMENTED validation_error;
	} data;
} PipelineConstantError;

// --- naga::front::atmoic_upgrade ---

typedef enum AtomicUpgradeFrontError {
	AtomicUpgradeFront_Unsupported,
	AtomicUpgradeFront_UnexpectedEndOfIndices,
	AtomicUpgradeFront_GlobalInitUnsupported,
	AtomicUpgradeFront_GlobalVariableMissing,
	AtomicUpgradeFront_CompareExchangeNonScalarBaseType,
} AtomicUpgradeFrontError;

// --- naga::front::glsl ---

typedef struct GLSLFrontDefinesEntry {
	char *key;
	char *value;
} GLSLFrontDefinesEntry;

typedef struct GLSLFrontDefines {
	GLSLFrontDefinesEntry *entries;
	size_t entries_len;
} GLSLFrontDefines;

typedef struct GLSLFrontOptions {
	ShaderStage stage;
	GLSLFrontDefines defines;
} GLSLFrontOptions;

typedef enum GLSLFrontPrecision {
	Low,
	Medium,
	High,
} GLSLFrontPrecision;

typedef struct GLSLFrontInteger {
	uint64_t value;
	bool signed_;
	int32_t width;

} GLSLFrontInteger;

typedef struct GLSLFrontFloat {
	float value;
	int32_t width;
} GLSLFrontFloat;

typedef enum GLSLFrontTokenValueTag {
	GLSLFrontTokenValueTag_Identifier,
	GLSLFrontTokenValueTag_FloatConstant,
	GLSLFrontTokenValueTag_IntConstant,
	GLSLFrontTokenValueTag_BoolConstant,
	GLSLFrontTokenValueTag_Layout,
	GLSLFrontTokenValueTag_In,
	GLSLFrontTokenValueTag_Out,
	GLSLFrontTokenValueTag_InOut,
	GLSLFrontTokenValueTag_Uniform,
	GLSLFrontTokenValueTag_Buffer,
	GLSLFrontTokenValueTag_Const,
	GLSLFrontTokenValueTag_Shared,
	GLSLFrontTokenValueTag_Restrict,
	GLSLFrontTokenValueTag_MemoryQualifier,
	GLSLFrontTokenValueTag_Invariant,
	GLSLFrontTokenValueTag_Interpolation,
	GLSLFrontTokenValueTag_Sampling,
	GLSLFrontTokenValueTag_Precision,
	GLSLFrontTokenValueTag_PrecisionQualifier,
	GLSLFrontTokenValueTag_Continue,
	GLSLFrontTokenValueTag_Break,
	GLSLFrontTokenValueTag_Return,
	GLSLFrontTokenValueTag_Discard,
	GLSLFrontTokenValueTag_If,
	GLSLFrontTokenValueTag_Else,
	GLSLFrontTokenValueTag_Switch,
	GLSLFrontTokenValueTag_Case,
	GLSLFrontTokenValueTag_Default,
	GLSLFrontTokenValueTag_While,
	GLSLFrontTokenValueTag_Do,
	GLSLFrontTokenValueTag_For,
	GLSLFrontTokenValueTag_Void,
	GLSLFrontTokenValueTag_Struct,
	GLSLFrontTokenValueTag_TypeName,
	GLSLFrontTokenValueTag_Assign,
	GLSLFrontTokenValueTag_AddAssign,
	GLSLFrontTokenValueTag_SubAssign,
	GLSLFrontTokenValueTag_MulAssign,
	GLSLFrontTokenValueTag_DivAssign,
	GLSLFrontTokenValueTag_ModAssign,
	GLSLFrontTokenValueTag_LeftShiftAssign,
	GLSLFrontTokenValueTag_RightShiftAssign,
	GLSLFrontTokenValueTag_AndAssign,
	GLSLFrontTokenValueTag_XorAssign,
	GLSLFrontTokenValueTag_OrAssign,
	GLSLFrontTokenValueTag_Increment,
	GLSLFrontTokenValueTag_Decrement,
	GLSLFrontTokenValueTag_LogicalOr,
	GLSLFrontTokenValueTag_LogicalAnd,
	GLSLFrontTokenValueTag_LogicalXor,
	GLSLFrontTokenValueTag_LessEqual,
	GLSLFrontTokenValueTag_GreaterEqual,
	GLSLFrontTokenValueTag_Equal,
	GLSLFrontTokenValueTag_NotEqual,
	GLSLFrontTokenValueTag_LeftShift,
	GLSLFrontTokenValueTag_RightShift,
	GLSLFrontTokenValueTag_LeftBrace,
	GLSLFrontTokenValueTag_RightBrace,
	GLSLFrontTokenValueTag_LeftParen,
	GLSLFrontTokenValueTag_RightParen,
	GLSLFrontTokenValueTag_LeftBracket,
	GLSLFrontTokenValueTag_RightBracket,
	GLSLFrontTokenValueTag_LeftAngle,
	GLSLFrontTokenValueTag_RightAngle,
	GLSLFrontTokenValueTag_Comma,
	GLSLFrontTokenValueTag_Semicolon,
	GLSLFrontTokenValueTag_Colon,
	GLSLFrontTokenValueTag_Dot,
	GLSLFrontTokenValueTag_Bang,
	GLSLFrontTokenValueTag_Dash,
	GLSLFrontTokenValueTag_Tilde,
	GLSLFrontTokenValueTag_Plus,
	GLSLFrontTokenValueTag_Star,
	GLSLFrontTokenValueTag_Slash,
	GLSLFrontTokenValueTag_Percent,
	GLSLFrontTokenValueTag_VerticalBar,
	GLSLFrontTokenValueTag_Caret,
	GLSLFrontTokenValueTag_Ampersand,
	GLSLFrontTokenValueTag_Question,
} GLSLFrontTokenValueTag;

typedef struct GLSLFrontTokenValue {
	GLSLFrontTokenValueTag tag;
	union {
		char *identifier;
		GLSLFrontFloat float_constant;
		GLSLFrontInteger int_constant;
		bool bool_constant;
		StorageAccess memory_qualifier;
		Interpolation interpolation;
		Sampling sampling;
		GLSLFrontPrecision precision_qualifier;
		Type type_name;
	} data;
} GLSLFrontTokenValue;

typedef enum GLSLFrontExpectedTokenTag {
	GLSLFrontExpectedTokenTag_Token,
	GLSLFrontExpectedTokenTag_TypeName,
	GLSLFrontExpectedTokenTag_Identifier,
	GLSLFrontExpectedTokenTag_IntLiteral,
	GLSLFrontExpectedTokenTag_FloatLiteral,
	GLSLFrontExpectedTokenTag_BoolLiteral,
	GLSLFrontExpectedTokenTag_Eof,
} GLSLFrontExpectedTokenTag;

typedef struct GLSLFrontExpectedToken {
	GLSLFrontExpectedTokenTag tag;
	union {
		GLSLFrontTokenValue token;
	} data;
} GLSLFrontExpectedToken;

typedef enum GLSLFrontPreprocessorError {
	GLSLFrontPreprocessorError_IntegerOverflow,
	GLSLFrontPreprocessorError_FloatParsingError,
	GLSLFrontPreprocessorError_UnexpectedCharacter,
	GLSLFrontPreprocessorError_UnexpectedToken,
	GLSLFrontPreprocessorError_UnexpectedHash,
	GLSLFrontPreprocessorError_UnexpectedNewLine,
	GLSLFrontPreprocessorError_UnexpectedEndOfInput,
	GLSLFrontPreprocessorError_TooFewDefineArguments,
	GLSLFrontPreprocessorError_TooManyDefineArguments,
	GLSLFrontPreprocessorError_ErrorDirective,
	GLSLFrontPreprocessorError_DuplicateParameter,
	GLSLFrontPreprocessorError_UnknownDirective,
	GLSLFrontPreprocessorError_DefineRedefined,
	GLSLFrontPreprocessorError_ElifOutsideOfBlock,
	GLSLFrontPreprocessorError_ElseOutsideOfBlock,
	GLSLFrontPreprocessorError_EndifOutsideOfBlock,
	GLSLFrontPreprocessorError_ElifAfterElse,
	GLSLFrontPreprocessorError_MoreThanOneElse,
	GLSLFrontPreprocessorError_UnfinishedBlock,
	GLSLFrontPreprocessorError_LineOverflow,
	GLSLFrontPreprocessorError_NotSupported16BitLiteral,
	GLSLFrontPreprocessorError_NotSupported64BitLiteral,
	GLSLFrontPreprocessorError_MacroNotDefined,
	GLSLFrontPreprocessorError_RecursionLimitReached,
	GLSLFrontPreprocessorError_DivisionByZero,
	GLSLFrontPreprocessorError_RemainderByZero,
} GLSLFrontPreprocessorError;

typedef enum GLSLFrontErrorKindTag {
	GLSLFrontErrorKindTag_EndOfFile,
	GLSLFrontErrorKindTag_InvalidProfile,
	GLSLFrontErrorKindTag_InvalidVersion,
	GLSLFrontErrorKindTag_InvalidToken,
	GLSLFrontErrorKindTag_NotImplemented,
	GLSLFrontErrorKindTag_UnknownVariable,
	GLSLFrontErrorKindTag_UnknownType,
	GLSLFrontErrorKindTag_UnknownField,
	GLSLFrontErrorKindTag_UnknownLayoutQualifier,
	GLSLFrontErrorKindTag_UnsupportedMatrixWithTwoRowsInStd140,
	GLSLFrontErrorKindTag_UnsupportedF16MatrixInStd140,
	GLSLFrontErrorKindTag_VariableAlreadyDeclared,
	GLSLFrontErrorKindTag_SemanticError,
	GLSLFrontErrorKindTag_PreprocessorError,
	GLSLFrontErrorKindTag_InternalError,
} GLSLFrontErrorKindTag;

typedef struct GLSLFrontErrorKind {
	GLSLFrontErrorKindTag tag;
	union {
		char *invalid_profile;
		uint64_t invalid_version;
		struct {
			GLSLFrontTokenValue token;
			GLSLFrontExpectedToken *expected;
			size_t expected_len;
		} invalid_token;
		char *not_implemented;
		char *unknown_variable;
		char *unknown_type;
		char *unknown_field;
		char *unknown_layout_qualifier;
		struct {
			uint8_t columns;
		} unsupported_matrix_with_two_rows_in_std140;
		struct {
			uint8_t columns;
			uint8_t rows;
		} unsupported_f16_matrix_in_std140;
		char *variable_already_declared;
		char *semantic_error;
		// Requires questionable `pp_rs` dependency
		struct Empty *NAGA_UNIMPLEMENTED preprocessor_error;
		char *internal_error;
	} data;
} GLSLFrontErrorKind;

typedef struct GLSLFrontParseError {
	GLSLFrontErrorKind kind;
	Span span;
} GLSLFrontParseError;

typedef struct GLSLFrontParseErrors {
	GLSLFrontParseError *errors;
	size_t errors_len;
} GLSLFrontParseErrors;

// --- naga::front::spv ---

typedef struct SPVFrontOptions {
	bool adjust_coordinate_space;
	bool strict_capabilities;
	char *NAGA_NULLABLE block_ctx_dump_prefix;
} SPVFrontOptions;

typedef enum SPVFrontModuleState {
	SPVFrontModuleState_Empty,
	SPVFrontModuleState_Capability,
	SPVFrontModuleState_Extension,
	SPVFrontModuleState_ExtInstImport,
	SPVFrontModuleState_MemoryModel,
	SPVFrontModuleState_EntryPoint,
	SPVFrontModuleState_ExecutionMode,
	SPVFrontModuleState_Source,
	SPVFrontModuleState_Name,
	SPVFrontModuleState_ModuleProcessed,
	SPVFrontModuleState_Annotation,
	SPVFrontModuleState_Type,
	SPVFrontModuleState_Function,
} SPVFrontModuleState;

// private fields
//
// typedef struct SPVFrontInstruction {
// 	uint16_t op;
// 	uint16_t wc;
// } SPVFrontInstruction;

typedef enum SPVFrontErrorTag {
	SPVFrontErrorTag_InvalidHeader,
	SPVFrontErrorTag_InvalidWordCount,
	SPVFrontErrorTag_UnknownInstruction,
	SPVFrontErrorTag_UnknownCapability,
	SPVFrontErrorTag_UnsupportedInstruction,
	SPVFrontErrorTag_UnsupportedCapability,
	SPVFrontErrorTag_UnsupportedExtension,
	SPVFrontErrorTag_UnsupportedExtSet,
	SPVFrontErrorTag_UnsupportedExtInstSet,
	SPVFrontErrorTag_UnsupportedExtInst,
	SPVFrontErrorTag_UnsupportedType,
	SPVFrontErrorTag_UnsupportedExecutionModel,
	SPVFrontErrorTag_UnsupportedExecutionMode,
	SPVFrontErrorTag_UnsupportedStorageClass,
	SPVFrontErrorTag_UnsupportedImageDim,
	SPVFrontErrorTag_UnsupportedImageFormat,
	SPVFrontErrorTag_UnsupportedBuiltIn,
	SPVFrontErrorTag_UnsupportedControlFlow,
	SPVFrontErrorTag_UnsupportedBinaryOperator,
	SPVFrontErrorTag_UnsupportedRuntimeArrayStorageClass,
	SPVFrontErrorTag_UnsupportedMatrixStride,
	SPVFrontErrorTag_UnknownBinaryOperator,
	SPVFrontErrorTag_UnknownRelationalFunction,
	SPVFrontErrorTag_UnsupportedGroupOperation,
	SPVFrontErrorTag_InvalidParameter,
	SPVFrontErrorTag_InvalidOperandCount,
	SPVFrontErrorTag_InvalidOperand,
	SPVFrontErrorTag_InvalidId,
	SPVFrontErrorTag_InvalidDecoration,
	SPVFrontErrorTag_InvalidTypeWidth,
	SPVFrontErrorTag_InvalidSign,
	SPVFrontErrorTag_InvalidInnerType,
	SPVFrontErrorTag_InvalidVectorSize,
	SPVFrontErrorTag_InvalidAccessType,
	SPVFrontErrorTag_InvalidAccess,
	SPVFrontErrorTag_InvalidAccessIndex,
	SPVFrontErrorTag_InvalidIndexType,
	SPVFrontErrorTag_InvalidBinding,
	SPVFrontErrorTag_InvalidGlobalVar,
	SPVFrontErrorTag_InvalidImageExpression,
	SPVFrontErrorTag_InvalidImageWriteType,
	SPVFrontErrorTag_InvalidImageBaseType,
	SPVFrontErrorTag_InvalidImage,
	SPVFrontErrorTag_InvalidAsType,
	SPVFrontErrorTag_InvalidVectorType,
	SPVFrontErrorTag_InconsistentComparisonSampling,
	SPVFrontErrorTag_WrongFunctionResultType,
	SPVFrontErrorTag_WrongFunctionArgumentType,
	SPVFrontErrorTag_MissingDecoration,
	SPVFrontErrorTag_BadString,
	SPVFrontErrorTag_IncompleteData,
	SPVFrontErrorTag_InvalidTerminator,
	SPVFrontErrorTag_InvalidEdgeClassification,
	SPVFrontErrorTag_ControlFlowGraphCycle,
	SPVFrontErrorTag_FunctionCallCycle,
	SPVFrontErrorTag_InvalidArraySize,
	SPVFrontErrorTag_InvalidBarrierScope,
	SPVFrontErrorTag_InvalidBarrierMemorySemantics,
	SPVFrontErrorTag_NonBindingArrayOfImageOrSamplers,
	SPVFrontErrorTag_SpecIdTooHigh,
	SPVFrontErrorTag_AtomicUpgradeError,
} SPVFrontErrorTag;

typedef struct SPVFrontError {
	SPVFrontErrorTag tag;
	union {
		uint16_t unknown_instruction;
		uint32_t unknown_capability;
		struct {
			SPVFrontModuleState module_state;
			uint16_t op;
		} unsupported_instruction;
		uint32_t unsupported_capability;
		char *unsupported_extension;
		char *unsupported_ext_set;
		uint32_t unsupported_ext_inst_set;
		uint32_t unsupported_ext_inst;
		DEFINE_HANDLE_INDEX(Type)
		unsupported_type;
		uint32_t unsupported_execution_model;
		uint32_t unsupported_execution_mode;
		uint32_t unsupported_storage_class;
		uint32_t unsupported_image_dim;
		uint32_t unsupported_image_format;
		uint32_t unsupported_built_in;
		uint32_t unsupported_control_flow;
		uint32_t unsupported_binary_operator;
		struct {
			uint32_t stride;
			uint8_t columns;
			uint8_t rows;
			uint8_t width;
		} unsupported_matrix_stride;
		uint16_t unknown_binary_operator;
		uint16_t unknown_relational_function;
		uint32_t unsupported_group_operation;
		uint16_t invalid_parameter;
		struct {
			uint16_t op;
			uint16_t count;
		} invalid_operand_count;
		uint32_t invalid_id;
		uint32_t invalid_decoration;
		uint32_t invalid_type_width;
		uint32_t invalid_sign;
		uint32_t invalid_inner_type;
		uint32_t invalid_vector_size;
		uint32_t invalid_access_type;
		struct Empty *NAGA_UNIMPLEMENTED invalid_access;
		uint32_t invalid_access_index;
		uint32_t invalid_index_type;
		uint32_t invalid_binding;
		struct Empty *NAGA_UNIMPLEMENTED invalid_global_var;
		struct Empty *NAGA_UNIMPLEMENTED invalid_image_expression;
		DEFINE_HANDLE_INDEX(Type)
		invalid_image_base_type;
		DEFINE_HANDLE_INDEX(Type)
		invalid_image;
		DEFINE_HANDLE_INDEX(Type)
		invalid_as_type;
		DEFINE_HANDLE_INDEX(Type)
		invalid_vector_type;
		DEFINE_HANDLE_INDEX(GlobalVariable)
		inconsistent_comparison_sampling;
		uint32_t wrong_function_result_type;
		uint32_t wrong_function_argument_type;
		uint32_t missing_decoration;
		uint32_t control_flow_graph_cycle;
		uint32_t function_call_cycle;
		uint32_t invalid_array_size;
		uint32_t invalid_barrier_scope;
		uint32_t invalid_barrier_memory_semantics;
		uint32_t spec_id_too_high;
		AtomicUpgradeFrontError atomic_upgrade_error;
	} data;
} SPVFrontError;

// --- naga::front::wgsl ---

typedef struct WGSLFrontOptions {
	bool parse_doc_comments;
} WGSLFrontOptions;

typedef struct WGSLFrontParseErrorLabel {
	Span span;
	char *message;
} WGSLFrontParseErrorLabel;

typedef struct WGSLFrontParseError {
	char *message;
	WGSLFrontParseErrorLabel *labels;
	size_t labels_len;
} WGSLFrontParseError;

// --- naga::compact

typedef enum KeepUnused {
	KeepUnused_No,
	KeepUnused_Yes,
} KeepUnused;

// --- Wrapper Methods ---

typedef struct GLSLFrontendResult {
	Module module;
	GLSLFrontParseErrors errors;
} GLSLFrontendResult;

typedef struct SPVFrontendResult {
	Module module;
	SPVFrontError error;
} SPVFrontendResult;

typedef struct WGSLFrontendResult {
	Module module;
	WGSLFrontParseError error;
} WGSLFrontendResult;

#ifndef NAGA_FFI_NO_METHODS

GLSLFrontendResult naga_front_glsl_parse(
		GLSLFrontOptions options,
		const char *source);
SPVFrontendResult naga_front_spv_parse(
		SPVFrontOptions options,
		uint32_t *const source,
		uint32_t source_length);
WGSLFrontendResult naga_front_wgsl_parse(
		WGSLFrontOptions options,
		const char *source);

#endif

typedef struct ValidateResult {
	ModuleInfo module_info;
	struct Empty *NAGA_UNIMPLEMENTED error;
} ValidateResult;

#ifndef NAGA_FFI_NO_METHODS

Validator naga_valid_validator_new(ValidationFlags flags, Capabilities capabilities);
void naga_valid_validator_reset(Validator *validator);
ValidateResult naga_valid_validator_validate(Validator *validator, Module *const module);
ValidateResult naga_valid_validator_validate_resolved_overrides(Validator *validator, Module *const module);

#endif

#ifndef NAGA_FFI_NO_METHODS

void naga_compact_compact(Module *module, KeepUnused keep_unused);

#endif

typedef struct DOTWriteResult {
	char *output;
	char *error;
} DOTWriteResult;

typedef struct GLSLWriteResult {
	GLSLBackReflectionInfo reflection_info;
	char *output;
	GLSLBackError error;
} GLSLWriteResult;

typedef struct HLSLWriteResult {
	HLSLBackReflectionInfo reflection_info;
	char *output;
	HLSLBackError error;
} HLSLWriteResult;

typedef struct MSLWriteResult {
	MSLBackTranslationInfo translation_info;
	char *output;
	MSLBackError error;
} MSLWriteResult;

typedef struct SPVWriteResult {
	uint32_t *output;
	uint32_t output_count;
	SPVBackError error;
} SPVWriteResult;

typedef struct WGSLWriteResult {
	uint32_t *output;
	uint32_t output_count;
	SPVBackError error;
} WGSLWriteResult;

typedef struct ProcessOverridesResult {
	Module *module;
	ModuleInfo *module_info;
	PipelineConstantError error;
} ProcessOverridesResult;

#ifndef NAGA_FFI_NO_METHODS

DOTWriteResult naga_back_dot_write(
		Module *const module,
		ModuleInfo *const module_info,
		DOTBackOptions options);
GLSLWriteResult naga_back_glsl_write(
		Module *const module,
		ModuleInfo *const module_info,
		GLSLBackOptions options,
		GLSLBackPipelineOptions pipeline_options,
		BoundsCheckPolicies policies);
HLSLWriteResult naga_back_hlsl_write(
		Module *const module,
		ModuleInfo *const module_info,
		HLSLBackOptions options,
		HLSLBackPipelineOptions pipeline_options,
		HLSLBackFragmentEntryPoint *NAGA_NULLABLE fragment_entry_point);
MSLWriteResult naga_back_msl_write(
		Module *const module,
		ModuleInfo *const module_info,
		MSLBackOptions options,
		MSLBackPipelineOptions pipeline_options);
SPVWriteResult naga_back_spv_write(
		Module *const module,
		ModuleInfo *const module_info,
		SPVBackOptions options,
		SPVBackPipelineOptions pipeline_options);
WGSLWriteResult naga_back_wgsl_write(
		Module *const module,
		ModuleInfo *const module_info,
		WGSLBackWriterFlags writer_flags);
ProcessOverridesResult naga_back_process_overrides(
		Module *const module,
		ModuleFillFlags module_flags,
		ModuleInfo *const module_info,
		ModuleInfoFillFlags module_fill_flags,
		ShaderStage NAGA_NULLABLE entry_point_stage,
		const char *NAGA_NULLABLE entry_point_name,
		PipelineConstant *const constants,
		uint32_t constants_count);

#endif

#endif
