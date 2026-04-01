package auth

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

// PgUserRepository implements UserRepository backed by PostgreSQL or SQLite.
type PgUserRepository struct {
	db db.Provider
}

// NewPgUserRepository creates a db.Provider-backed user repository.
func NewPgUserRepository(db db.Provider) *PgUserRepository {
	return &PgUserRepository{db: db}
}

func (r *PgUserRepository) CreateUser(ctx context.Context, user *User) error {
	var err error
	if r.db.IsPostgres() {
		_, err = r.db.ExecContext(ctx, `
			INSERT INTO users (id, username, email, password_hash, roles, active, organization_id, oidc_subject, created_at, updated_at)
			VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)`,
			user.ID, user.Username, user.Email, user.PasswordHash,
			user.Roles, user.Active, user.OrganizationID,
			nilIfEmpty(user.OIDCSubject),
			user.CreatedAt, user.UpdatedAt,
		)
	} else {
		rolesJson, _ := json.Marshal(user.Roles)
		_, err = r.db.ExecContext(ctx, `
			INSERT INTO users (id, username, email, password_hash, roles, active, organization_id, oidc_subject, created_at, updated_at)
			VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)`,
			user.ID, user.Username, user.Email, user.PasswordHash,
			string(rolesJson), user.Active, user.OrganizationID,
			nilIfEmpty(user.OIDCSubject),
			user.CreatedAt, user.UpdatedAt,
		)
	}
	if err != nil {
		return fmt.Errorf("db: create user: %w", err)
	}
	return nil
}

func (r *PgUserRepository) GetByID(ctx context.Context, id string) (*User, error) {
	if r.db.IsPostgres() {
		return r.scanUserPg(ctx, "SELECT id, username, email, password_hash, roles, active, organization_id, COALESCE(oidc_subject,''), created_at, updated_at FROM users WHERE id = $1", id)
	}
	return r.scanUserSqlite(ctx, "SELECT id, username, email, password_hash, roles, active, organization_id, COALESCE(oidc_subject,''), created_at, updated_at FROM users WHERE id = ?", id)
}

func (r *PgUserRepository) GetByUsername(ctx context.Context, username string) (*User, error) {
	if r.db.IsPostgres() {
		return r.scanUserPg(ctx, "SELECT id, username, email, password_hash, roles, active, organization_id, COALESCE(oidc_subject,''), created_at, updated_at FROM users WHERE username = $1", username)
	}
	return r.scanUserSqlite(ctx, "SELECT id, username, email, password_hash, roles, active, organization_id, COALESCE(oidc_subject,''), created_at, updated_at FROM users WHERE username = ?", username)
}

func (r *PgUserRepository) GetByEmail(ctx context.Context, email string) (*User, error) {
	if r.db.IsPostgres() {
		return r.scanUserPg(ctx, "SELECT id, username, email, password_hash, roles, active, organization_id, COALESCE(oidc_subject,''), created_at, updated_at FROM users WHERE email = $1", email)
	}
	return r.scanUserSqlite(ctx, "SELECT id, username, email, password_hash, roles, active, organization_id, COALESCE(oidc_subject,''), created_at, updated_at FROM users WHERE email = ?", email)
}

func (r *PgUserRepository) GetByOIDCSubject(ctx context.Context, sub string) (*User, error) {
	if r.db.IsPostgres() {
		return r.scanUserPg(ctx, "SELECT id, username, email, password_hash, roles, active, organization_id, COALESCE(oidc_subject,''), created_at, updated_at FROM users WHERE oidc_subject = $1", sub)
	}
	return r.scanUserSqlite(ctx, "SELECT id, username, email, password_hash, roles, active, organization_id, COALESCE(oidc_subject,''), created_at, updated_at FROM users WHERE oidc_subject = ?", sub)
}

func (r *PgUserRepository) ListUsers(ctx context.Context) ([]*User, error) {
	var rows interface {
		Next() bool
		Scan(...any) error
		Close() error
	}
	var err error

	if r.db.IsPostgres() {
		rows, err = r.db.QueryContext(ctx, "SELECT id, username, email, password_hash, roles, active, organization_id, COALESCE(oidc_subject,''), created_at, updated_at FROM users ORDER BY created_at")
	} else {
		rows, err = r.db.QueryContext(ctx, "SELECT id, username, email, password_hash, roles, active, organization_id, COALESCE(oidc_subject,''), created_at, updated_at FROM users ORDER BY created_at")
	}

	if err != nil {
		return nil, fmt.Errorf("db: list users: %w", err)
	}
	defer rows.Close()

	var users []*User
	for rows.Next() {
		u := &User{}
		if r.db.IsPostgres() {
			if err := rows.Scan(&u.ID, &u.Username, &u.Email, &u.PasswordHash, &u.Roles, &u.Active, &u.OrganizationID, &u.OIDCSubject, &u.CreatedAt, &u.UpdatedAt); err != nil {
				return nil, fmt.Errorf("db: scan user pg: %w", err)
			}
		} else {
			var rolesJson string
			if err := rows.Scan(&u.ID, &u.Username, &u.Email, &u.PasswordHash, &rolesJson, &u.Active, &u.OrganizationID, &u.OIDCSubject, &u.CreatedAt, &u.UpdatedAt); err != nil {
				return nil, fmt.Errorf("db: scan user sqlite: %w", err)
			}
			json.Unmarshal([]byte(rolesJson), &u.Roles)
		}
		users = append(users, u)
	}
	return users, nil
}

func (r *PgUserRepository) UpdateUser(ctx context.Context, user *User) error {
	var err error
	if r.db.IsPostgres() {
		_, err = r.db.ExecContext(ctx, `
			UPDATE users SET username=$2, email=$3, password_hash=$4, roles=$5, active=$6,
			organization_id=$7, oidc_subject=$8, updated_at=$9
			WHERE id=$1`,
			user.ID, user.Username, user.Email, user.PasswordHash,
			user.Roles, user.Active, user.OrganizationID,
			nilIfEmpty(user.OIDCSubject), user.UpdatedAt,
		)
	} else {
		rolesJson, _ := json.Marshal(user.Roles)
		_, err = r.db.ExecContext(ctx, `
			UPDATE users SET username=?, email=?, password_hash=?, roles=?, active=?,
			organization_id=?, oidc_subject=?, updated_at=?
			WHERE id=?`,
			user.Username, user.Email, user.PasswordHash, string(rolesJson), user.Active,
			user.OrganizationID, nilIfEmpty(user.OIDCSubject), user.UpdatedAt, user.ID,
		)
	}
	if err != nil {
		return fmt.Errorf("db: update user: %w", err)
	}
	return nil
}

func (r *PgUserRepository) DeleteUser(ctx context.Context, id string) error {
	var err error
	if r.db.IsPostgres() {
		_, err = r.db.ExecContext(ctx, "DELETE FROM users WHERE id = $1", id)
	} else {
		_, err = r.db.ExecContext(ctx, "DELETE FROM users WHERE id = ?", id)
	}
	if err != nil {
		return fmt.Errorf("db: delete user: %w", err)
	}
	return nil
}

func (r *PgUserRepository) RevokeToken(ctx context.Context, jti string, exp time.Time) error {
	var err error
	if r.db.IsPostgres() {
		_, err = r.db.ExecContext(ctx, `
			INSERT INTO revoked_tokens (jti, expires_at) VALUES ($1, $2)
			ON CONFLICT (jti) DO NOTHING`, jti, exp)
		if err != nil {
			return fmt.Errorf("db: revoke token pg: %w", err)
		}
		_, _ = r.db.ExecContext(ctx, "DELETE FROM revoked_tokens WHERE expires_at < NOW()")
	} else {
		_, err = r.db.ExecContext(ctx, `
			INSERT OR IGNORE INTO revoked_tokens (jti, expires_at) VALUES (?, ?)`, jti, exp)
		if err != nil {
			return fmt.Errorf("db: revoke token sqlite: %w", err)
		}
		_, _ = r.db.ExecContext(ctx, "DELETE FROM revoked_tokens WHERE expires_at < CURRENT_TIMESTAMP")
	}
	return nil
}

func (r *PgUserRepository) IsRevoked(ctx context.Context, jti string) (bool, error) {
	var count int
	var err error
	if r.db.IsPostgres() {
		err = r.db.QueryRowContext(ctx, "SELECT COUNT(*) FROM revoked_tokens WHERE jti = $1 AND expires_at >= NOW()", jti).Scan(&count)
	} else {
		err = r.db.QueryRowContext(ctx, "SELECT COUNT(*) FROM revoked_tokens WHERE jti = ? AND expires_at >= CURRENT_TIMESTAMP", jti).Scan(&count)
	}

	if err != nil {
		return false, fmt.Errorf("db: check revoked: %w", err)
	}
	return count > 0, nil
}

// --- helpers ---

func (r *PgUserRepository) scanUserPg(ctx context.Context, query string, args ...any) (*User, error) {
	u := &User{}
	err := r.db.QueryRowContext(ctx, query, args...).Scan(
		&u.ID, &u.Username, &u.Email, &u.PasswordHash,
		&u.Roles, &u.Active, &u.OrganizationID, &u.OIDCSubject,
		&u.CreatedAt, &u.UpdatedAt,
	)
	if err != nil {
		if strings.Contains(err.Error(), "no rows in result set") {
			return nil, ErrUserNotFound
		}
		return nil, fmt.Errorf("pg: scan user: %w", err)
	}
	return u, nil
}

func (r *PgUserRepository) scanUserSqlite(ctx context.Context, query string, args ...any) (*User, error) {
	u := &User{}
	var rolesJson string
	err := r.db.QueryRowContext(ctx, query, args...).Scan(
		&u.ID, &u.Username, &u.Email, &u.PasswordHash,
		&rolesJson, &u.Active, &u.OrganizationID, &u.OIDCSubject,
		&u.CreatedAt, &u.UpdatedAt,
	)
	if err != nil {
		if strings.Contains(err.Error(), "no rows in result set") {
			return nil, ErrUserNotFound
		}
		return nil, fmt.Errorf("sqlite: scan user: %w", err)
	}
	json.Unmarshal([]byte(rolesJson), &u.Roles)
	return u, nil
}


func nilIfEmpty(s string) *string {
	if s == "" {
		return nil
	}
	return &s
}
