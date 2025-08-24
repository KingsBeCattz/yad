export function jsStringToCString(str: string): Uint8Array {
	const encoder = new TextEncoder();
	return encoder.encode(`${str}\0`);
}