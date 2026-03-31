package plane

import (
	"os"
)

// IsEnabled reports whether Plane is configured in the environment.
// Accepts no parameters.
// Returns bool.
// Produces no errors.
// Has no side effects.
func IsEnabled() bool {
	return os.Getenv("PLANE_URL") != "" || os.Getenv("PLANE_API_KEY") != ""
}
