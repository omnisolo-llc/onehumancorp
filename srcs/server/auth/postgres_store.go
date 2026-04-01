package auth

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

// PgUserRepository implements UserRepository backed by PostgreSQL and SQLite.
type PgUserRepository struct {
	pool db.Provider
}

// NewPgUserRepository creates a Database-backed user repository.
func NewPgUserRepository(pool db.Provider) *PgUserRepository {
	return &PgUserRepository{pool: pool}
}

func (r *PgUserRepository) CreateUser(ctx context.Context, user *User) error {
	ctx, span := db.Tracer().Start(ctx, "PgUserRepository.CreateUser")
	defer span.End()

	rolesJSON, _ := json.Marshal(user.Roles)
	_, isSqlite := r.pool.(*db.SqliteProvider)
	var rolesArg any = user.Roles
	if isSqlite {
		rolesArg = string(rolesJSON)
	}

	_, err := r.pool.Exec(ctx, `
		INSERT INTO users (id, username, email, password_hash, roles, active, organization_id, oidc_subject, created_at, updated_at)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)`,
		user.ID, user.Username, user.Email, user.PasswordHash,
		rolesArg, user.Active, user.OrganizationID,
		nilIfEmpty(user.OIDCSubject),
		user.CreatedAt, user.UpdatedAt,
	)
	if err != nil {
		return fmt.Errorf("db: create user: %w", err)
	}
	return nil
}

func (r *PgUserRepository) GetByID(ctx context.Context, id string) (*User, error) {
	ctx, span := db.Tracer().Start(ctx, "PgUserRepository.GetByID")
	defer span.End()
	return r.scanUser(ctx, "SELECT id, username, email, password_hash, roles, active, organization_id, COALESCE(oidc_subject,''), created_at, updated_at FROM users WHERE id = $1", id)
}

func (r *PgUserRepository) GetByUsername(ctx context.Context, username string) (*User, error) {
	ctx, span := db.Tracer().Start(ctx, "PgUserRepository.GetByUsername")
	defer span.End()
	return r.scanUser(ctx, "SELECT id, username, email, password_hash, roles, active, organization_id, COALESCE(oidc_subject,''), created_at, updated_at FROM users WHERE username = $1", username)
}

func (r *PgUserRepository) GetByEmail(ctx context.Context, email string) (*User, error) {
	ctx, span := db.Tracer().Start(ctx, "PgUserRepository.GetByEmail")
	defer span.End()
	return r.scanUser(ctx, "SELECT id, username, email, password_hash, roles, active, organization_id, COALESCE(oidc_subject,''), created_at, updated_at FROM users WHERE email = $1", email)
}

func (r *PgUserRepository) GetByOIDCSubject(ctx context.Context, sub string) (*User, error) {
	ctx, span := db.Tracer().Start(ctx, "PgUserRepository.GetByOIDCSubject")
	defer span.End()
	return r.scanUser(ctx, "SELECT id, username, email, password_hash, roles, active, organization_id, COALESCE(oidc_subject,''), created_at, updated_at FROM users WHERE oidc_subject = $1", sub)
}

func (r *PgUserRepository) ListUsers(ctx context.Context) ([]*User, error) {
	ctx, span := db.Tracer().Start(ctx, "PgUserRepository.ListUsers")
	defer span.End()

	rows, err := r.pool.Query(ctx, "SELECT id, username, email, password_hash, roles, active, organization_id, COALESCE(oidc_subject,''), created_at, updated_at FROM users ORDER BY created_at")
	if err != nil {
		return nil, fmt.Errorf("db: list users: %w", err)
	}
	defer rows.Close()

	var users []*User
	_, isSqlite := r.pool.(*db.SqliteProvider)
	for rows.Next() {
		u := &User{}
		if isSqlite {
			var rolesJSON string
			if err := rows.Scan(&u.ID, &u.Username, &u.Email, &u.PasswordHash, &rolesJSON, &u.Active, &u.OrganizationID, &u.OIDCSubject, &u.CreatedAt, &u.UpdatedAt); err != nil {
				return nil, fmt.Errorf("db: scan user: %w", err)
			}
			_ = json.Unmarshal([]byte(rolesJSON), &u.Roles)
		} else {
			if err := rows.Scan(&u.ID, &u.Username, &u.Email, &u.PasswordHash, &u.Roles, &u.Active, &u.OrganizationID, &u.OIDCSubject, &u.CreatedAt, &u.UpdatedAt); err != nil {
				return nil, fmt.Errorf("db: scan user: %w", err)
			}
		}
		users = append(users, u)
	}
	return users, nil
}

func (r *PgUserRepository) UpdateUser(ctx context.Context, user *User) error {
	ctx, span := db.Tracer().Start(ctx, "PgUserRepository.UpdateUser")
	defer span.End()

	rolesJSON, _ := json.Marshal(user.Roles)
	_, isSqlite := r.pool.(*db.SqliteProvider)
	var rolesArg any = user.Roles
	if isSqlite {
		rolesArg = string(rolesJSON)
	}

	_, err := r.pool.Exec(ctx, `
		UPDATE users SET username=$2, email=$3, password_hash=$4, roles=$5, active=$6,
		organization_id=$7, oidc_subject=$8, updated_at=$9
		WHERE id=$1`,
		user.ID, user.Username, user.Email, user.PasswordHash,
		rolesArg, user.Active, user.OrganizationID,
		nilIfEmpty(user.OIDCSubject), user.UpdatedAt,
	)
	if err != nil {
		return fmt.Errorf("db: update user: %w", err)
	}
	return nil
}

func (r *PgUserRepository) DeleteUser(ctx context.Context, id string) error {
	ctx, span := db.Tracer().Start(ctx, "PgUserRepository.DeleteUser")
	defer span.End()

	_, err := r.pool.Exec(ctx, "DELETE FROM users WHERE id = $1", id)
	if err != nil {
		return fmt.Errorf("db: delete user: %w", err)
	}
	return nil
}

func (r *PgUserRepository) RevokeToken(ctx context.Context, jti string, exp time.Time) error {
	ctx, span := db.Tracer().Start(ctx, "PgUserRepository.RevokeToken")
	defer span.End()

	_, err := r.pool.Exec(ctx, `
		INSERT INTO revoked_tokens (jti, expires_at) VALUES ($1, $2)
		ON CONFLICT (jti) DO NOTHING`, jti, exp)
	if err != nil {
		return fmt.Errorf("db: revoke token: %w", err)
	}
	// GC expired entries.
	_, _ = r.pool.Exec(ctx, "DELETE FROM revoked_tokens WHERE expires_at < CURRENT_TIMESTAMP")
	return nil
}

func (r *PgUserRepository) IsRevoked(ctx context.Context, jti string) (bool, error) {
	ctx, span := db.Tracer().Start(ctx, "PgUserRepository.IsRevoked")
	defer span.End()

	var count int
	err := r.pool.QueryRow(ctx, "SELECT COUNT(*) FROM revoked_tokens WHERE jti = $1 AND expires_at >= CURRENT_TIMESTAMP", jti).Scan(&count)
	if err != nil {
		return false, fmt.Errorf("db: check revoked: %w", err)
	}
	return count > 0, nil
}

// --- helpers ---

func (r *PgUserRepository) scanUser(ctx context.Context, query string, args ...any) (*User, error) {
	u := &User{}
	_, isSqlite := r.pool.(*db.SqliteProvider)

	if isSqlite {
		var rolesJSON string
		err := r.pool.QueryRow(ctx, query, args...).Scan(
			&u.ID, &u.Username, &u.Email, &u.PasswordHash,
			&rolesJSON, &u.Active, &u.OrganizationID, &u.OIDCSubject,
			&u.CreatedAt, &u.UpdatedAt,
		)
		if err != nil {
			if strings.Contains(err.Error(), "no rows in result set") || strings.Contains(err.Error(), "sql: no rows in result set") {
				return nil, ErrUserNotFound
			}
			return nil, fmt.Errorf("db: scan user: %w", err)
		}
		_ = json.Unmarshal([]byte(rolesJSON), &u.Roles)
		return u, nil
	}

	err := r.pool.QueryRow(ctx, query, args...).Scan(
		&u.ID, &u.Username, &u.Email, &u.PasswordHash,
		&u.Roles, &u.Active, &u.OrganizationID, &u.OIDCSubject,
		&u.CreatedAt, &u.UpdatedAt,
	)
	if err != nil {
		if strings.Contains(err.Error(), "no rows in result set") {
			return nil, ErrUserNotFound
		}
		return nil, fmt.Errorf("db: scan user: %w", err)
	}
	return u, nil
}

func nilIfEmpty(s string) *string {
	if s == "" {
		return nil
	}
	return &s
}
