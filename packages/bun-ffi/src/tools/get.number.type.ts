// export enum NumberType {
// 	Uint8,
// 	Int8,
// 	Uint16,
// 	Int16,
// 	Uint32,
// 	Int32,
// 	Uint64,
// 	Int64,
// 	Mini,
// 	Half,
// 	Float,
// 	Double,
// }
//
// export function getNumberType(num: number | bigint): NumberType {
// 	const negative = num < 0;
// 	let float = typeof num !== "bigint";
//
// 	if (!float) {
//
// 	}
//
// 	if (!negative) {
// 		if (num < 2 ** 8 - 1) return NumberType.Uint8
// 	}
//
// 	if (!negative && num < 255) {
// 		return NumberType.Uint8
// 	}
// }