package auth

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"strings"
	"time"
)

// SqliteUserRepository implements UserRepository backed by SQLite.
type SqliteUserRepository struct {
	db *sql.DB
}

// NewSqliteUserRepository creates a SQLite-backed user repository.
func NewSqliteUserRepository(db *sql.DB) *SqliteUserRepository {
	return &SqliteUserRepository{db: db}
}

func (r *SqliteUserRepository) CreateUser(ctx context.Context, user *User) error {
	rolesBytes, _ := json.Marshal(user.Roles)
	if len(user.Roles) == 0 {
		rolesBytes = []byte("[]")
	}

	createdAt := user.CreatedAt.Format("2006-01-02 15:04:05")
	updatedAt := user.UpdatedAt.Format("2006-01-02 15:04:05")

	_, err := r.db.ExecContext(ctx, `
		INSERT INTO users (id, username, email, password_hash, roles, active, organization_id, oidc_subject, created_at, updated_at)
		VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)`,
		user.ID, user.Username, user.Email, user.PasswordHash,
		string(rolesBytes), user.Active, user.OrganizationID,
		nilIfEmpty(user.OIDCSubject),
		createdAt, updatedAt,
	)
	if err != nil {
		return fmt.Errorf("sqlite: create user: %w", err)
	}
	return nil
}

func (r *SqliteUserRepository) GetByID(ctx context.Context, id string) (*User, error) {
	return r.scanUser(ctx, "SELECT id, username, email, password_hash, roles, active, organization_id, COALESCE(oidc_subject,''), created_at, updated_at FROM users WHERE id = ?", id)
}

func (r *SqliteUserRepository) GetByUsername(ctx context.Context, username string) (*User, error) {
	return r.scanUser(ctx, "SELECT id, username, email, password_hash, roles, active, organization_id, COALESCE(oidc_subject,''), created_at, updated_at FROM users WHERE username = ?", username)
}

func (r *SqliteUserRepository) GetByEmail(ctx context.Context, email string) (*User, error) {
	return r.scanUser(ctx, "SELECT id, username, email, password_hash, roles, active, organization_id, COALESCE(oidc_subject,''), created_at, updated_at FROM users WHERE email = ?", email)
}

func (r *SqliteUserRepository) GetByOIDCSubject(ctx context.Context, sub string) (*User, error) {
	return r.scanUser(ctx, "SELECT id, username, email, password_hash, roles, active, organization_id, COALESCE(oidc_subject,''), created_at, updated_at FROM users WHERE oidc_subject = ?", sub)
}

func (r *SqliteUserRepository) ListUsers(ctx context.Context) ([]*User, error) {
	rows, err := r.db.QueryContext(ctx, "SELECT id, username, email, password_hash, roles, active, organization_id, COALESCE(oidc_subject,''), created_at, updated_at FROM users ORDER BY created_at")
	if err != nil {
		return nil, fmt.Errorf("sqlite: list users: %w", err)
	}
	defer rows.Close()

	var users []*User
	for rows.Next() {
		u := &User{}
		var rolesStr string
		var cStr, uStr string
		if err := rows.Scan(&u.ID, &u.Username, &u.Email, &u.PasswordHash, &rolesStr, &u.Active, &u.OrganizationID, &u.OIDCSubject, &cStr, &uStr); err != nil {
			return nil, fmt.Errorf("sqlite: scan user: %w", err)
		}

		if err := json.Unmarshal([]byte(rolesStr), &u.Roles); err != nil {
			u.Roles = []string{}
		}
		if t, err := time.Parse("2006-01-02 15:04:05", cStr); err == nil {
			u.CreatedAt = t
		}
		if t, err := time.Parse("2006-01-02 15:04:05", uStr); err == nil {
			u.UpdatedAt = t
		}

		users = append(users, u)
	}
	return users, nil
}

func (r *SqliteUserRepository) UpdateUser(ctx context.Context, user *User) error {
	rolesBytes, _ := json.Marshal(user.Roles)
	if len(user.Roles) == 0 {
		rolesBytes = []byte("[]")
	}

	updatedAt := user.UpdatedAt.Format("2006-01-02 15:04:05")

	_, err := r.db.ExecContext(ctx, `
		UPDATE users SET username=?, email=?, password_hash=?, roles=?, active=?,
		organization_id=?, oidc_subject=?, updated_at=?
		WHERE id=?`,
		user.Username, user.Email, user.PasswordHash, string(rolesBytes), user.Active,
		user.OrganizationID, nilIfEmpty(user.OIDCSubject), updatedAt,
		user.ID,
	)
	if err != nil {
		return fmt.Errorf("sqlite: update user: %w", err)
	}
	return nil
}

func (r *SqliteUserRepository) DeleteUser(ctx context.Context, id string) error {
	_, err := r.db.ExecContext(ctx, "DELETE FROM users WHERE id = ?", id)
	if err != nil {
		return fmt.Errorf("sqlite: delete user: %w", err)
	}
	return nil
}

func (r *SqliteUserRepository) RevokeToken(ctx context.Context, jti string, exp time.Time) error {
	expStr := exp.Format("2006-01-02 15:04:05")
	_, err := r.db.ExecContext(ctx, `
		INSERT INTO revoked_tokens (jti, expires_at) VALUES (?, ?)
		ON CONFLICT(jti) DO NOTHING`, jti, expStr)
	if err != nil {
		return fmt.Errorf("sqlite: revoke token: %w", err)
	}
	// GC expired entries.
	_, _ = r.db.ExecContext(ctx, "DELETE FROM revoked_tokens WHERE expires_at < CURRENT_TIMESTAMP")
	return nil
}

func (r *SqliteUserRepository) IsRevoked(ctx context.Context, jti string) (bool, error) {
	var count int
	err := r.db.QueryRowContext(ctx, "SELECT COUNT(*) FROM revoked_tokens WHERE jti = ? AND expires_at >= CURRENT_TIMESTAMP", jti).Scan(&count)
	if err != nil {
		return false, fmt.Errorf("sqlite: check revoked: %w", err)
	}
	return count > 0, nil
}

// --- helpers ---

func (r *SqliteUserRepository) scanUser(ctx context.Context, query string, args ...any) (*User, error) {
	u := &User{}
	var rolesStr string
	var cStr, uStr string
	err := r.db.QueryRowContext(ctx, query, args...).Scan(
		&u.ID, &u.Username, &u.Email, &u.PasswordHash,
		&rolesStr, &u.Active, &u.OrganizationID, &u.OIDCSubject,
		&cStr, &uStr,
	)
	if err != nil {
		if err == sql.ErrNoRows {
			return nil, ErrUserNotFound
		}
		if strings.Contains(err.Error(), "no rows in result set") {
			return nil, ErrUserNotFound
		}
		return nil, fmt.Errorf("sqlite: scan user: %w", err)
	}

	if err := json.Unmarshal([]byte(rolesStr), &u.Roles); err != nil {
		u.Roles = []string{}
	}
	if t, err := time.Parse("2006-01-02 15:04:05", cStr); err == nil {
		u.CreatedAt = t
	}
	if t, err := time.Parse("2006-01-02 15:04:05", uStr); err == nil {
		u.UpdatedAt = t
	}

	return u, nil
}
