package main

// NOTE: There should be NO space between the comments and the `import "C"` line.

/*
#cgo LDFLAGS: -L./lib -lhello
#include "./lib/hello.h"
#include <stdlib.h>
*/
import "C"
import (
	"fmt"
	"sync"
	"time"
	"unsafe"
)

func main() {

	// call_rust_render_template_and_print()
	// call_rust_render_template_and_print_error()
	benchmark()
	benchmark_wo_goroutine()

}

func call_rust_render_template_and_print() {
	val := call_rust_render_template("Good afternoon, {{name}}", "{\"name\":\"hari\"}")
	println(val)
}

func call_rust_render_template_and_print_error() {
	val := call_rust_render_template("Good afternoon, {{name}}, {{age}}", "{\"name\":\"hari\"}")
	println(val)
}

func benchmark() {
	var wg sync.WaitGroup
	defer timer("benchmark")()
	for i := 1; i <= 50000; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			call_rust_render_template("Good afternoon, {{name}}", "{\"name\":\"hari\"}")
		}()
	}
	wg.Wait()
}

func benchmark_wo_goroutine() {
	defer timer("benchmark")()
	for i := 1; i <= 50000; i++ {
		call_rust_render_template("Good afternoon, {{name}}", "{\"name\":\"hari\"}")
	}
}

func call_rust_render_template(template string, data string) string {
	template_c := C.CString(template)
	data_c := C.CString(data)
	defer C.free(unsafe.Pointer(template_c))
	defer C.free(unsafe.Pointer(data_c))
	rString := C.render_template(template_c, data_c)
	// we need to free it this way because of this https://doc.rust-lang.org/std/ffi/struct.CString.html#method.into_raw
	defer C.free_rust_string(rString)
	return C.GoString(rString)

}
func timer(name string) func() {
	start := time.Now()
	return func() {
		fmt.Printf("%s took %v\n", name, time.Since(start))
	}
}
