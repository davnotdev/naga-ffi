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

	NagaCapabilitiesFlags caps =
			NagaCapabilities_MULTISAMPLED_SHADING |
			NagaCapabilities_CUBE_ARRAY_TEXTURES |
			NagaCapabilities_IMMEDIATES |
			NagaCapabilities_STORAGE_TEXTURE_16BIT_NORM_FORMATS |
			NagaCapabilities_SHADER_FLOAT16_IN_FLOAT32 |
			NagaCapabilities_TEXTURE_AND_SAMPLER_BINDING_ARRAY |
			NagaCapabilities_TEXTURE_AND_SAMPLER_BINDING_ARRAY_NON_UNIFORM_INDEXING |
			NagaCapabilities_STORAGE_TEXTURE_BINDING_ARRAY |
			NagaCapabilities_STORAGE_TEXTURE_BINDING_ARRAY_NON_UNIFORM_INDEXING |
			NagaCapabilities_SUBGROUP;
	NagaModuleFillFlags fill_flags = NAGA_FLAGS_ALL(NagaModuleFillFlags);
	NagaSPVFrontOptions options = (NagaSPVFrontOptions){
		.adjust_coordinate_space = 1,
		.strict_capabilities = 1,
		.block_ctx_dump_prefix = NULL,
	};
	NagaSPVFrontResult front_result;
	front_result.flags = NagaFrontResultOption_FormattedErrorOnly;
	success = naga_front_spv_parse(options, (uint32_t *)buffer, input_size / 4, fill_flags, &front_result);
	if (!success) {
		printf("Front error: %s\n", front_result.fmt_error);
		exit(1);
	}

	// --- valid

	NagaValidator validator = naga_valid_validator_new(NAGA_FLAGS_ALL(NagaValidationFlagsFlags), caps);
	NagaValidateResult valid_result;
	valid_result.flags = NagaValidateResultOption_FormattedErrorOnly;
	success = naga_valid_validator_validate(&validator, &front_result.module, &valid_result);
	if (!success) {
		printf("Valid error: %s\n", valid_result.fmt_error);
		exit(1);
	}

	// --- back

	NagaWGSLBackWriterFlagsFlags writer_flags = NAGA_FLAGS_EMPTY(NagaWGSLBackWriterFlagsFlags);
	NagaWGSLWriteResult back_result;
	back_result.flags = NagaWriteResultOption_FormattedErrorOnly;
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
