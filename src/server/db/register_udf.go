package db

import (
	"database/sql/driver"
	"encoding/json"
	"math"

	"modernc.org/sqlite"
)

func init() {
	_ = sqlite.RegisterDeterministicScalarFunction("vec_distance_cosine", 2, func(ctx *sqlite.FunctionContext, args []driver.Value) (driver.Value, error) {
		a, okA := args[0].(string)
		b, okB := args[1].(string)
		if !okA || !okB {
			return float64(1.0), nil
		}

		var v1, v2 []float32
		if err := json.Unmarshal([]byte(a), &v1); err != nil {
			return float64(1.0), nil
		}
		if err := json.Unmarshal([]byte(b), &v2); err != nil {
			return float64(1.0), nil
		}
		if len(v1) != len(v2) || len(v1) == 0 {
			return float64(1.0), nil
		}

		var dot, mag1, mag2 float64
		for i := range v1 {
			dot += float64(v1[i] * v2[i])
			mag1 += float64(v1[i] * v1[i])
			mag2 += float64(v2[i] * v2[i])
		}
		if mag1 == 0 || mag2 == 0 {
			return float64(1.0), nil
		}
		distance := 1.0 - (dot / (math.Sqrt(mag1) * math.Sqrt(mag2)))
		return distance, nil
	})
}
