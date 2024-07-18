package main
import "fmt"


func Fibonacci(index int) int {
	if index < 0 {
		return -1
	}
	if index == 0 {
		return 0
	}
	if index == 1 {
		return 1
	} else {
		return Fibonacci(index-1) + Fibonacci(index-2)
	}
}


func main(){
	const n = 4
	fmt.Printf("fibonacci de %v = %v \n", n, Fibonacci(n))
}

//go run main.go