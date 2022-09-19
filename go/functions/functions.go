package main

import (
	"fmt"
	"reflect"
)

var (
	area = func(l int, b int) int {
		return l * b
	}
)

func rectangle(l int, b int) (area int, parameter int) {
	parameter = 2 * (l + b)
	area = l * b
	return
}
func main() {
	var a, p int
	a, p = rectangle(10, 20)
	fmt.Println("Area = ", a, "Parameter = ", p)

	fmt.Println("Area =", area(20, 30))

	fmt.Println(squareSum(5)(6)(7))

	fmt.Println(square_Sum(5)(6)(7))

	variadicExample(1, "red", true, 10.5, []string{"foo", "bar", "baz"},
		map[string]int{"apple": 23, "tomato": 13})
}

func squareSum(x int) func(int) func(int) int {
	return func(y int) func(int) int {
		return func(z int) int {
			return x*x + y*y + z*z
		}
	}
}

type First func(int) int
type Second func(int) First

func square_Sum(x int) Second {
	return func(y int) First {
		return func(z int) int {
			return x*x + y*y + z*z
		}
	}
}

func variadicExample(i ...interface{}) {
	for _, v := range i {
		fmt.Println(v, "--", reflect.ValueOf(v).Kind())
	}
}
