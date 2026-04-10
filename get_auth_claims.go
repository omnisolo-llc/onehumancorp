package main

import (
    "fmt"
    "github.com/onehumancorp/mono/srcs/server/auth"
)

func main() {
    fmt.Println("Claims exists:", auth.Claims{})
}
