package auth

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"
	"time"

	"google.golang.org/protobuf/types/known/timestamppb"

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
	rolesJSON, _ := json.Marshal(user.Metadata.Roles)
	var rolesArg any = user.Metadata.Roles
	if r.pool.IsSQLite() {
		rolesArg = string(rolesJSON)
	}

	email := user.Metadata.Email
	oidcSubject := user.Metadata.OidcSubject

	if r.pool.IsSQLite() {
		email = EncryptDeterministic(email)
		if oidcSubject != "" {
			oidcSubject = EncryptDeterministic(oidcSubject)
		}
	}

	// We'll verify uniqueness at application level or let the new constraints handle it.
	// But let's first check if there's a collision in the same org.
	existing, _ := r.GetByUsername(ctx, user.Metadata.Username, user.Metadata.OrganizationId)
	if existing != nil {
		return fmt.Errorf("username already taken")
	}
	existingEmail, _ := r.GetByEmail(ctx, user.Metadata.Email, user.Metadata.OrganizationId)
	if existingEmail != nil {
		return fmt.Errorf("email already registered")
	}

	_, err := r.pool.Exec(ctx, `
		INSERT INTO users (id, username, email, password_hash, roles, active, organization_id, oidc_subject, created_at, updated_at)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)`,
		user.Metadata.Id, user.Metadata.Username, email, user.PasswordHash,
		rolesArg, user.Metadata.Active, user.Metadata.OrganizationId,
		nilIfEmpty(oidcSubject),
		user.Metadata.CreatedAt.AsTime(), user.Metadata.UpdatedAt.AsTime(),
	)
	if err != nil {
		return fmt.Errorf("pg: create user: %w", err)
	}
	return nil
}

func (r *PgUserRepository) GetByID(ctx context.Context, id string, orgID string) (*User, error) {
	if orgID == "" || orgID == "sys" {
		return r.scanUser(ctx, "SELECT id, username, email, password_hash, roles, active, organization_id, COALESCE(oidc_subject,''), created_at, updated_at FROM users WHERE id = $1", id)
	}
	return r.scanUser(ctx, "SELECT id, username, email, password_hash, roles, active, organization_id, COALESCE(oidc_subject,''), created_at, updated_at FROM users WHERE id = $1 AND organization_id = $2", id, orgID)
}

func (r *PgUserRepository) GetByUsername(ctx context.Context, username string, orgID string) (*User, error) {
	if orgID == "" || orgID == "sys" {
		return r.scanUser(ctx, "SELECT id, username, email, password_hash, roles, active, organization_id, COALESCE(oidc_subject,''), created_at, updated_at FROM users WHERE username = $1", username)
	}
	return r.scanUser(ctx, "SELECT id, username, email, password_hash, roles, active, organization_id, COALESCE(oidc_subject,''), created_at, updated_at FROM users WHERE username = $1 AND organization_id = $2", username, orgID)
}

func (r *PgUserRepository) GetByEmail(ctx context.Context, email string, orgID string) (*User, error) {
	lookupEmail := email
	if r.pool.IsSQLite() {
		lookupEmail = EncryptDeterministic(email)
	}
	if orgID == "" || orgID == "sys" {
		return r.scanUser(ctx, "SELECT id, username, email, password_hash, roles, active, organization_id, COALESCE(oidc_subject,''), created_at, updated_at FROM users WHERE email = $1", lookupEmail)
	}
	return r.scanUser(ctx, "SELECT id, username, email, password_hash, roles, active, organization_id, COALESCE(oidc_subject,''), created_at, updated_at FROM users WHERE email = $1 AND organization_id = $2", lookupEmail, orgID)
}

func (r *PgUserRepository) GetByOIDCSubject(ctx context.Context, sub string, orgID string) (*User, error) {
	lookupSub := sub
	if r.pool.IsSQLite() {
		lookupSub = EncryptDeterministic(sub)
	}
	if orgID == "" || orgID == "sys" {
		return r.scanUser(ctx, "SELECT id, username, email, password_hash, roles, active, organization_id, COALESCE(oidc_subject,''), created_at, updated_at FROM users WHERE oidc_subject = $1", lookupSub)
	}
	return r.scanUser(ctx, "SELECT id, username, email, password_hash, roles, active, organization_id, COALESCE(oidc_subject,''), created_at, updated_at FROM users WHERE oidc_subject = $1 AND organization_id = $2", lookupSub, orgID)
}

func (r *PgUserRepository) ListUsers(ctx context.Context, orgID string) ([]*User, error) {
	var rows db.Rows
	var err error
	if orgID == "" || orgID == "sys" {
		rows, err = r.pool.Query(ctx, "SELECT id, username, email, password_hash, roles, active, organization_id, COALESCE(oidc_subject,''), created_at, updated_at FROM users ORDER BY created_at")
	} else {
		rows, err = r.pool.Query(ctx, "SELECT id, username, email, password_hash, roles, active, organization_id, COALESCE(oidc_subject,''), created_at, updated_at FROM users WHERE organization_id = $1 ORDER BY created_at", orgID)
	}
	if err != nil {
		return nil, fmt.Errorf("pg: list users: %w", err)
	}
	defer rows.Close()

	var users []*User
	for rows.Next() {
		u := &User{Metadata: &UserMetadata{}}
		var created, updated db.FlexTime

		if r.pool.IsSQLite() {
			var rolesJSON string
			if err := rows.Scan(&u.Metadata.Id, &u.Metadata.Username, &u.Metadata.Email, &u.PasswordHash, &rolesJSON, &u.Metadata.Active, &u.Metadata.OrganizationId, &u.Metadata.OidcSubject, &created, &updated); err != nil {
				return nil, fmt.Errorf("pg: scan user: %w", err)
			}
			_ = json.Unmarshal([]byte(rolesJSON), &u.Metadata.Roles)
			u.Metadata.Email = DecryptDeterministic(u.Metadata.Email)
			if u.Metadata.OidcSubject != "" {
				u.Metadata.OidcSubject = DecryptDeterministic(u.Metadata.OidcSubject)
			}
		} else {
			if err := rows.Scan(&u.Metadata.Id, &u.Metadata.Username, &u.Metadata.Email, &u.PasswordHash, &u.Metadata.Roles, &u.Metadata.Active, &u.Metadata.OrganizationId, &u.Metadata.OidcSubject, &created, &updated); err != nil {
				return nil, fmt.Errorf("pg: scan user: %w", err)
			}
		}
		u.Metadata.CreatedAt = timestamppb.New(created.Time)
		u.Metadata.UpdatedAt = timestamppb.New(updated.Time)
		users = append(users, u)
	}
	return users, nil
}

func (r *PgUserRepository) UpdateUser(ctx context.Context, user *User) error {
	rolesJSON, _ := json.Marshal(user.Metadata.Roles)
	var rolesArg any = user.Metadata.Roles
	if r.pool.IsSQLite() {
		rolesArg = string(rolesJSON)
	}

	email := user.Metadata.Email
	oidcSubject := user.Metadata.OidcSubject

	if r.pool.IsSQLite() {
		email = EncryptDeterministic(email)
		if oidcSubject != "" {
			oidcSubject = EncryptDeterministic(oidcSubject)
		}
	}

	existingEmail, _ := r.GetByEmail(ctx, user.Metadata.Email, user.Metadata.OrganizationId)
	if existingEmail != nil && existingEmail.Metadata.Id != user.Metadata.Id {
		return fmt.Errorf("email already registered")
	}

	var err error
	if user.Metadata.OrganizationId == "" || user.Metadata.OrganizationId == "sys" {
		_, err = r.pool.Exec(ctx, `
			UPDATE users SET username=$2, email=$3, password_hash=$4, roles=$5, active=$6,
			organization_id=$7, oidc_subject=$8, updated_at=$9
			WHERE id=$1`,
			user.Metadata.Id, user.Metadata.Username, email, user.PasswordHash,
			rolesArg, user.Metadata.Active, user.Metadata.OrganizationId,
			nilIfEmpty(oidcSubject), user.Metadata.UpdatedAt.AsTime(),
		)
	} else {
		_, err = r.pool.Exec(ctx, `
			UPDATE users SET username=$2, email=$3, password_hash=$4, roles=$5, active=$6,
			organization_id=$7, oidc_subject=$8, updated_at=$9
			WHERE id=$1 AND organization_id=$7`,
			user.Metadata.Id, user.Metadata.Username, email, user.PasswordHash,
			rolesArg, user.Metadata.Active, user.Metadata.OrganizationId,
			nilIfEmpty(oidcSubject), user.Metadata.UpdatedAt.AsTime(),
		)
	}
	if err != nil {
		return fmt.Errorf("pg: update user: %w", err)
	}
	return nil
}

func (r *PgUserRepository) DeleteUser(ctx context.Context, id string, orgID string) error {
	var err error
	if orgID == "" || orgID == "sys" {
		_, err = r.pool.Exec(ctx, "DELETE FROM users WHERE id = $1", id)
	} else {
		_, err = r.pool.Exec(ctx, "DELETE FROM users WHERE id = $1 AND organization_id = $2", id, orgID)
	}
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
	u := &User{Metadata: &UserMetadata{}}
	var created, updated db.FlexTime
 
	if r.pool.IsSQLite() {
		var rolesJSON string
		err := r.pool.QueryRow(ctx, query, args...).Scan(
			&u.Metadata.Id, &u.Metadata.Username, &u.Metadata.Email, &u.PasswordHash,
			&rolesJSON, &u.Metadata.Active, &u.Metadata.OrganizationId, &u.Metadata.OidcSubject,
			&created, &updated,
		)
		if err != nil {
			if strings.Contains(err.Error(), "no rows in result set") || strings.Contains(err.Error(), "sql: no rows in result set") {
				return nil, ErrUserNotFound
			}
			return nil, fmt.Errorf("pg: scan user: %w", err)
		}
		_ = json.Unmarshal([]byte(rolesJSON), &u.Metadata.Roles)
		u.Metadata.CreatedAt = timestamppb.New(created.Time)
		u.Metadata.UpdatedAt = timestamppb.New(updated.Time)
		u.Metadata.Email = DecryptDeterministic(u.Metadata.Email)
		if u.Metadata.OidcSubject != "" {
			u.Metadata.OidcSubject = DecryptDeterministic(u.Metadata.OidcSubject)
		}
		return u, nil
	}
 
	err := r.pool.QueryRow(ctx, query, args...).Scan(
		&u.Metadata.Id, &u.Metadata.Username, &u.Metadata.Email, &u.PasswordHash,
		&u.Metadata.Roles, &u.Metadata.Active, &u.Metadata.OrganizationId, &u.Metadata.OidcSubject,
		&created, &updated,
	)
	if err != nil {
		if strings.Contains(err.Error(), "no rows in result set") {
			return nil, ErrUserNotFound
		}
		return nil, fmt.Errorf("pg: scan user: %w", err)
	}
	u.Metadata.CreatedAt = timestamppb.New(created.Time)
	u.Metadata.UpdatedAt = timestamppb.New(updated.Time)
	return u, nil
}

func nilIfEmpty(s string) *string {
	if s == "" {
		return nil
	}
	return &s
}
