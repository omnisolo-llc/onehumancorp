package main

import (
	"context"
	"database/sql"
	"fmt"
	"log"
	"math/rand"
	"time"

	_ "modernc.org/sqlite"

	"github.com/onehumancorp/mono/services/growth/referrals"
)

func main() {
	db, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		log.Fatalf("failed to open db: %v", err)
	}
	defer db.Close()

	_, err = db.Exec("CREATE TABLE referrals (tenant_id TEXT, code TEXT, user_id TEXT, usages INTEGER, PRIMARY KEY(tenant_id, code))")
	if err != nil {
		log.Fatalf("failed to create table: %v", err)
	}

	rs, err := referrals.NewReferralSystem(db)
	if err != nil {
		log.Fatalf("failed to create referral system: %v", err)
	}

	ctx := context.Background()
	tenantID := "tenant_growth_audit"

	fmt.Println("Starting Growth Audit Simulation...")

	r := rand.New(rand.NewSource(time.Now().UnixNano()))

	// Simulate 100 initial users
	var codes []string
	for i := 0; i < 100; i++ {
		userID := fmt.Sprintf("user_%d", i)
		code, _ := rs.GenerateCode(ctx, tenantID, userID)
		codes = append(codes, code)
	}

	fmt.Printf("100 users generated referral codes.\n")

	// Simulate viral loop usages
	usages := 0
	for _, code := range codes {
		// Random chance to have 0 to 3 usages
		count := r.Intn(4)
		for j := 0; j < count; j++ {
			_, err := rs.UseCode(ctx, tenantID, code)
			if err == nil {
				usages++
			}
		}
	}

	fmt.Printf("Simulated %d total usages.\n", usages)

	k, err := rs.GetViralCoefficient(ctx, tenantID)
	if err != nil {
		log.Fatalf("failed to compute k-factor: %v", err)
	}

	fmt.Printf("=> Computed Viral Coefficient (K-factor): %.2f\n", k)
}
