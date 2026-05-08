package builder

import (
	"context"
	"encoding/json"
	"net/http"

	"onehumancorp/srcs/server/memory"
)

type contextKey string

const tenantContextKey contextKey = "tenant_id"

type APIHandler struct {
	store BuilderStore
	llmClient memory.LLMClient
}

func NewAPIHandler(store BuilderStore, llmClient memory.LLMClient) *APIHandler {
	return &APIHandler{store: store, llmClient: llmClient}
}

// TenantAuthMiddleware extracts the X-Tenant-Id header and injects it into the request context.
func TenantAuthMiddleware(next http.HandlerFunc) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		tenantID := r.Header.Get("X-Tenant-Id")
		if tenantID == "" {
			http.Error(w, "Missing X-Tenant-Id header", http.StatusUnauthorized)
			return
		}
		// Inject into context
		ctx := context.WithValue(r.Context(), tenantContextKey, tenantID)
		next.ServeHTTP(w, r.WithContext(ctx))
	}
}

func (h *APIHandler) HandleCreateSite(w http.ResponseWriter, r *http.Request) {
	tenantID, ok := r.Context().Value(tenantContextKey).(string)
	if !ok || tenantID == "" {
		http.Error(w, "Unauthorized", http.StatusUnauthorized)
		return
	}

	var site Site
	if err := json.NewDecoder(r.Body).Decode(&site); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	site.TenantID = tenantID // Ensure tenant ID from context is used

	if err := h.store.CreateSite(r.Context(), &site); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(site)
}

func (h *APIHandler) HandleGetSite(w http.ResponseWriter, r *http.Request) {
	tenantID, ok := r.Context().Value(tenantContextKey).(string)
	if !ok || tenantID == "" {
		http.Error(w, "Unauthorized", http.StatusUnauthorized)
		return
	}

	id := r.URL.Query().Get("id")
	site, err := h.store.GetSite(r.Context(), id)
	if err != nil {
		http.Error(w, err.Error(), http.StatusNotFound)
		return
	}

	// Basic BOLA/IDOR protection
	if site.TenantID != tenantID {
		http.Error(w, "Unauthorized access to site", http.StatusForbidden)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(site)
}

func (h *APIHandler) HandleCreatePage(w http.ResponseWriter, r *http.Request) {
	tenantID, ok := r.Context().Value(tenantContextKey).(string)
	if !ok || tenantID == "" {
		http.Error(w, "Unauthorized", http.StatusUnauthorized)
		return
	}

	var page Page
	if err := json.NewDecoder(r.Body).Decode(&page); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	// Validate tenant owns the site before allowing page creation
	site, err := h.store.GetSite(r.Context(), page.SiteID)
	if err != nil || site.TenantID != tenantID {
		http.Error(w, "Unauthorized access to parent site", http.StatusForbidden)
		return
	}

	page.TenantID = tenantID

	if err := h.store.CreatePage(r.Context(), &page); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(page)
}

func (h *APIHandler) HandleGetPage(w http.ResponseWriter, r *http.Request) {
	tenantID, ok := r.Context().Value(tenantContextKey).(string)
	if !ok || tenantID == "" {
		http.Error(w, "Unauthorized", http.StatusUnauthorized)
		return
	}

	id := r.URL.Query().Get("id")
	page, err := h.store.GetPage(r.Context(), id)
	if err != nil {
		http.Error(w, err.Error(), http.StatusNotFound)
		return
	}

	if page.TenantID != tenantID {
		http.Error(w, "Unauthorized access to page", http.StatusForbidden)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(page)
}

func (h *APIHandler) HandleGetBlocks(w http.ResponseWriter, r *http.Request) {
	tenantID, ok := r.Context().Value(tenantContextKey).(string)
	if !ok || tenantID == "" {
		http.Error(w, "Unauthorized", http.StatusUnauthorized)
		return
	}

	pageID := r.URL.Query().Get("page_id")

	// Validate page ownership
	page, err := h.store.GetPage(r.Context(), pageID)
	if err != nil || page.TenantID != tenantID {
		http.Error(w, "Unauthorized access to parent page", http.StatusForbidden)
		return
	}

	blocks, err := h.store.GetBlocks(r.Context(), pageID)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(blocks)
}

func (h *APIHandler) HandleCreateBlock(w http.ResponseWriter, r *http.Request) {
	tenantID, ok := r.Context().Value(tenantContextKey).(string)
	if !ok || tenantID == "" {
		http.Error(w, "Unauthorized", http.StatusUnauthorized)
		return
	}

	var block Block
	if err := json.NewDecoder(r.Body).Decode(&block); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	// Validate page ownership
	page, err := h.store.GetPage(r.Context(), block.PageID)
	if err != nil || page.TenantID != tenantID {
		http.Error(w, "Unauthorized access to parent page", http.StatusForbidden)
		return
	}

	block.TenantID = tenantID

	if err := ValidateBlockPayload(&block); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	if err := h.store.CreateBlock(r.Context(), &block); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(block)
}

func (h *APIHandler) HandleUpdateBlock(w http.ResponseWriter, r *http.Request) {
	tenantID, ok := r.Context().Value(tenantContextKey).(string)
	if !ok || tenantID == "" {
		http.Error(w, "Unauthorized", http.StatusUnauthorized)
		return
	}

	var block Block
	if err := json.NewDecoder(r.Body).Decode(&block); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	// Validate page ownership (to ensure block belongs to tenant)
	page, err := h.store.GetPage(r.Context(), block.PageID)
	if err != nil || page.TenantID != tenantID {
		http.Error(w, "Unauthorized access to parent page", http.StatusForbidden)
		return
	}

	if err := ValidateBlockPayload(&block); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	if err := h.store.UpdateBlock(r.Context(), &block); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(block)
}

func (h *APIHandler) HandleReorderBlocks(w http.ResponseWriter, r *http.Request) {
	tenantID, ok := r.Context().Value(tenantContextKey).(string)
	if !ok || tenantID == "" {
		http.Error(w, "Unauthorized", http.StatusUnauthorized)
		return
	}

	var req struct {
		PageID   string   `json:"page_id"`
		BlockIDs []string `json:"block_ids"`
	}
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	// Validate page ownership
	page, err := h.store.GetPage(r.Context(), req.PageID)
	if err != nil || page.TenantID != tenantID {
		http.Error(w, "Unauthorized access to parent page", http.StatusForbidden)
		return
	}

	if err := h.store.ReorderBlocks(r.Context(), req.PageID, req.BlockIDs); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
}

func (h *APIHandler) HandlePublishSite(w http.ResponseWriter, r *http.Request) {
	tenantID, ok := r.Context().Value(tenantContextKey).(string)
	if !ok || tenantID == "" {
		http.Error(w, "Unauthorized", http.StatusUnauthorized)
		return
	}

	var req struct {
		SiteID string `json:"site_id"`
	}
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	site, err := h.store.GetSite(r.Context(), req.SiteID)
	if err != nil {
		http.Error(w, err.Error(), http.StatusNotFound)
		return
	}

	if site.TenantID != tenantID {
		http.Error(w, "Unauthorized access to site", http.StatusForbidden)
		return
	}

	// Trigger async publish
	go PublishSiteAsync(context.Background(), h.store, site, h.llmClient)

	w.WriteHeader(http.StatusAccepted)
	w.Write([]byte(`{"status":"publishing"}`))
}
