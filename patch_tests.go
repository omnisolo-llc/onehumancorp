package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	// Look at sip_throttling_test.go to clear semaphore
	content, err := os.ReadFile("srcs/server/orchestration/sip_throttling_test.go")
	if err == nil {
		strContent := string(content)
		if !strings.Contains(strContent, "orchestration.ClearSemaphore()") && !strings.Contains(strContent, "ClearSemaphore()") {
			// Find os.Setenv("OHC_STANDALONE", "true")
			// Add defer orchestration.ClearSemaphore() or something
			fmt.Println("Might need to update sip_throttling_test.go to use ClearSemaphore()")
		}
	}

	content, err = os.ReadFile("srcs/server/orchestration/throttle_test.go")
	if err == nil {
		strContent := string(content)
		if !strings.Contains(strContent, "ClearSemaphore()") {
			// replace `defer os.Setenv("OHC_STANDALONE", os.Getenv("OHC_STANDALONE"))`
			// with also clearing the semaphore
			insertPoint := "defer os.Setenv(\"OHC_STANDALONE\", os.Getenv(\"OHC_STANDALONE\"))"
			if strings.Contains(strContent, insertPoint) {
				strContent = strings.Replace(strContent, insertPoint, insertPoint+"\n\tdefer ClearSemaphore()", 1)
				os.WriteFile("srcs/server/orchestration/throttle_test.go", []byte(strContent), 0644)
				fmt.Println("Patched throttle_test.go")
			}
		}
	}
}
