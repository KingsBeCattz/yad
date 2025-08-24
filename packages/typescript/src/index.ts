import { dlopen, FFIType } from "bun:ffi";

const path = `./artifacts/yad_core.dll`;

dlopen(
	path,
	{
		yad_from_buffer: {
			args: [FFIType.ptr, FFIType.ptr],
			returns: FFIType.ptr,
		},
	},
);
