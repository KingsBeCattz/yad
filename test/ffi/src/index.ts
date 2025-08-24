import { dlopen, FFIType, suffix } from "bun:ffi";

const path = `/artifacts/yad_core.${suffix}`;

const {
	symbols: {
		yad_from_buffer,
	},
} = dlopen(
	path,
	{
		yad_from_buffer: {
			args: [FFIType.buffer, FFIType.u32],
			returns: FFIType.ptr,
		},
	},
);

const yad_bin = Bun.file("./my_first_yad.yad");

console.log(await yad_bin.arrayBuffer())