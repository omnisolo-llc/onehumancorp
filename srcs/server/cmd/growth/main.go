package main

import (
"context"
"encoding/json"
"fmt"
"log"
"net/http"
"os"

"github.com/onehumancorp/mono/srcs/server/lib/analytics"
"github.com/onehumancorp/mono/srcs/server/services/growth"
)

type ReferralRequest struct {
SenderID      string `json:"sender_id"`
ReceiverEmail string `json:"receiver_email"`
}

type QuotaRequest struct {
TenantID string `json:"tenant_id"`
}

func authMiddleware(next http.HandlerFunc) http.HandlerFunc {
return func(w http.ResponseWriter, r *http.Request) {
// SPIFFE/SPIRE check.
// Note: The actual deployment environment uses a sidecar proxy (like Envoy or internal K8s ingress)
// that terminates the mTLS SPIFFE connection and forwards the validated SPIFFE ID via a trusted header
// (e.g., X-Forwarded-Client-Cert or X-Spiffe-Id).
// We validate the presence of this trusted header instead of raw TLS termination here,
// allowing the Go service to run as a standard HTTP listener internally.
spiffeID := r.Header.Get("X-Spiffe-Id")
if spiffeID == "" && r.TLS == nil {
http.Error(w, "missing SPIFFE identity", http.StatusUnauthorized)
return
}

next(w, r)
}
}

func NewGrowthMux(_ *analytics.Tracker) *http.ServeMux {
mux := http.NewServeMux()

referralTracker := growth.NewReferralTracker()
quotaTracker := growth.NewQuotaTracker(100, 50)

mux.HandleFunc("/growth/referral", authMiddleware(func(w http.ResponseWriter, r *http.Request) {
if r.Method != http.MethodPost {
http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
return
}
var req ReferralRequest
if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
http.Error(w, "Invalid request body", http.StatusBadRequest)
return
}

if req.SenderID == "" || req.ReceiverEmail == "" {
http.Error(w, "missing sender_id or receiver_email", http.StatusBadRequest)
return
}

// Generate a referral code for the sender and record the invite
code := referralTracker.GenerateReferralCode(req.SenderID)
referralTracker.RecordReferral(context.Background(), code)
w.WriteHeader(http.StatusOK)
fmt.Fprintf(w, "Invite sent successfully\n")
}))

mux.HandleFunc("/growth/quota/check", authMiddleware(func(w http.ResponseWriter, r *http.Request) {
if r.Method != http.MethodPost {
http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
return
}
var req QuotaRequest
if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
http.Error(w, "Invalid request body", http.StatusBadRequest)
return
}
if req.TenantID == "" {
http.Error(w, "missing tenant_id", http.StatusBadRequest)
return
}
used := referralTracker.GetTotalReferrals()
quota := quotaTracker.CalculateQuota(used)
w.Header().Set("Content-Type", "application/json")
json.NewEncoder(w).Encode(map[string]int{"quota": quota, "used": used})
}))

mux.HandleFunc("/growth/quota/increment", authMiddleware(func(w http.ResponseWriter, r *http.Request) {
if r.Method != http.MethodPost {
http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
return
}
var req QuotaRequest
if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
http.Error(w, "Invalid request body", http.StatusBadRequest)
return
}
if req.TenantID == "" {
http.Error(w, "missing tenant_id", http.StatusBadRequest)
return
}
referralTracker.RecordReferral(context.Background(), req.TenantID)
w.WriteHeader(http.StatusOK)
fmt.Fprintf(w, "Quota incremented\n")
}))

return mux
}

func main() {
tracker := analytics.NewTracker()
mux := NewGrowthMux(tracker)

port := os.Getenv("PORT")
if port == "" {
port = "8081"
}

log.Printf("Starting growth service on :%s", port)
if err := http.ListenAndServe(":"+port, mux); err != nil {
log.Fatalf("Failed to start server: %v", err)
}
}
