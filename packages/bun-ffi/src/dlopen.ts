import {dlopen as open_dl, FFIType} from "bun:ffi";

const YadPtr = FFIType.ptr;

const RowPtr = FFIType.ptr;

const ValuePtr = FFIType.ptr;
const ValueBytesPtr = FFIType.ptr;
const ValueAsTargetPtr = FFIType.ptr;

const PtrOfPtrCChar = FFIType.ptr;
const PtrOfPtrValue = FFIType.ptr;
const PtrCChar = FFIType.ptr;
const PtrCArray = FFIType.ptr;

const path = "../../target/release/yad_core.dll";

export const { symbols, close } = open_dl(
	path,
	{
		yad_new: {
			args: [],
			returns: YadPtr
		},
		yad_from_buffer: {
			args: [FFIType.ptr, FFIType.u64_fast],
			returns: YadPtr,
		},
		yad_version: {
			args: [YadPtr],
			returns: FFIType.ptr,
		},
		yad_free: {
			args: [YadPtr],
			returns: FFIType.void
		},
		yad_rows_len: {
			args: [YadPtr],
			returns: FFIType.u64_fast
		},
		yad_get_row: {
			args: [YadPtr, FFIType.cstring],
			returns: RowPtr
		},
		yad_rows_names: {
			args: [YadPtr],
			returns: PtrOfPtrCChar
		},
		yad_rows_names_free: {
			args: [PtrOfPtrCChar, FFIType.u64_fast],
			returns: FFIType.void
		},
		
		// Value functions
		value_free: {
			args: [ValuePtr],
			returns: FFIType.void
		},
		value_from_buffer: {
			args: [ValueBytesPtr, FFIType.u64_fast],
			returns: ValuePtr
		},
		value_type: {
			args: [ValuePtr],
			returns: FFIType.u8
		},
		value_len: {
			args: [ValuePtr],
			returns: FFIType.u8
		},
		value_raw_bytes: {
			args: [ValuePtr],
			returns: ValueBytesPtr
		},
		value_raw_bytes_length: {
			args: [ValuePtr],
			returns: FFIType.u64_fast
		},
		value_from_uint_8: {
			args: [FFIType.u8],
			returns: ValuePtr
		},
		value_from_int_8: {
			args: [FFIType.i8],
			returns: ValuePtr
		},
		value_as_f8_from_float: {
			args: [FFIType.f32],
			returns: ValuePtr
		},
		uint8_from_value: {
			args: [ValuePtr, ValueAsTargetPtr],
			returns: FFIType.bool
		},
		int8_from_value: {
			args: [ValuePtr, ValueAsTargetPtr],
			returns: FFIType.bool
		},
		float_from_f8_value: {
			args: [ValuePtr, ValueAsTargetPtr],
			returns: FFIType.bool
		},
		value_from_uint_16: {
			args: [FFIType.u16],
			returns: ValuePtr
		},
		value_from_int_16: {
			args: [FFIType.i16],
			returns: ValuePtr
		},
		value_as_f16_from_float: {
			args: [FFIType.f32],
			returns: ValuePtr
		},
		uint16_from_value: {
			args: [ValuePtr, ValueAsTargetPtr],
			returns: FFIType.bool
		},
		int16_from_value: {
			args: [ValuePtr, ValueAsTargetPtr],
			returns: FFIType.bool
		},
		float_from_f16_value: {
			args: [ValuePtr, ValueAsTargetPtr],
			returns: FFIType.bool
		},
		value_from_uint_32: {
			args: [FFIType.u32],
			returns: ValuePtr
		},
		value_from_int_32: {
			args: [FFIType.i32],
			returns: ValuePtr
		},
		value_from_float: {
			args: [FFIType.f32],
			returns: ValuePtr
		},
		uint32_from_value: {
			args: [ValuePtr, ValueAsTargetPtr],
			returns: FFIType.bool
		},
		int32_from_value: {
			args: [ValuePtr, ValueAsTargetPtr],
			returns: FFIType.bool
		},
		float_from_value: {
			args: [ValuePtr, ValueAsTargetPtr],
			returns: FFIType.bool
		},
		value_from_uint_64: {
			args: [FFIType.u64],
			returns: ValuePtr
		},
		value_from_int_64: {
			args: [FFIType.i64],
			returns: ValuePtr
		},
		value_from_double: {
			args: [FFIType.f64],
			returns: ValuePtr
		},
		uint64_from_value: {
			args: [ValuePtr, ValueAsTargetPtr],
			returns: FFIType.bool
		},
		int64_from_value: {
			args: [ValuePtr, ValueAsTargetPtr],
			returns: FFIType.bool
		},
		double_from_value: {
			args: [ValuePtr, ValueAsTargetPtr],
			returns: FFIType.bool
		},
		value_from_bool: {
			args: [FFIType.bool],
			returns: ValuePtr
		},
		bool_from_value: {
			args: [ValuePtr, ValueAsTargetPtr],
			returns: FFIType.bool
		},
		value_from_cstring: {
			args: [FFIType.cstring],
			returns: ValuePtr
		},
		cstring_from_value: {
			args: [ValuePtr],
			returns: PtrCChar
		},
		cstring_free: {
			args: [PtrCChar],
			returns: FFIType.void
		},
		cstring_len_from_value: {
			args: [ValuePtr],
			returns: FFIType.u64_fast
		},
		value_from_c_array: {
			args: [PtrCArray],
			returns: PtrCArray
		},
		c_array_new: {
			args: [],
			returns: PtrCArray
		},
		c_array_push: {
			args: [PtrCArray, ValuePtr],
			returns: FFIType.bool
		},
		c_array_from_value: {
			args: [ValuePtr],
			returns: PtrCArray
		},
		c_array_as_ptr: {
			args: [PtrCArray, FFIType.u64_fast],
			returns: PtrOfPtrValue
		}
	},
);

/* Author: Johan | Date: 8/24/2025 12:09 AM
Check value functions
Fix Value.as_array, error:Symbol "c_array_from_value" not found in "../../target/release/yad_core.dll"
AND EVERYTHING :sob_LOUDLY:
*/