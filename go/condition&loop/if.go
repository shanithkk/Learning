package main

import "fmt"

func main() {
	fmt.Println("Condition and Looping")

	x := 70
	if x == 50 {
		fmt.Println("Number is 50")
	} else if x > 50 {
		fmt.Println("Number is greater than 50")
	} else {
		fmt.Println("Number if less than 50")
	}

	fmt.Println("Variable initialization and condition")

	if x := 100; x == 100 {
		fmt.Println("Value is ", x)
	}

}
