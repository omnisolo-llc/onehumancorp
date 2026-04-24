The review explicitly stated:
"Because OHC uses `modernc.org/sqlite` (a pure-Go driver), it does not natively support C extensions like `sqlite-vec`. Vector operations (like `vec_distance_cosine`) must be implemented as Go-based User-Defined Functions (UDFs) via `sqlite.MustRegisterDeterministicScalarFunction`."

Let's register this UDF in `src/server/db/sqlite_provider.go`.

```go
package db

import (
	"math"
	"encoding/json"
	"modernc.org/sqlite"
	"modernc.org/sqlite/lib"
)

func init() {
	sqlite.MustRegisterDeterministicScalarFunction("vec_distance_cosine", 2, func(ctx *sqlite.FunctionContext, args []sqlite.Value) (sqlite.Value, error) {
		// parse two JSON array strings and compute cosine distance
	})
}
```
Wait, we can just do it in `src/server/db/sqlite_provider.go` inside an `init()` block or in `NewSqliteProvider`.

Let's check `src/server/db/sqlite_provider.go`.
