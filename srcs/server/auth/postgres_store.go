package auth

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/lib/crypto"
)

// PgUserRepository implements UserRepository backed by PostgreSQL.
type PgUserRepository struct {
	pool db.Provider
}

// NewPgUserRepository creates a Postgres-backed user repository.
func NewPgUserRepository(pool db.Provider) *PgUserRepository {
	return &PgUserRepository{pool: pool}
}

func (r *PgUserRepository) CreateUser(ctx context.Context, user *User) error {
	rolesJSON, _ := json.Marshal(user.Roles)
	var rolesArg any = user.Roles
	if r.pool.IsSQLite() {
		rolesArg = string(rolesJSON)
	}

	email := user.Email
	oidcSub := user.OIDCSubject
	if key := os.Getenv("OHC_SQLITE_ENCRYPTION_KEY"); key != "" {
		var err error
		email, err = crypto.EncryptDeterministic(email, key)
		if err != nil {
			return fmt.Errorf("pg: encrypt email: %w", err)
		}
		if oidcSub != "" {
			oidcSub, err = crypto.EncryptDeterministic(oidcSub, key)
			if err != nil {
				return fmt.Errorf("pg: encrypt oidc_subject: %w", err)
			}
		}
	}

	_, err := r.pool.Exec(ctx, `
		INSERT INTO users (id, username, email, password_hash, roles, active, organization_id, oidc_subject, created_at, updated_at)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)`,
		user.ID, user.Username, email, user.PasswordHash,
		rolesArg, user.Active, user.OrganizationID,
		nilIfEmpty(oidcSub),
		user.CreatedAt, user.UpdatedAt,
	)
	if err != nil {
		return fmt.Errorf("pg: create user: %w", err)
	}
	return nil
}

func (r *PgUserRepository) GetByID(ctx context.Context, orgID, id string) (*User, error) {
	query := "SELECT id, username, email, password_hash, roles, active, organization_id, COALESCE(oidc_subject,''), created_at, updated_at FROM users WHERE id = $1"
	args := []any{id}
	if orgID != "" && orgID != "sys" {
		query += " AND organization_id = $2"
		args = append(args, orgID)
	}
	return r.scanUser(ctx, query, args...)
}

func (r *PgUserRepository) GetByUsername(ctx context.Context, orgID, username string) (*User, error) {
	query := "SELECT id, username, email, password_hash, roles, active, organization_id, COALESCE(oidc_subject,''), created_at, updated_at FROM users WHERE username = $1"
	args := []any{username}
	if orgID != "" && orgID != "sys" {
		query += " AND organization_id = $2"
		args = append(args, orgID)
	}
	return r.scanUser(ctx, query, args...)
}

func (r *PgUserRepository) GetByEmail(ctx context.Context, orgID, email string) (*User, error) {
	if key := os.Getenv("OHC_SQLITE_ENCRYPTION_KEY"); key != "" {
		var err error
		email, err = crypto.EncryptDeterministic(email, key)
		if err != nil {
			return nil, fmt.Errorf("pg: encrypt email for lookup: %w", err)
		}
	}
	query := "SELECT id, username, email, password_hash, roles, active, organization_id, COALESCE(oidc_subject,''), created_at, updated_at FROM users WHERE email = $1"
	args := []any{email}
	if orgID != "" && orgID != "sys" {
		query += " AND organization_id = $2"
		args = append(args, orgID)
	}
	return r.scanUser(ctx, query, args...)
}

func (r *PgUserRepository) GetByOIDCSubject(ctx context.Context, orgID, sub string) (*User, error) {
	if key := os.Getenv("OHC_SQLITE_ENCRYPTION_KEY"); key != "" {
		var err error
		sub, err = crypto.EncryptDeterministic(sub, key)
		if err != nil {
			return nil, fmt.Errorf("pg: encrypt oidc_subject for lookup: %w", err)
		}
	}
	query := "SELECT id, username, email, password_hash, roles, active, organization_id, COALESCE(oidc_subject,''), created_at, updated_at FROM users WHERE oidc_subject = $1"
	args := []any{sub}
	if orgID != "" && orgID != "sys" {
		query += " AND organization_id = $2"
		args = append(args, orgID)
	}
	return r.scanUser(ctx, query, args...)
}

func (r *PgUserRepository) ListUsers(ctx context.Context, orgID string) ([]*User, error) {
	query := "SELECT id, username, email, password_hash, roles, active, organization_id, COALESCE(oidc_subject,''), created_at, updated_at FROM users"
	var args []any
	if orgID != "" && orgID != "sys" {
		query += " WHERE organization_id = $1"
		args = append(args, orgID)
	}
	query += " ORDER BY created_at"
	rows, err := r.pool.Query(ctx, query, args...)
	if err != nil {
		return nil, fmt.Errorf("pg: list users: %w", err)
	}
	defer rows.Close()

	var users []*User
	for rows.Next() {
		u := &User{}
		var created, updated db.FlexTime

		if r.pool.IsSQLite() {
			var rolesJSON string
			if err := rows.Scan(&u.ID, &u.Username, &u.Email, &u.PasswordHash, &rolesJSON, &u.Active, &u.OrganizationID, &u.OIDCSubject, &created, &updated); err != nil {
				return nil, fmt.Errorf("pg: scan user: %w", err)
			}
			_ = json.Unmarshal([]byte(rolesJSON), &u.Roles)
		} else {
			if err := rows.Scan(&u.ID, &u.Username, &u.Email, &u.PasswordHash, &u.Roles, &u.Active, &u.OrganizationID, &u.OIDCSubject, &created, &updated); err != nil {
				return nil, fmt.Errorf("pg: scan user: %w", err)
			}
		}
		u.CreatedAt = created.Time
		u.UpdatedAt = updated.Time
		users = append(users, u)
	}
	return users, nil
}

func (r *PgUserRepository) UpdateUser(ctx context.Context, user *User) error {
	rolesJSON, _ := json.Marshal(user.Roles)
	var rolesArg any = user.Roles
	if r.pool.IsSQLite() {
		rolesArg = string(rolesJSON)
	}

	email := user.Email
	oidcSub := user.OIDCSubject
	if key := os.Getenv("OHC_SQLITE_ENCRYPTION_KEY"); key != "" {
		var err error
		email, err = crypto.EncryptDeterministic(email, key)
		if err != nil {
			return fmt.Errorf("pg: encrypt email: %w", err)
		}
		if oidcSub != "" {
			oidcSub, err = crypto.EncryptDeterministic(oidcSub, key)
			if err != nil {
				return fmt.Errorf("pg: encrypt oidc_subject: %w", err)
			}
		}
	}

	_, err := r.pool.Exec(ctx, `
		UPDATE users SET username=$2, email=$3, password_hash=$4, roles=$5, active=$6,
		organization_id=$7, oidc_subject=$8, updated_at=$9
		WHERE id=$1`,
		user.ID, user.Username, email, user.PasswordHash,
		rolesArg, user.Active, user.OrganizationID,
		nilIfEmpty(oidcSub), user.UpdatedAt,
	)
	if err != nil {
		return fmt.Errorf("pg: update user: %w", err)
	}
	return nil
}

func (r *PgUserRepository) DeleteUser(ctx context.Context, id string) error {
	_, err := r.pool.Exec(ctx, "DELETE FROM users WHERE id = $1", id)
	if err != nil {
		return fmt.Errorf("pg: delete user: %w", err)
	}
	return nil
}

func (r *PgUserRepository) RevokeToken(ctx context.Context, jti string, exp time.Time) error {
	_, err := r.pool.Exec(ctx, `
		INSERT INTO revoked_tokens (jti, expires_at) VALUES ($1, $2)
		ON CONFLICT (jti) DO NOTHING`, jti, exp)
	if err != nil {
		return fmt.Errorf("pg: revoke token: %w", err)
	}
	// GC expired entries.
	_, _ = r.pool.Exec(ctx, "DELETE FROM revoked_tokens WHERE expires_at < CURRENT_TIMESTAMP")
	return nil
}

func (r *PgUserRepository) IsRevoked(ctx context.Context, jti string) (bool, error) {
	var count int
	err := r.pool.QueryRow(ctx, "SELECT COUNT(*) FROM revoked_tokens WHERE jti = $1 AND expires_at >= CURRENT_TIMESTAMP", jti).Scan(&count)
	if err != nil {
		return false, fmt.Errorf("pg: check revoked: %w", err)
	}
	return count > 0, nil
}

// --- helpers ---

func (r *PgUserRepository) scanUser(ctx context.Context, query string, args ...any) (*User, error) {
	u := &User{}
	var created, updated db.FlexTime

	if r.pool.IsSQLite() {
		var rolesJSON string
		err := r.pool.QueryRow(ctx, query, args...).Scan(
			&u.ID, &u.Username, &u.Email, &u.PasswordHash,
			&rolesJSON, &u.Active, &u.OrganizationID, &u.OIDCSubject,
			&created, &updated,
		)
		if err != nil {
			if strings.Contains(err.Error(), "no rows in result set") || strings.Contains(err.Error(), "sql: no rows in result set") {
				return nil, ErrUserNotFound
			}
			return nil, fmt.Errorf("pg: scan user: %w", err)
		}
		_ = json.Unmarshal([]byte(rolesJSON), &u.Roles)

		if key := os.Getenv("OHC_SQLITE_ENCRYPTION_KEY"); key != "" {
			if dec, err := crypto.Decrypt(u.Email, key); err == nil {
				u.Email = dec
			}
			if u.OIDCSubject != "" {
				if dec, err := crypto.Decrypt(u.OIDCSubject, key); err == nil {
					u.OIDCSubject = dec
				}
			}
		}

		u.CreatedAt = created.Time
		u.UpdatedAt = updated.Time
		return u, nil
	}

	err := r.pool.QueryRow(ctx, query, args...).Scan(
		&u.ID, &u.Username, &u.Email, &u.PasswordHash,
		&u.Roles, &u.Active, &u.OrganizationID, &u.OIDCSubject,
		&created, &updated,
	)
	if err != nil {
		if strings.Contains(err.Error(), "no rows in result set") {
			return nil, ErrUserNotFound
		}
		return nil, fmt.Errorf("pg: scan user: %w", err)
	}

	if key := os.Getenv("OHC_SQLITE_ENCRYPTION_KEY"); key != "" {
		if dec, err := crypto.Decrypt(u.Email, key); err == nil {
			u.Email = dec
		}
		if u.OIDCSubject != "" {
			if dec, err := crypto.Decrypt(u.OIDCSubject, key); err == nil {
				u.OIDCSubject = dec
			}
		}
	}

	u.CreatedAt = created.Time
	u.UpdatedAt = updated.Time
	return u, nil
}

func nilIfEmpty(s string) *string {
	if s == "" {
		return nil
	}
	return &s
}
