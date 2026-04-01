package auth

import (
	"context"
	"fmt"
	"strings"
	"encoding/json"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
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
	var roles any = user.Roles
	if _, isSQLite := r.pool.(*db.SqlitePool); isSQLite {
		b, err := json.Marshal(user.Roles)
		if err != nil {
			return err
		}
		roles = string(b)
	}

	_, err := r.pool.Exec(ctx, `
		INSERT INTO users (id, username, email, password_hash, roles, active, organization_id, oidc_subject, created_at, updated_at)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)`,
		user.ID, user.Username, user.Email, user.PasswordHash,
		roles, user.Active, user.OrganizationID,
		nilIfEmpty(user.OIDCSubject),
		user.CreatedAt, user.UpdatedAt,
	)
	if err != nil {
		return fmt.Errorf("pg: create user: %w", err)
	}
	return nil
}

func (r *PgUserRepository) GetByID(ctx context.Context, id string) (*User, error) {
	return r.scanUser(ctx, "SELECT id, username, email, password_hash, roles, active, organization_id, COALESCE(oidc_subject,''), created_at, updated_at FROM users WHERE id = $1", id)
}

func (r *PgUserRepository) GetByUsername(ctx context.Context, username string) (*User, error) {
	return r.scanUser(ctx, "SELECT id, username, email, password_hash, roles, active, organization_id, COALESCE(oidc_subject,''), created_at, updated_at FROM users WHERE username = $1", username)
}

func (r *PgUserRepository) GetByEmail(ctx context.Context, email string) (*User, error) {
	return r.scanUser(ctx, "SELECT id, username, email, password_hash, roles, active, organization_id, COALESCE(oidc_subject,''), created_at, updated_at FROM users WHERE email = $1", email)
}

func (r *PgUserRepository) GetByOIDCSubject(ctx context.Context, sub string) (*User, error) {
	return r.scanUser(ctx, "SELECT id, username, email, password_hash, roles, active, organization_id, COALESCE(oidc_subject,''), created_at, updated_at FROM users WHERE oidc_subject = $1", sub)
}

func (r *PgUserRepository) ListUsers(ctx context.Context) ([]*User, error) {
	rows, err := r.pool.Query(ctx, "SELECT id, username, email, password_hash, roles, active, organization_id, COALESCE(oidc_subject,''), created_at, updated_at FROM users ORDER BY created_at")
	if err != nil {
		return nil, fmt.Errorf("pg: list users: %w", err)
	}
	defer rows.Close()

	var users []*User
	for rows.Next() {
		u := &User{}
		var roles any
		if _, isSQLite := r.pool.(*db.SqlitePool); isSQLite {
			var rolesJSON string
			roles = &rolesJSON
		} else {
			roles = &u.Roles
		}

		if err := rows.Scan(&u.ID, &u.Username, &u.Email, &u.PasswordHash, roles, &u.Active, &u.OrganizationID, &u.OIDCSubject, &u.CreatedAt, &u.UpdatedAt); err != nil {
			return nil, fmt.Errorf("pg: scan user: %w", err)
		}
		if ptr, ok := roles.(*string); ok {
			_ = json.Unmarshal([]byte(*ptr), &u.Roles)
		}
		users = append(users, u)
	}
	return users, nil
}

func (r *PgUserRepository) UpdateUser(ctx context.Context, user *User) error {
	var roles any = user.Roles
	if _, isSQLite := r.pool.(*db.SqlitePool); isSQLite {
		b, err := json.Marshal(user.Roles)
		if err != nil {
			return err
		}
		roles = string(b)
	}

	_, err := r.pool.Exec(ctx, `
		UPDATE users SET username=$2, email=$3, password_hash=$4, roles=$5, active=$6,
		organization_id=$7, oidc_subject=$8, updated_at=$9
		WHERE id=$1`,
		user.ID, user.Username, user.Email, user.PasswordHash,
		roles, user.Active, user.OrganizationID,
		nilIfEmpty(user.OIDCSubject), user.UpdatedAt,
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
	query := `
		INSERT INTO revoked_tokens (jti, expires_at) VALUES ($1, $2)
		ON CONFLICT (jti) DO NOTHING`
	if _, isSQLite := r.pool.(*db.SqlitePool); isSQLite {
		query = `INSERT OR IGNORE INTO revoked_tokens (jti, expires_at) VALUES ($1, $2)`
	}
	_, err := r.pool.Exec(ctx, query, jti, exp)
	if err != nil {
		return fmt.Errorf("pg: revoke token: %w", err)
	}

	nowSql := "NOW()"
	if _, isSQLite := r.pool.(*db.SqlitePool); isSQLite {
		nowSql = "CURRENT_TIMESTAMP"
	}
	_, _ = r.pool.Exec(ctx, "DELETE FROM revoked_tokens WHERE expires_at < " + nowSql)
	return nil
}

func (r *PgUserRepository) IsRevoked(ctx context.Context, jti string) (bool, error) {
	var count int
	nowSql := "NOW()"
	if _, isSQLite := r.pool.(*db.SqlitePool); isSQLite {
		nowSql = "CURRENT_TIMESTAMP"
	}
	err := r.pool.QueryRow(ctx, "SELECT COUNT(*) FROM revoked_tokens WHERE jti = $1 AND expires_at >= " + nowSql, jti).Scan(&count)
	if err != nil {
		return false, fmt.Errorf("pg: check revoked: %w", err)
	}
	return count > 0, nil
}

// --- helpers ---

func (r *PgUserRepository) scanUser(ctx context.Context, query string, args ...any) (*User, error) {
	u := &User{}
	var roles any
	if _, isSQLite := r.pool.(*db.SqlitePool); isSQLite {
		var rolesJSON string
		roles = &rolesJSON
	} else {
		roles = &u.Roles
	}

	err := r.pool.QueryRow(ctx, query, args...).Scan(
		&u.ID, &u.Username, &u.Email, &u.PasswordHash,
		roles, &u.Active, &u.OrganizationID, &u.OIDCSubject,
		&u.CreatedAt, &u.UpdatedAt,
	)
	if ptr, ok := roles.(*string); ok {
		_ = json.Unmarshal([]byte(*ptr), &u.Roles)
	}
	if err != nil {
		if strings.Contains(err.Error(), "no rows in result set") {
			return nil, ErrUserNotFound
		}
		return nil, fmt.Errorf("pg: scan user: %w", err)
	}
	return u, nil
}

func nilIfEmpty(s string) *string {
	if s == "" {
		return nil
	}
	return &s
}
