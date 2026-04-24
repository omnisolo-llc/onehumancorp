package db

import (
	"database/sql/driver"
	"encoding/json"
	"math"

	"modernc.org/sqlite"
)

func init() {
	sqlite.MustRegisterDeterministicScalarFunction("vec_distance", 2, func(ctx *sqlite.FunctionContext, args []driver.Value) (driver.Value, error) {
		var aBytes, bBytes []byte

		switch v := args[0].(type) {
		case string:
			aBytes = []byte(v)
		case []byte:
			aBytes = v
		default:
			return 2.0, nil // Max cosine distance to drop to bottom
		}

		switch v := args[1].(type) {
		case string:
			bBytes = []byte(v)
		case []byte:
			bBytes = v
		default:
			return 2.0, nil
		}

		var a, b []float32
		if err := json.Unmarshal(aBytes, &a); err != nil {
			return 2.0, err
		}
		if err := json.Unmarshal(bBytes, &b); err != nil {
			return 2.0, err
		}

		if len(a) != len(b) {
			return 2.0, nil
		}

		var dotProduct, normA, normB float64
		for i := 0; i < len(a); i++ {
			dotProduct += float64(a[i]) * float64(b[i])
			normA += float64(a[i]) * float64(a[i])
			normB += float64(b[i]) * float64(b[i])
		}

		if normA == 0 || normB == 0 {
			return 2.0, nil
		}

		cosineSimilarity := dotProduct / (math.Sqrt(normA) * math.Sqrt(normB))
		return 1.0 - cosineSimilarity, nil
	})
}
