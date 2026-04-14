package main

import (
    "fmt"
    "io/ioutil"
    "strings"
    "regexp"
)

func main() {
    content, err := ioutil.ReadFile("srcs/server/orchestration/mesh.go")
    if err != nil {
        fmt.Println("Error reading file:", err)
        return
    }

    strContent := string(content)

    // Remove the duplicate RedisMeshTransport.AdvertiseCapabilities

    re := regexp.MustCompile(`(?s)func \(rm \*RedisMeshTransport\) AdvertiseCapabilities\(ctx context\.Context, caps pb\.AgentCapabilities\) error \{\n\treturn fmt\.Errorf\("not implemented"\)\n\}\n\nfunc \(rm \*RedisMeshTransport\) AdvertiseCapabilities\(ctx context\.Context, caps pb\.AgentCapabilities\) error \{\n\treturn fmt\.Errorf\("not implemented"\)\n\}`)

    strContent = re.ReplaceAllString(strContent, "func (rm *RedisMeshTransport) AdvertiseCapabilities(ctx context.Context, caps pb.AgentCapabilities) error {\n\treturn fmt.Errorf(\"not implemented\")\n}")

    // Fallback if the first didn't work (they might not be strictly adjacent)
    re2 := regexp.MustCompile(`func \(rm \*RedisMeshTransport\) AdvertiseCapabilities\(ctx context\.Context, caps pb\.AgentCapabilities\) error \{\n\treturn fmt\.Errorf\("not implemented"\)\n\}`)
    matches := re2.FindAllStringIndex(strContent, -1)
    if len(matches) > 1 {
        // Replace the second occurrence
        start := matches[1][0]
        end := matches[1][1]
        strContent = strContent[:start] + strContent[end:]
    }


    err = ioutil.WriteFile("srcs/server/orchestration/mesh.go", []byte(strContent), 0644)
    if err != nil {
        fmt.Println("Error writing file:", err)
    }
}
