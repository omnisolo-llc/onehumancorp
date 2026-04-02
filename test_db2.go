package main

import (
	"context"
	"fmt"
	"os"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func main() {
    os.Setenv("OHC_STANDALONE", "true")
	os.Setenv("DATABASE_URL", "sqlite://file:testdb?mode=memory&cache=shared")
	prov, err := db.New(context.Background())
	if err != nil {
	    panic(err)
	}
	fmt.Println(prov.IsSQLite())
}
