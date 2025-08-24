/* Author: Johan | Date: 8/23/2025 12:00 AM
EVERYTHING AHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHH
*/

export function isNumberUint8(num: number | bigint) {
	try {
		const int = BigInt(num);
		return int >= 0n && int <= 2n ** 8n - 1n;
	} catch (_e) {
		return false
	}
}

export function isNumberInt8(num: number | bigint) {
	try {
		const int = BigInt(num);
		return int >= -(2n ** 7n) && int <= 2n ** 7n - 1n;
		
	} catch (_e) {
		return false
	}
}

export function isNumberMiniFloat(num: number): boolean {
	if (Number.isNaN(num) || !Number.isFinite(num)) {
		return false;
	}
	
	const abs = Math.abs(num);
	return abs === 0 || (abs >= 2 ** -6 && abs <= 240);
}

export function isNumberUint16(num: number | bigint) {
	try {
		const int = BigInt(num);
		return int >= 0n && int <= 2n ** 16n - 1n;
		
	} catch (_e) {
		return false
	}
}

export function isNumberInt16(num: number | bigint) {
	try {
		const int = BigInt(num);
		return int >= -(2n**15n) && int <= 2n**15n-1n;
		
	} catch (_e) {
		return false
	}
}

export function isNumberHalfFloat(num: number): boolean {
	if (Number.isNaN(num) || !Number.isFinite(num)) {
		return false;
	}
	
	const abs = Math.abs(num);
	return abs === 0 || (abs >= 5.96e-8 && abs <= 65504);
}

export function isNumberUint32(num: number | bigint) {
	try {
		const int = BigInt(num);
		return int >= 0n && int <= 2n ** 32n - 1n;
		
	} catch (_e) {
		return false
	}
}

export function isNumberInt32(num: number | bigint) {
	try {
		const int = BigInt(num);
		return int >= -(2n**31n) && int <= 2n**31n - 1n;
		
	} catch (_e) {
		return false
	}
}

export function isNumberFloat(num: number): boolean {
	if (Number.isNaN(num) || !Number.isFinite(num)) {
		return false;
	}
	
	const abs = Math.abs(num);
	return abs === 0 || (abs >= 1.4e-45 && abs <= 3.4028235e38);
}

export function isNumberUint64(num: number | bigint) {
	try {
		const int = BigInt(num);
		return int >= 0n && int < 2n ** 64n - 1n;
		
	} catch (_e) {
		return false
	}
}

export function isNumberInt64(num: number | bigint) {
	try {
		const int = BigInt(num);
		return int >= -(2n**63n) && int <= 2n**63n-1n;
		
	} catch (_e) {
		return false
	}
}

export function isNumberDouble(num: number): boolean {
	if (Number.isNaN(num) || !Number.isFinite(num)) {
		return false;
	}
	
	const abs = Math.abs(num);
	return abs === 0 || (abs >= 5e-324 && abs <= 1.7976931348623157e308);
}