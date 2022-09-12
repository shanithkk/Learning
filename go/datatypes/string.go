package main

import "fmt"

func StringsTest() {
	var test string
	test = "hello String"
	fmt.Println(test)

	var test2 string = "Hello Strings"
	fmt.Println(test2)

	var description string
	description = test + " is by: " + test2 + "."
	fmt.Println(description)

	var dynamic_declaration = "Dynamic Declaration"
	fmt.Println(dynamic_declaration)

	direct_declaration := "Direct Declaration"
	fmt.Println(direct_declaration)

	multiple, declaration := "Multiple declaration  string and booloean boolean value = ", true
	fmt.Println(multiple, declaration)
}
