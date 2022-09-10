package main

import (
	"fmt"

	"rsc.io/quote"
)

const gravity = 9.8

func main() {
	fmt.Println(quote.Go())

	var nextTrainTime int8
	nextTrainTime = 12
	fmt.Println("Next train:", nextTrainTime, "minutes")

	fmt.Println("gravity of earth : ", gravity)
}
