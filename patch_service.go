package main

import (
	"bytes"
	"fmt"
	"io/ioutil"
	"strings"
)

func main() {
    content, err := ioutil.ReadFile("srcs/server/orchestration/service.go")
    if err != nil {
        panic(err)
    }

    if !strings.Contains(string(content), "AdvertiseCapabilities") {
        idx := bytes.Index(content, []byte("func (s *HubServiceServer) DiscoverAgents"))
        if idx != -1 {
            var newContent bytes.Buffer
            newContent.Write(content[:idx])
            newContent.WriteString("func (s *HubServiceServer) AdvertiseCapabilities(ctx context.Context, req *pb.AgentCapabilities) (*pb.PublishMessageResponse, error) {\n")
            newContent.WriteString("\tif s.mesh == nil {\n")
            newContent.WriteString("\t\treturn nil, fmt.Errorf(\"mesh transport not configured\")\n")
            newContent.WriteString("\t}\n")
            newContent.WriteString("\terr := s.mesh.AdvertiseCapabilities(ctx, *req)\n")
            newContent.WriteString("\tif err != nil {\n")
            newContent.WriteString("\t\treturn nil, err\n")
            newContent.WriteString("\t}\n")
            newContent.WriteString("\treturn &pb.PublishMessageResponse{Success: true}, nil\n")
            newContent.WriteString("}\n\n")
            newContent.Write(content[idx:])
            ioutil.WriteFile("srcs/server/orchestration/service.go", newContent.Bytes(), 0644)
            fmt.Println("Patched service.go")
        }
    } else {
        fmt.Println("Already patched")
    }
}
