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
}
