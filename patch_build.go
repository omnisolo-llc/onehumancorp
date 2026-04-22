package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func main() {
	filePath := "srcs/server/db/BUILD.bazel"
	contentBytes, err := ioutil.ReadFile(filePath)
	if err != nil {
		fmt.Printf("Error reading file: %v\n", err)
		return
	}
	content := string(contentBytes)

	if strings.Contains(content, "\"@org_uber_go_mock//gomock\",") {
		content = strings.Replace(content, "\"@org_uber_go_mock//gomock\",\n", "", 1)
		err = ioutil.WriteFile(filePath, []byte(content), 0644)
		if err != nil {
			fmt.Printf("Error writing file: %v\n", err)
			return
		}
		fmt.Println("Successfully modified BUILD.bazel")
	} else {
		fmt.Println("BUILD.bazel already patched")
	}

    redisLockFilePath := "srcs/server/db/redis_lock_test.go"
    redisLockBytes, err := ioutil.ReadFile(redisLockFilePath)
    if err == nil {
        redisLockContent := string(redisLockBytes)
        redisLockContent = strings.Replace(redisLockContent, "\"go.uber.org/mock/gomock\"", "", 1)
        redisLockContent = strings.Replace(redisLockContent, "ctrl := gomock.NewController(t)\n\tdefer ctrl.Finish()\n\n\tmockClient := mock.NewClient(ctrl)", "mockClient := mock.NewClient(t)", 1)
        redisLockContent = strings.Replace(redisLockContent, "mockClient.EXPECT()", "mockClient.EXPECT()", -1) // gomock EXPECTs

        ioutil.WriteFile(redisLockFilePath, []byte(redisLockContent), 0644)
    }
}
