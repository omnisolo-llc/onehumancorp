package main

import (
	"fmt"
	"reflect"
	authpb "github.com/onehumancorp/mono/srcs/proto/ohc/auth"
)

func main() {
	u := authpb.User{}
	t := reflect.TypeOf(u)
	fmt.Printf("Fields for authpb.User:\n")
	for i := 0; i < t.NumField(); i++ {
		fmt.Printf("  %s\n", t.Field(i).Name)
	}
}
