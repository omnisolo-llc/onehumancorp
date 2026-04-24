package auth

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"
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

func (r *PgUserRepository) CreateUser(ctx context.Context, user *User, orgID string) error {
	if orgID != "" {
		user.OrganizationID = orgID
	}

	rolesJSON, _ := json.Marshal(user.Roles)
	var rolesArg any = user.Roles
	if r.pool.IsSQLite() {
		rolesArg = string(rolesJSON)
	}

	email := user.Email
	oidcSubject := user.OIDCSubject

	if r.pool.IsSQLite() {
		email = EncryptDeterministic(email)
		if oidcSubject != "" {
			oidcSubject = EncryptDeterministic(oidcSubject)
		}
	}

	// We'll verify uniqueness at application level or let the new constraints handle it.
	// But let's first check if there's a collision in the same org.
	checkOrgID := orgID
	existing, _ := r.GetByUsername(ctx, user.Username, checkOrgID)
	if existing != nil {
		return fmt.Errorf("username already taken")
	}
	existingEmail, _ := r.GetByEmail(ctx, user.Email, checkOrgID)
	if existingEmail != nil {
		return fmt.Errorf("email already registered")
	}

	_, err := r.pool.Exec(ctx, `
		INSERT INTO users (id, username, email, password_hash, roles, active, organization_id, oidc_subject, created_at, updated_at)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)`,
		user.ID, user.Username, email, user.PasswordHash,
		rolesArg, user.Active, user.OrganizationID,
		nilIfEmpty(oidcSubject),
		user.CreatedAt, user.UpdatedAt,
	)
	if err != nil {
		return fmt.Errorf("pg: create user: %w", err)
	}
	return nil
}

func (r *PgUserRepository) GetByID(ctx context.Context, id string, orgID string) (*User, error) {
	return r.scanUser(ctx, "SELECT id, username, email, password_hash, roles, active, organization_id, COALESCE(oidc_subject,''), created_at, updated_at FROM users WHERE id = $1 AND organization_id = $2", id, orgID)
}

func (r *PgUserRepository) GetByUsername(ctx context.Context, username string, orgID string) (*User, error) {
	return r.scanUser(ctx, "SELECT id, username, email, password_hash, roles, active, organization_id, COALESCE(oidc_subject,''), created_at, updated_at FROM users WHERE username = $1 AND organization_id = $2", username, orgID)
}

func (r *PgUserRepository) GetByEmail(ctx context.Context, email string, orgID string) (*User, error) {
	lookupEmail := email
	if r.pool.IsSQLite() {
		lookupEmail = EncryptDeterministic(email)
	}
	return r.scanUser(ctx, "SELECT id, username, email, password_hash, roles, active, organization_id, COALESCE(oidc_subject,''), created_at, updated_at FROM users WHERE email = $1 AND organization_id = $2", lookupEmail, orgID)
}

func (r *PgUserRepository) GetByOIDCSubject(ctx context.Context, sub string, orgID string) (*User, error) {
	lookupSub := sub
	if r.pool.IsSQLite() {
		lookupSub = EncryptDeterministic(sub)
	}
	return r.scanUser(ctx, "SELECT id, username, email, password_hash, roles, active, organization_id, COALESCE(oidc_subject,''), created_at, updated_at FROM users WHERE oidc_subject = $1 AND organization_id = $2", lookupSub, orgID)
}

func (r *PgUserRepository) ListUsers(ctx context.Context, orgID string) ([]*User, error) {
	var rows db.Rows
	var err error
	rows, err = r.pool.Query(ctx, "SELECT id, username, email, password_hash, roles, active, organization_id, COALESCE(oidc_subject,''), created_at, updated_at FROM users WHERE organization_id = $1 ORDER BY created_at", orgID)
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
			u.Email = DecryptDeterministic(u.Email)
			if u.OIDCSubject != "" {
				u.OIDCSubject = DecryptDeterministic(u.OIDCSubject)
			}
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

func (r *PgUserRepository) UpdateUser(ctx context.Context, user *User, orgID string) error {
	if orgID != "" {
		user.OrganizationID = orgID
	}

	rolesJSON, _ := json.Marshal(user.Roles)
	var rolesArg any = user.Roles
	if r.pool.IsSQLite() {
		rolesArg = string(rolesJSON)
	}

	email := user.Email
	oidcSubject := user.OIDCSubject

	if r.pool.IsSQLite() {
		email = EncryptDeterministic(email)
		if oidcSubject != "" {
			oidcSubject = EncryptDeterministic(oidcSubject)
		}
	}

	checkOrgID := orgID
	existingEmail, _ := r.GetByEmail(ctx, user.Email, checkOrgID)
	if existingEmail != nil && existingEmail.ID != user.ID {
		return fmt.Errorf("email already registered")
	}

	var err error
	_, err = r.pool.Exec(ctx, `
		UPDATE users SET username=$2, email=$3, password_hash=$4, roles=$5, active=$6,
		organization_id=$7, oidc_subject=$8, updated_at=$9
		WHERE id=$1 AND organization_id=$10`,
		user.ID, user.Username, email, user.PasswordHash,
		rolesArg, user.Active, user.OrganizationID,
		nilIfEmpty(oidcSubject), user.UpdatedAt, orgID,
	)
	if err != nil {
		return fmt.Errorf("pg: update user: %w", err)
	}
	return nil
}

func (r *PgUserRepository) DeleteUser(ctx context.Context, id string, orgID string) error {
	var err error
	_, err = r.pool.Exec(ctx, "DELETE FROM users WHERE id = $1 AND organization_id = $2", id, orgID)
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
		u.CreatedAt = created.Time
		u.UpdatedAt = updated.Time
		u.Email = DecryptDeterministic(u.Email)
		if u.OIDCSubject != "" {
			u.OIDCSubject = DecryptDeterministic(u.OIDCSubject)
		}
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
