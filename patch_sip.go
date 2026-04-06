package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	content, err := os.ReadFile("srcs/server/orchestration/sip.go")
	if err != nil {
		fmt.Println("Error reading:", err)
		return
	}

	strContent := string(content)

	// Add ClearSemaphore
	clearSemaphoreCode := `
func ClearSemaphore() {
	if os.Getenv("OHC_STANDALONE") == "true" {
		select {
		case <-standaloneThrottle:
		default:
		}
	}
}
`
	if !strings.Contains(strContent, "func ClearSemaphore()") {
		// insert after releaseThrottle
		insertPoint := "func releaseThrottle() {\n\tif os.Getenv(\"OHC_STANDALONE\") == \"true\" {\n\t\t<-standaloneThrottle\n\t}\n}"
		if strings.Contains(strContent, insertPoint) {
			strContent = strings.Replace(strContent, insertPoint, insertPoint+"\n"+clearSemaphoreCode, 1)
		} else {
			fmt.Println("Could not find insert point")
		}
	}

	err = os.WriteFile("srcs/server/orchestration/sip.go", []byte(strContent), 0644)
	if err != nil {
		fmt.Println("Error writing:", err)
	} else {
		fmt.Println("Patched successfully")
	}
}
