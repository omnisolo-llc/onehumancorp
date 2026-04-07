package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
    content, _ := os.ReadFile("srcs/server/main.go")
    str := string(content)

    str = strings.Replace(str, "// 5. Start the builtin agent process (Rust binary).", "// 5. Start the Go builtin agent process.", 1)
    str = strings.Replace(str, "startBuiltinAgentProcess(ctx, grpcEndpoint)", `
    // Start local builtin agent
    go func() {
        time.Sleep(1 * time.Second)
        cfg := local.AgentConfig{} // default config
        runner, err := local.StartDefaultRunner(hub, cfg)
        if err != nil {
            slog.Error("failed to start local runner", "error", err)
            return
        }

        <-ctx.Done()
        runner.Stop()
    }()
`, 1)

    str = strings.Replace(str, "\"github.com/onehumancorp/mono/srcs/server/dashboard\"", "\"github.com/onehumancorp/mono/srcs/server/dashboard\"\n\t\"github.com/onehumancorp/mono/srcs/server/agents/local\"", 1)

    os.WriteFile("srcs/server/main.go", []byte(str), 0644)
    fmt.Println("done")
}
