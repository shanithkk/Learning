package main

import (
	"fmt"
	"strings"
)

func Roman() {
	fmt.Println("Roman to Number Conversion")
	roman := "XII"
	splt := strings.Split(roman, "")

	fmt.Println(splt)
	var sum int32 = 0
	for key := 0; key < len(splt); key++ {
		sum += get_value(splt[key])
	}
	fmt.Println(sum)
}

func get_value(char string) int32 {

	switch char {
	case "I":
		return 1
	case "V":
		return 5
	case "X":
		return 10

	default:
		return 0
	}

}
