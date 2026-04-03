package main

import (
	"context"
	"fmt"
	"os"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func main() {
    os.Setenv("OHC_STANDALONE", "true")
    os.Setenv("DATABASE_URL", "")
    ctx := context.Background()

    d, err := db.New(ctx)
    if err != nil {
        fmt.Println("Error:", err)
        return
    }
    defer d.Close()
    fmt.Println("Success")
}
