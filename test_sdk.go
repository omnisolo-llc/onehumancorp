package main

import (
	"fmt"
	"reflect"

	"github.com/modelcontextprotocol/go-sdk/mcp"
)

func main() {
	fmt.Println(reflect.TypeOf(mcp.Tool{}))
}
