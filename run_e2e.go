package main

import (
	"fmt"
	"os/exec"
)

func main() {
	cmd := exec.Command("bazelisk", "test", "//src/tests/e2e:e2e_misc_test", "--test_output=all", "--nocache_test_results")
	out, err := cmd.CombinedOutput()
	fmt.Printf("%s\n", out)
	if err != nil {
		fmt.Printf("Error: %v\n", err)
	}
}
