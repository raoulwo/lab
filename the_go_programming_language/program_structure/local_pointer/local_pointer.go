package main

import "fmt"

func main() {
	fmt.Println(*f() == 1)
}

func f() *int {
	v := 1
	return &v
}
