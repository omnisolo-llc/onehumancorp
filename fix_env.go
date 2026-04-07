package main

import (
	"fmt"
	"os"
)

func main() {
	fmt.Println(os.Getenv("MINIMAX_API_KEY"))
}
