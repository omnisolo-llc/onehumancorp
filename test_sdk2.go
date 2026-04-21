package main

import (
	"fmt"
	"reflect"

	"github.com/modelcontextprotocol/go-sdk/mcp"
)

func main() {
	t := mcp.Tool{}
	v := reflect.ValueOf(t)
	typ := v.Type()
	for i := 0; i < typ.NumField(); i++ {
		field := typ.Field(i)
		fmt.Printf("%s %s\n", field.Name, field.Type)
	}
}
