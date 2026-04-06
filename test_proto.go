package main

import (
	"fmt"
	pb "github.com/onehumancorp/mono/srcs/proto"
)

func main() {
	var _ *pb.PublishMessageResponse = pb.PublishMessageResponse_builder{Success: pb.Bool(true)}.Build()
	fmt.Println("Compiles")
}
