export enum GenericErrorMessages {
	FAILED_PUSHING_ON_C_ARRAY = "Failed to push value into CArray"
}

export enum ReferenceErrorMessages {
	INVALID_BUFFER_FOR_VALUE = "The given buffer is not a valid value.",
	INVALID_STRING_FOR_VALUE = "The given string is not valid to build a value.",
	INVALID_VALUE_FOR_STRING = "The given value is not valid to build a string.",
	FAILED_ALLOCATE_ARRAY = "Failed to allocate CArray",
	FAILED_CREATING_VALUE_FROM_ARRAY = "Failed creating a value from CArray"
}

export enum TypeErrorMessages {
	NUMBER_IS_NOT_A_UINT8 = "The given number does not fit into the uint8 type.",
	VALUE_IS_NOT_A_UINT8 = "The given value does not fit into the uint8 type.",
	NUMBER_IS_NOT_A_INT8 = "The given number does not fit into the int8 type.",
	VALUE_IS_NOT_A_INT8 = "The given value does not fit into the int8 type.",
	NUMBER_IS_NOT_A_MINI_FLOAT = "The given number does not fit into the mini float type.",
	VALUE_IS_NOT_A_MINI_FLOAT = "The given value does not fit into the mini float type.",
	NUMBER_IS_NOT_A_UINT16 = "The given number does not fit into the uint16 type.",
	VALUE_IS_NOT_A_UINT16 = "The given value does not fit into the uint16 type.",
	NUMBER_IS_NOT_A_INT16 = "The given number does not fit into the int16 type.",
	VALUE_IS_NOT_A_INT16 = "The given value does not fit into the int16 type.",
	NUMBER_IS_NOT_A_HALF_FLOAT = "The given number does not fit into the half float type.",
	VALUE_IS_NOT_A_HALF_FLOAT = "The given value does not fit into the half float type.",
	NUMBER_IS_NOT_A_UINT32 = "The given number does not fit into the uint32 type.",
	VALUE_IS_NOT_A_UINT32 = "The given value does not fit into the uint32 type.",
	NUMBER_IS_NOT_A_INT32 = "The given number does not fit into the int32 type.",
	VALUE_IS_NOT_A_INT32 = "The given value does not fit into the int32 type.",
	NUMBER_IS_NOT_A_FLOAT = "The given number does not fit into the float type.",
	VALUE_IS_NOT_A_FLOAT = "The given value does not fit into the float type.",
	NUMBER_IS_NOT_A_UINT64 = "The given number does not fit into the uint64 type.",
	VALUE_IS_NOT_A_UINT64 = "The given value does not fit into the uint64 type.",
	NUMBER_IS_NOT_A_INT64 = "The given number does not fit into the int64 type.",
	VALUE_IS_NOT_A_INT64 = "The given value does not fit into the int64 type.",
	NUMBER_IS_NOT_A_DOUBLE = "The given number does not fit into the double float type.",
	VALUE_IS_NOT_A_DOUBLE = "The given value does not fit into the double float type.",
	VALUE_IS_NOT_A_BOOL = "The given value is not a boolean.",
	VALUE_IS_NOT_A_ARRAY = "The given value is not a array",
	CANNOT_PUSH_A_NULL_PTR = "You cannot push a value with null pointer."
}