package onboarding

import (
	"context"
	"crypto/hmac"
	"crypto/sha256"
	"encoding/base64"
	"encoding/json"
	"net/http"
	"os"
	"strings"
)

type contextKey string
const tenantContextKey contextKey = "tenant_id"

type APIHandler struct {
	service *Service
}

func NewAPIHandler(service *Service) *APIHandler {
	return &APIHandler{service: service}
}

func (h *APIHandler) HandleStartOnboarding(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var req OnboardingRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "Invalid request body", http.StatusBadRequest)
		return
	}

	res, err := h.service.StartOnboarding(r.Context(), req)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusAccepted)
	json.NewEncoder(w).Encode(res)
}

func (h *APIHandler) HandleGetStatus(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	// Multi-Tenant Safety Check: Read tenant_id from session/context, not from headers/body/query
	tenantID, ok := r.Context().Value(tenantContextKey).(string)
	if !ok || tenantID == "" {
		http.Error(w, "Unauthorized: missing or invalid tenant session", http.StatusUnauthorized)
		return
	}

	res, err := h.service.GetOnboardingStatus(r.Context(), tenantID)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(res)
}

func (h *APIHandler) HandleSaveState(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	tenantID, ok := r.Context().Value(tenantContextKey).(string)
	if !ok || tenantID == "" {
		http.Error(w, "Unauthorized: missing or invalid tenant session", http.StatusUnauthorized)
		return
	}

	var req TenantStateRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "Invalid request body", http.StatusBadRequest)
		return
	}

	if err := h.service.SaveTenantState(r.Context(), tenantID, req.State); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusNoContent)
}

func (h *APIHandler) HandleGetState(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	tenantID, ok := r.Context().Value(tenantContextKey).(string)
	if !ok || tenantID == "" {
		http.Error(w, "Unauthorized: missing or invalid tenant session", http.StatusUnauthorized)
		return
	}

	res, err := h.service.GetTenantState(r.Context(), tenantID)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(res)
}

// TenantAuthMiddleware extracts the organization_id from the JWT in the Authorization header.
func TenantAuthMiddleware(next http.HandlerFunc) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		authHeader := r.Header.Get("Authorization")
		if authHeader == "" || !strings.HasPrefix(authHeader, "Bearer ") {
			http.Error(w, "Unauthorized: missing or invalid token", http.StatusUnauthorized)
			return
		}

		tokenString := strings.TrimPrefix(authHeader, "Bearer ")
		parts := strings.Split(tokenString, ".")
		if len(parts) != 3 {
			http.Error(w, "Unauthorized: invalid token format", http.StatusUnauthorized)
			return
		}

		// Validate JWT Signature securely
		secret := os.Getenv("JWT_SECRET")
		if secret == "" {
			if os.Getenv("OHC_STANDALONE") == "true" {
				// Standalone fallback: derive secret from OHC_SQLITE_KEY like Rust backend
				sqliteKey := os.Getenv("OHC_SQLITE_KEY")
				if sqliteKey != "" {
					mac := hmac.New(sha256.New, []byte("ohc_jwt_derivation_salt"))
					mac.Write([]byte(sqliteKey))
					secret = string(mac.Sum(nil))
				}
			}
		}

		// If we still have no secret, we must reject in cloud mode to prevent forged tokens
		if secret == "" && os.Getenv("OHC_STANDALONE") != "true" {
			http.Error(w, "Internal Server Error: Missing JWT_SECRET", http.StatusInternalServerError)
			return
		}

		if secret != "" {
			mac := hmac.New(sha256.New, []byte(secret))
			mac.Write([]byte(parts[0] + "." + parts[1]))
			expectedSignature := base64.RawURLEncoding.EncodeToString(mac.Sum(nil))

			if !hmac.Equal([]byte(parts[2]), []byte(expectedSignature)) {
				http.Error(w, "Unauthorized: invalid token signature", http.StatusUnauthorized)
				return
			}
		}

		payload, err := base64.RawURLEncoding.DecodeString(parts[1])
		if err != nil {
			http.Error(w, "Unauthorized: malformed token payload", http.StatusUnauthorized)
			return
		}

		var claims struct {
			OrganizationID string `json:"organization_id"`
		}
		if err := json.Unmarshal(payload, &claims); err != nil || claims.OrganizationID == "" {
			http.Error(w, "Unauthorized: missing organization_id in token", http.StatusUnauthorized)
			return
		}

		// Inject into context
		ctx := context.WithValue(r.Context(), tenantContextKey, claims.OrganizationID)
		next.ServeHTTP(w, r.WithContext(ctx))
	}
}
