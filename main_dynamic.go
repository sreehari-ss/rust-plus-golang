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
	"time"
	"unsafe"
)

func timer(name string) func() {
	start := time.Now()
	return func() {
		fmt.Printf("%s took %v\n", name, time.Since(start))
	}
}

func main() {
	// var wg sync.WaitGroup
	// defer timer("main")()
	// for i := 1; i <= 50000; i++ {
	// 	wg.Add(1)
	// 	go func() {
	// 		defer wg.Done()
	callrust()
	//		}()
	//	}
	//
	// wg.Wait()
}

func callrust() {

	template := C.CString("Good afternoon, {{name}}, isDefined {{ isdefined age}}")
	// print("hi")
	data := C.CString("{\"name\":\"hari\"}")
	defer C.free(unsafe.Pointer(template))
	defer C.free(unsafe.Pointer(data))

	rString := C.render_template(template, data)
	// we need to free it this way because of this https://doc.rust-lang.org/std/ffi/struct.CString.html#method.into_raw
	defer C.free_rust_string(rString)
	_r := C.GoString(rString)
	print(_r)
}
