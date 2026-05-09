package onboarding

import (
	"encoding/json"
	"context"
	"net/http"
	"strings"
	"fmt"
	"os"

	"github.com/golang-jwt/jwt/v5"
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

// TenantAuthMiddleware validates a JWT/Bearer token to securely extract tenant_id to prevent multi-tenant spoofing vulnerabilities (IDOR).
func TenantAuthMiddleware(next http.HandlerFunc) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		authHeader := r.Header.Get("Authorization")
		if authHeader == "" || !strings.HasPrefix(authHeader, "Bearer ") {
			http.Error(w, "Missing or invalid Authorization header", http.StatusUnauthorized)
			return
		}

		tokenStr := strings.TrimPrefix(authHeader, "Bearer ")
		tenantID, err := validateTokenAndGetTenant(tokenStr)
		if err != nil || tenantID == "" {
			http.Error(w, "Invalid token or missing tenant claim", http.StatusUnauthorized)
			return
		}

		ctx := context.WithValue(r.Context(), tenantContextKey, tenantID)
		next.ServeHTTP(w, r.WithContext(ctx))
	}
}

func getJWTSecret() ([]byte, error) {
	secret := os.Getenv("OHC_JWT_SECRET")
	if secret == "" {
		return nil, fmt.Errorf("OHC_JWT_SECRET environment variable is not set")
	}
	return []byte(secret), nil
}

func validateTokenAndGetTenant(tokenStr string) (string, error) {
	secret, err := getJWTSecret()
	if err != nil {
		return "", err
	}

	token, err := jwt.Parse(tokenStr, func(token *jwt.Token) (interface{}, error) {
		if _, ok := token.Method.(*jwt.SigningMethodHMAC); !ok {
			return nil, fmt.Errorf("Unexpected signing method: %v", token.Header["alg"])
		}
		return secret, nil
	})

	if err != nil {
		return "", err
	}

	if claims, ok := token.Claims.(jwt.MapClaims); ok && token.Valid {
		if tenantID, ok := claims["tenant_id"].(string); ok {
			return tenantID, nil
		}
	}
	return "", fmt.Errorf("tenant claim missing or token invalid")
}
