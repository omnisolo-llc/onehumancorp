package main

import (
	"context"
	"fmt"
	"log"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func main() {
	prov := db.NewTestProvider(nil)
	fmt.Println(prov)
}
