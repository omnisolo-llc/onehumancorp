package auth

import (
	"context"
	"errors"
	"time"
)

// ErrUserNotFound is returned when a repository lookup misses.
var ErrUserNotFound = errors.New("user not found")

// UserRepository defines the persistence contract for user accounts and
// authentication state.  The in-memory Store satisfies this interface by
// default; a Postgres-backed implementation enables horizontal scaling.
type UserRepository interface {
	// CreateUser persists a new user.  Returns an error if the username or
	// email is already taken.
	CreateUser(ctx context.Context, user *User, orgID string) error
	// GetByID returns a user by primary key.
	GetByID(ctx context.Context, id string, orgID string) (*User, error)
	// GetByUsername returns the user matching the given username.
	GetByUsername(ctx context.Context, username string, orgID string) (*User, error)
	// GetByEmail returns the user matching the given email address.
	GetByEmail(ctx context.Context, email string, orgID string) (*User, error)
	// GetByOIDCSubject returns the user matching the given OIDC subject.
	GetByOIDCSubject(ctx context.Context, sub string, orgID string) (*User, error)
	// ListUsers returns every registered user.
	ListUsers(ctx context.Context, orgID string) ([]*User, error)
	// UpdateUser persists changes to a user record.
	UpdateUser(ctx context.Context, user *User, orgID string) error
	// DeleteUser removes a user by ID.
	DeleteUser(ctx context.Context, id string, orgID string) error

	// --- Token revocation ---

	// RevokeToken records a JWT ID (jti) as revoked until exp.
	RevokeToken(ctx context.Context, jti string, exp time.Time) error
	// IsRevoked reports whether a JWT ID has been revoked.
	IsRevoked(ctx context.Context, jti string) (bool, error)
}
