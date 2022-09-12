package main

import "fmt"

func main() {
	var typeint8 int8
	var typeint16 int16
	var typeint32 int32
	var typeint64 int64
	fmt.Println(typeint8, typeint16, typeint32, typeint64)

	var typeuint8 uint8
	var typeuint16 uint16
	var typeuint32 uint32
	var typeuint64 uint64
	fmt.Println(typeuint8, typeuint16, typeuint32, typeuint64)

	var boolean bool
	var float float32
	fmt.Println(boolean, float)

	var kilometersToMars int32 = 62100000
	kilometersToMars = 62100000
	fmt.Println(kilometersToMars)

	StringsTest()

	var dynamic_declaration = 7
	fmt.Println(dynamic_declaration)

	direct_declaration := 8
	fmt.Println(direct_declaration)

	//Muliple Declaration

	var multiple, declaration int32
	fmt.Println(multiple, declaration)
}
