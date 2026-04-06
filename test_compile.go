package main

import (
	"os"
	"os/exec"
)

func main() {
	cmd := exec.Command("go", "build", "./srcs/server/orchestration")
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	cmd.Run()
}
