package main

import "os"
import "fmt"

func main() {
    fmt.Println(os.Getenv("OHC_ENV_MODE"))
}
