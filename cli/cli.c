#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "../naga.h"

// NOTE: This exists less as an example and more for testing.

// TODO: Expand support outside of SPIR-V => WGSL
int main(int argc, char **argv) {
	if (argc != 3) {
		printf("naga-native cli <input.spv> <output.wgsl>\n");
		exit(1);
	}

	char *input_path = argv[1];
	char *output_path = argv[2];

	FILE *input_file = fopen(input_path, "rb");
	FILE *output_file = fopen(output_path, "w");

	fseek(input_file, 0, SEEK_END);
	long input_size = ftell(input_file);

	uint8_t *buffer = (uint8_t *)malloc(input_size);
	rewind(input_file);
	fread(buffer, 1, input_size, input_file);

	// ---

	uint8_t success;

	// --- front

	CapabilitiesFlags caps =
			Capabilities_MULTISAMPLED_SHADING |
			Capabilities_CUBE_ARRAY_TEXTURES |
			Capabilities_IMMEDIATES |
			Capabilities_STORAGE_TEXTURE_16BIT_NORM_FORMATS |
			Capabilities_SHADER_FLOAT16_IN_FLOAT32 |
			Capabilities_TEXTURE_AND_SAMPLER_BINDING_ARRAY |
			Capabilities_TEXTURE_AND_SAMPLER_BINDING_ARRAY_NON_UNIFORM_INDEXING |
			Capabilities_STORAGE_TEXTURE_BINDING_ARRAY |
			Capabilities_STORAGE_TEXTURE_BINDING_ARRAY_NON_UNIFORM_INDEXING |
			Capabilities_SUBGROUP;
	ModuleFillFlags fill_flags = NAGA_FLAGS_ALL(ModuleFillFlags);
	SPVFrontOptions options = (SPVFrontOptions){
		.adjust_coordinate_space = 1,
		.strict_capabilities = 1,
		.block_ctx_dump_prefix = NULL,
	};
	SPVFrontResult front_result;
	front_result.flags = FrontResultOption_FormattedErrorOnly;
	success = naga_front_spv_parse(options, (uint32_t *)buffer, input_size / 4, fill_flags, &front_result);
	if (!success) {
		printf("Front error: %s\n", front_result.fmt_error);
		exit(1);
	}

	// --- valid

	Validator validator = naga_valid_validator_new(NAGA_FLAGS_ALL(ValidationFlagsFlags), caps);
	ValidateResult valid_result;
	valid_result.flags = ValidateResultOption_FormattedErrorOnly;
	success = naga_valid_validator_validate(&validator, &front_result.module, &valid_result);
	if (!success) {
		printf("Valid error: %s\n", valid_result.fmt_error);
		exit(1);
	}

	// --- back

	WGSLBackWriterFlagsFlags writer_flags = NAGA_FLAGS_EMPTY(WGSLBackWriterFlagsFlags);
	WGSLWriteResult back_result;
	back_result.flags = WriteResultOption_FormattedErrorOnly;
	success = naga_back_wgsl_write(&front_result.module, &valid_result.module_info, writer_flags, &back_result);
	if (!success) {
		printf("Back error: %s\n", back_result.fmt_error);
		exit(1);
	}

	// ---

	fwrite(back_result.output, 1, strlen(back_result.output), output_file);

	fclose(input_file);
	fclose(output_file);
	return 0;
}
