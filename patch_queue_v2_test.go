package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	// Let's use NewSqliteProvider with CreateTestSqliteDB, but how do we access CreateTestSqliteDB?
	// Oh, `db.NewTestProvider` used to be `db.NewTestProvider(t)`!
	// The other files (e.g. `mesh_test.go` and `queue_test.go`) DO NOT have an error with `db.NewTestProvider(t)`.
	// Why only `queue_v2_test.go` fails?
	// Ah! In `queue_v2_test.go`, I have `import "github.com/onehumancorp/mono/srcs/server/db"`!
	// But `db.NewTestProvider(t)` works in `queue_test.go` because it's defined in the same test package maybe?
	// Let's check where `NewTestProvider` is defined!
}
