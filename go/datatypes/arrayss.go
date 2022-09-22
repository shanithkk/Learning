package main

import "fmt"

func Arrays() {
	x := [5]int{10, 20, 30, 40, 50}
	var y [5]int = [5]int{10, 20, 30}

	fmt.Println(x)
	fmt.Println(y)

	var theArray [3]string
	theArray[0] = "India"
	theArray[1] = "Canada"
	theArray[2] = "Japan"

	fmt.Println(theArray[0])
	fmt.Println(theArray[1])
	fmt.Println(theArray[2])

	intArray := [5]int{10, 20, 30, 40, 50}

	fmt.Println("\n---------------Example 1--------------------\n")
	for i := 0; i < len(intArray); i++ {
		fmt.Println(intArray[i])
	}

	fmt.Println("\n---------------Example 2--------------------\n")
	for index, element := range intArray {
		fmt.Println(index, "=>", element)

	}

	fmt.Println("\n---------------Example 3--------------------\n")
	for _, value := range intArray {
		fmt.Println(value)
	}

	j := 0
	fmt.Println("\n---------------Example 4--------------------\n")
	for range intArray {
		fmt.Println(intArray[j])
		j++
	}
}
