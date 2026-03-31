package main

import (
	"fmt"
	"os"

	"github.com/onehumancorp/mono/scripts/cleanup"
)

func main() {
	if err := cleanup.RunCleanup(".agent-task", "."); err != nil {
		fmt.Printf("Cleanup failed: %v\n", err)
		os.Exit(1)
	}
}
