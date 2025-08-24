import {type Pointer, ptr, toArrayBuffer, CString} from "bun:ffi";

import { symbols } from "@/dlopen";
import {
	isNumberDouble,
	isNumberFloat,
	isNumberHalfFloat,
	isNumberInt16,
	isNumberInt32,
	isNumberInt64,
	isNumberInt8,
	isNumberMiniFloat,
	isNumberUint16,
	isNumberUint32, isNumberUint64,
	isNumberUint8
} from "@tools/number.guards";
import {GenericErrorMessages, ReferenceErrorMessages, TypeErrorMessages} from "@/errors";
import {jsStringToCString} from "@tools/js.string.to.c.string";
const {
	value_from_buffer, value_free, value_from_uint_8, uint8_from_value, value_from_int_8, int8_from_value,
	value_raw_bytes, value_raw_bytes_length, value_as_f8_from_float, float_from_f8_value, value_type, value_len,
	value_from_uint_16, uint16_from_value, value_from_int_16, int16_from_value, value_as_f16_from_float,
	float_from_f16_value, value_from_uint_32, uint32_from_value, value_from_int_32, int32_from_value,
	value_from_float, float_from_value, value_from_uint_64, uint64_from_value, value_from_int_64, int64_from_value,
	value_from_double, double_from_value, value_from_cstring, cstring_from_value, cstring_free, value_from_bool,
	bool_from_value, c_array_new, c_array_push, value_from_c_array, c_array_from_value, c_array_as_ptr
} = symbols;

export enum ValueType {
	Unknown = 0x00,
	
	Uint8 = 0x11,
	Int8 = 0x21,
	Float8 = 0x31,
	
	Uint16 = 0x12,
	Int16 = 0x22,
	Float16 = 0x32,
	
	Uint32 = 0x13,
	Int32 = 0x23,
	Float32 = 0x33,
	
	Uint64 = 0x14,
	Int64 = 0x24,
	Float64 = 0x34,
	
	String = 0x4F, // Unifier for 0x41 ... 0x44
	Array = 0x5F, // Unifier for 0x41 ... 0x44
	
	False = 0x80,
	True = 0x81,
}

export class Value {
	#ptr: Pointer | null
	constructor(ptr: Pointer) {
		this.#ptr = ptr
	}
	
	static from_buffer(buff: ArrayBuffer): Value {
		const val_ptr = value_from_buffer(ptr(buff), buff.byteLength)
		
		if (!val_ptr) throw new ReferenceError(ReferenceErrorMessages.INVALID_BUFFER_FOR_VALUE)
		
		return new Value(val_ptr)
	}
	
	public get ptr() {
		return this.#ptr
	}
	
	public free() {
		value_free(this.ptr)
		this.#ptr = null
	}
	
	public buffer() {
		const raw_bytes_ptr = value_raw_bytes(this.ptr);
		if (!raw_bytes_ptr) return null;
		const raw_bytes_length_ptr = value_raw_bytes_length(this.ptr);
		return toArrayBuffer(raw_bytes_ptr, 0, Number(raw_bytes_length_ptr))
	}
	
	public type() {
		const type = value_type(this.ptr);
		let length = value_len(this.ptr);
		
		if ((type | 0x0F) === ValueType.String || (type | 0x0F) === ValueType.Array) length = 0x0F
		
		return (ValueType[type | length] ?? "Unknown") as keyof typeof ValueType
	}
	
	static from_uint8(num: number | bigint): Value {
		if (!isNumberUint8(num)) throw new TypeError(TypeErrorMessages.NUMBER_IS_NOT_A_UINT8);
		
		const ptr = value_from_uint_8(Number(num));
		if (!ptr) throw new TypeError(TypeErrorMessages.NUMBER_IS_NOT_A_UINT8);
		return new Value(ptr);
	}
	
	static try_from_uint8(num: number | bigint): Value | null {
		try {
			return Value.from_uint8(num)
		} catch (_e) {
			return null
		}
	}
	
	public as_uint8(): number {
		const buff = new Uint8ClampedArray(1);
		
		if (!uint8_from_value(this.ptr, ptr(buff))) throw new TypeError(TypeErrorMessages.VALUE_IS_NOT_A_UINT8);
		
		return buff[0] ?? 0;
	}
	
	public try_as_uint8(): number | null {
		try {
			return this.as_uint8()
		} catch (_e) {
			return null
		}
	}
	
	static from_int8(num: number | bigint): Value {
		if (!isNumberInt8(num)) throw new TypeError(TypeErrorMessages.NUMBER_IS_NOT_A_INT8);
		
		const ptr = value_from_int_8(Number(num));
		if (!ptr) throw new TypeError(TypeErrorMessages.NUMBER_IS_NOT_A_INT8);
		return new Value(ptr);
	}
	
	static try_from_int8(num: number | bigint): Value | null {
		try {
			return Value.from_int8(num)
		} catch (_e) {
			return null
		}
	}
	
	public as_int8(): number {
		const buff = new Int8Array(1);
		
		if (!int8_from_value(this.ptr, ptr(buff))) throw new TypeError(TypeErrorMessages.VALUE_IS_NOT_A_INT8);
		
		return buff[0] ?? 0;
	}
	
	public try_as_int8(): number | null {
		try {
			return this.as_int8()
		} catch (_e) {
			return null
		}
	}
	
	static from_mini_float(num: number): Value {
		if (!isNumberMiniFloat(num)) throw new TypeError(TypeErrorMessages.NUMBER_IS_NOT_A_MINI_FLOAT);
		
		const ptr = value_as_f8_from_float(Number(num));
		if (!ptr) throw new TypeError(TypeErrorMessages.NUMBER_IS_NOT_A_MINI_FLOAT);
		return new Value(ptr);
	}
	
	static try_from_mini_float(num: number): Value | null {
		try {
			return Value.from_mini_float(num)
		} catch (_e) {
			return null
		}
	}
	
	public as_mini_float(): number {
		const buff = new Float32Array(1);
		
		if (!float_from_f8_value(this.ptr, ptr(buff))) throw new TypeError(TypeErrorMessages.NUMBER_IS_NOT_A_MINI_FLOAT);
		
		return buff[0] ?? 0;
	}
	
	public try_as_mini_float(): number | null {
		try {
			return this.as_mini_float()
		} catch (_e) {
			return null
		}
	}
	
	static from_uint16(num: number | bigint): Value {
		if (!isNumberUint16(num)) throw new TypeError(TypeErrorMessages.NUMBER_IS_NOT_A_UINT16);
		
		const ptr = value_from_uint_16(Number(num));
		if (!ptr) throw new TypeError(TypeErrorMessages.NUMBER_IS_NOT_A_UINT16);
		return new Value(ptr);
	}
	
	static try_from_uint16(num: number | bigint): Value | null {
		try {
			return Value.from_uint16(num)
		} catch (_e) {
			return null
		}
	}
	
	public as_uint16(): number {
		const buff = new Uint16Array(1);
		
		if (!uint16_from_value(this.ptr, ptr(buff))) throw new TypeError(TypeErrorMessages.VALUE_IS_NOT_A_UINT16);
		
		return buff[0] ?? 0;
	}
	
	public try_as_uint16(): number | null {
		try {
			return this.as_uint16()
		} catch (_e) {
			return null
		}
	}
	
	static from_int16(num: number | bigint): Value {
		if (!isNumberInt16(num)) throw new TypeError(TypeErrorMessages.NUMBER_IS_NOT_A_INT16);
		
		const ptr = value_from_int_16(Number(num));
		if (!ptr) throw new TypeError(TypeErrorMessages.NUMBER_IS_NOT_A_INT16);
		return new Value(ptr);
	}
	
	static try_from_int16(num: number | bigint): Value | null {
		try {
			return Value.from_int16(num)
		} catch (_e) {
			return null
		}
	}
	
	public as_int16(): number {
		const buff = new Int16Array(1);
		
		if (!int16_from_value(this.ptr, ptr(buff))) throw new TypeError(TypeErrorMessages.VALUE_IS_NOT_A_INT16);
		
		return buff[0] ?? 0;
	}
	
	public try_as_int16(): number | null {
		try {
			return this.as_int16()
		} catch (_e) {
			return null
		}
	}
	
	static from_half_float(num: number): Value {
		if (!isNumberHalfFloat(num)) throw new TypeError(TypeErrorMessages.NUMBER_IS_NOT_A_HALF_FLOAT);
		
		const ptr = value_as_f16_from_float(Number(num));
		if (!ptr) throw new TypeError(TypeErrorMessages.NUMBER_IS_NOT_A_HALF_FLOAT);
		return new Value(ptr);
	}
	
	static try_from_half_float(num: number): Value | null {
		try {
			return Value.from_half_float(num)
		} catch (_e) {
			return null
		}
	}
	
	public as_half_float(): number {
		const buff = new Float32Array(1);
		
		if (!float_from_f16_value(this.ptr, ptr(buff))) throw new TypeError(TypeErrorMessages.NUMBER_IS_NOT_A_HALF_FLOAT);
		
		return buff[0] ?? 0;
	}
	
	public try_as_half_float(): number | null {
		try {
			return this.as_half_float()
		} catch (_e) {
			return null
		}
	}
	
	static from_uint32(num: number | bigint): Value {
		if (!isNumberUint32(num)) throw new TypeError(TypeErrorMessages.NUMBER_IS_NOT_A_UINT32);
		
		const ptr = value_from_uint_32(Number(num));
		if (!ptr) throw new TypeError(TypeErrorMessages.NUMBER_IS_NOT_A_UINT32);
		return new Value(ptr);
	}
	
	static try_from_uint32(num: number | bigint): Value | null {
		try {
			return Value.from_uint32(num)
		} catch (_e) {
			return null
		}
	}
	
	public as_uint32(): number {
		const buff = new Uint32Array(1);
		
		if (!uint32_from_value(this.ptr, ptr(buff))) throw new TypeError(TypeErrorMessages.VALUE_IS_NOT_A_UINT32);
		
		return buff[0] ?? 0;
	}
	
	public try_as_uint32(): number | null {
		try {
			return this.as_uint32()
		} catch (_e) {
			return null
		}
	}
	
	static from_int32(num: number | bigint): Value {
		if (!isNumberInt32(num)) throw new TypeError(TypeErrorMessages.NUMBER_IS_NOT_A_INT32);
		
		const ptr = value_from_int_32(Number(num));
		if (!ptr) throw new TypeError(TypeErrorMessages.NUMBER_IS_NOT_A_INT32);
		return new Value(ptr);
	}
	
	static try_from_int32(num: number | bigint): Value | null {
		try {
			return Value.from_int32(num)
		} catch (_e) {
			return null
		}
	}
	
	public as_int32(): number {
		const buff = new Int32Array(1);
		
		if (!int32_from_value(this.ptr, ptr(buff))) throw new TypeError(TypeErrorMessages.VALUE_IS_NOT_A_INT32);
		
		return buff[0] ?? 0;
	}
	
	public try_as_int32(): number | null {
		try {
			return this.as_int32()
		} catch (_e) {
			return null
		}
	}
	
	static from_float(num: number): Value {
		if (!isNumberFloat(num)) throw new TypeError(TypeErrorMessages.NUMBER_IS_NOT_A_FLOAT);
		
		const ptr = value_from_float(Number(num));
		if (!ptr) throw new TypeError(TypeErrorMessages.NUMBER_IS_NOT_A_FLOAT);
		return new Value(ptr);
	}
	
	static try_from_float(num: number): Value | null {
		try {
			return Value.from_float(num)
		} catch (_e) {
			return null
		}
	}
	
	public as_float(): number {
		const buff = new Float32Array(1);
		
		if (!float_from_value(this.ptr, ptr(buff))) throw new TypeError(TypeErrorMessages.NUMBER_IS_NOT_A_FLOAT);
		
		return buff[0] ?? 0;
	}
	
	public try_as_float(): number | null {
		try {
			return this.as_float()
		} catch (_e) {
			return null
		}
	}
	
	static from_uint64(num: number | bigint): Value {
		if (!isNumberUint64(num)) throw new TypeError(TypeErrorMessages.NUMBER_IS_NOT_A_UINT64);
		
		const ptr = value_from_uint_64(Number(num));
		if (!ptr) throw new TypeError(TypeErrorMessages.NUMBER_IS_NOT_A_UINT64);
		return new Value(ptr);
	}
	
	static try_from_uint64(num: number | bigint): Value | null {
		try {
			return Value.from_uint64(num)
		} catch (_e) {
			return null
		}
	}
	
	public as_uint64(): bigint {
		const buff = new BigUint64Array(1);
		
		if (!uint64_from_value(this.ptr, ptr(buff))) throw new TypeError(TypeErrorMessages.VALUE_IS_NOT_A_UINT64);
		
		return buff[0] ?? 0n;
	}
	
	public try_as_uint64(): bigint | null {
		try {
			return this.as_uint64()
		} catch (_e) {
			return null
		}
	}
	
	static from_int64(num: number | bigint): Value {
		if (!isNumberInt64(num)) throw new TypeError(TypeErrorMessages.NUMBER_IS_NOT_A_INT64);
		
		const ptr = value_from_int_64(Number(num));
		if (!ptr) throw new TypeError(TypeErrorMessages.NUMBER_IS_NOT_A_INT64);
		return new Value(ptr);
	}
	
	static try_from_int64(num: number | bigint): Value | null {
		try {
			return Value.from_int64(num)
		} catch (_e) {
			return null
		}
	}
	
	public as_int64(): bigint {
		const buff = new BigInt64Array(1);
		
		if (!int64_from_value(this.ptr, ptr(buff))) throw new TypeError(TypeErrorMessages.VALUE_IS_NOT_A_INT64);
		
		return buff[0] ?? 0n;
	}
	
	public try_as_int64(): bigint | null {
		try {
			return this.as_int64()
		} catch (_e) {
			return null
		}
	}
	
	static from_double(num: number): Value {
		if (!isNumberDouble(num)) throw new TypeError(TypeErrorMessages.NUMBER_IS_NOT_A_DOUBLE);
		
		const ptr = value_from_double(Number(num));
		if (!ptr) throw new TypeError(TypeErrorMessages.NUMBER_IS_NOT_A_DOUBLE);
		return new Value(ptr);
	}
	
	static try_from_double(num: number): Value | null {
		try {
			return Value.from_double(num)
		} catch (_e) {
			return null
		}
	}
	
	public as_double(): number {
		const buff = new Float64Array(1);
		
		if (!double_from_value(this.ptr, ptr(buff))) throw new TypeError(TypeErrorMessages.NUMBER_IS_NOT_A_DOUBLE);
		
		return buff[0] ?? 0;
	}
	
	public try_as_double(): number | null {
		try {
			return this.as_double()
		} catch (_e) {
			return null
		}
	}
	
	static from_string(string: string) {
		const cString = jsStringToCString(string);
		const value_ptr = value_from_cstring(ptr(cString));
		if (!value_ptr) throw new ReferenceError(ReferenceErrorMessages.INVALID_STRING_FOR_VALUE)
		return new Value(value_ptr)
	}
	
	public as_string() {
		const raw_c_string = cstring_from_value(this.ptr);
		
		try {
			if(!raw_c_string) throw new ReferenceError(ReferenceErrorMessages.INVALID_VALUE_FOR_STRING);
			
			return new CString(raw_c_string).toString()
		} finally {
			cstring_free(raw_c_string)
		}
	}
	
	public try_as_string() {
		try {
			return this.as_string();
		} catch (_e) {
			return null
		}
	}
	
	static from_boolean(bool: boolean): Value {
		const ptr = value_from_bool(bool);
		if (!ptr) throw new TypeError(TypeErrorMessages.VALUE_IS_NOT_A_BOOL);
		return new Value(ptr);
	}
	
	static try_from_boolean(bool: boolean): Value | null {
		try {
			return Value.from_boolean(bool)
		} catch (_e) {
			return null
		}
	}
	
	public as_bool(): boolean {
		const buff = new Uint8Array(1);
		
		if (!bool_from_value(this.ptr, ptr(buff))) throw new TypeError(TypeErrorMessages.VALUE_IS_NOT_A_BOOL);
		
		console.log(buff)
		
		return buff[0] !== 0;
	}
	
	public try_as_bool(): boolean | null {
		try {
			return this.as_bool()
		} catch (_e) {
			return null
		}
	}
	
	static from_array(arr: Value[]) {
		const c_arr = c_array_new();
		if (!c_arr) throw new ReferenceError(ReferenceErrorMessages.FAILED_ALLOCATE_ARRAY);
		
		for (const value of arr) {
			if (!value.ptr) throw new TypeError(TypeErrorMessages.CANNOT_PUSH_A_NULL_PTR);
			if (!c_array_push(c_arr, value.ptr)) {
				throw new Error(GenericErrorMessages.FAILED_PUSHING_ON_C_ARRAY);
			}
		}
		
		const c_arr_as_value = value_from_c_array(c_arr)
		
		if (!c_arr_as_value) throw new ReferenceError(ReferenceErrorMessages.FAILED_CREATING_VALUE_FROM_ARRAY)
		
		return new Value(c_arr_as_value);
	}
	
	static try_from_array(arr: Value[]) {
		try {
			return Value.from_array(arr)
		} catch (_e) {
			return null
		}
	}
	
	public as_array(): Value[] {
		const c_array_ptr = c_array_from_value(this.ptr)
		
		if (!c_array_ptr) throw new TypeError(TypeErrorMessages.VALUE_IS_NOT_A_ARRAY)
		
		const lenBuf = new BigUint64Array(1);
		const outPtr = ptr(lenBuf);
		const arrayPtr = c_array_as_ptr(c_array_ptr, outPtr);
		
		if (!arrayPtr) throw new Error("Idk lol")
		
		const length = Number(lenBuf[0]);
		
		const values: Value[] = [];
		const rawPointers = new BigUint64Array(arrayPtr);
		
		for (let i = 0; i < length; i++) {
			const valPtr = rawPointers[i];
			if (valPtr !== 0n) {
				values.push(new Value(Number(valPtr) as Pointer));
			}
		}
		
		return values;
	}
}