// import {FFIType, ptr} from "bun:ffi";
// import {jsStringToCString} from "@tools/js.string.to.c.string";
//
// import dlib from "./dlopen"
//
// console.log(dlib)
//
//
// //
// // const yad_pointer = yad_from_buffer(ptr(yad_bin_buff), yad_bin_buff.byteLength);
// //
// // const rawPtr = ptr(jsStringToCString("johan"));
// //
// // console.log(yad_get_row(yad_pointer, rawPtr))

// import {YadFile} from "@entities/yad";
//
// const yad_file = new YadFile(await Bun.file("./my_first_yad.yad").arrayBuffer());
//
// console.log(yad_file.rowNames());

import {Value} from "@entities/value";

const value = Value.from_array([Value.from_uint8(1)]);

console.log(value.type());
console.log(value.buffer());

value.free()

