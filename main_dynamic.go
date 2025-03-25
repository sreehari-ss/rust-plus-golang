package main

// NOTE: There should be NO space between the comments and the `import "C"` line.

/*
#cgo LDFLAGS: -L./lib -lhello
#include "./lib/hello.h"
#include <stdlib.h>
*/
import "C"
import "unsafe"

func main() {
	template := C.CString("Good afternoon, {{name}}, isDefined {{ isdefined age}}")
	data := C.CString("{\"name\":\"harweri\"}")
	defer C.free(unsafe.Pointer(template))
	defer C.free(unsafe.Pointer(data))
	rString := C.render_template(template, data)
	// we need to free it this way because of this https://doc.rust-lang.org/std/ffi/struct.CString.html#method.into_raw
	defer C.free_rust_string(rString)
	_r := C.GoString(rString)
	print(_r)
}
