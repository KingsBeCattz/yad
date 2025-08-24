import { ptr, CString, type Pointer, read } from "bun:ffi"

import { symbols } from "@/dlopen";
const { yad_new, yad_from_buffer, yad_free, yad_rows_len, yad_rows_names, yad_rows_names_free } = symbols;

export class YadFile {
	#ptr: Pointer | null;
	
	constructor(buffer?: ArrayBuffer) {
		if (buffer) {
			this.#ptr = yad_from_buffer(ptr(buffer), buffer.byteLength)
		} else {
			this.#ptr = yad_new();
		}
	}
	
	public get ptr() {
		return this.#ptr
	}
	
	public free() {
		yad_free(this.ptr)
		this.#ptr = null
	}
	
	public rowCount() {
		return yad_rows_len(this.#ptr)
	}
	
	public rowNames(): string[] {
		const ptrArray = yad_rows_names(this.#ptr);
		const rows: string[] = [];
		
		if (!ptrArray) {
			yad_rows_names_free(ptrArray);
			return []
		}
		
		const count = this.rowCount();
		
		for (let i = 0; i < count; i++) {
			const offset = i * 8;
			
			const rawPtr = Number(read.u64(ptrArray, offset));
			
			if (!rawPtr) {
				continue;
			}
			
			const jsStr = new CString(rawPtr as Pointer).toString();
			
			rows.push(jsStr);
		}
		
		yad_rows_names_free(ptrArray, count);
		return rows;
	}
}