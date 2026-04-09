package main

import (
    "bytes"
    "encoding/json"
    "flag"
    "fmt"
    "io"
    "net/http"
    "os"
)

func main() {
    scenario := flag.String("scenario", "launch-readiness", "The mock data scenario to seed")
    flag.Parse()

    port := os.Getenv("PORT")
    if port == "" {
        port = "8080"
    }
    url := fmt.Sprintf("http://127.0.0.1:%s/api/dev/seed", port)

    payload := map[string]string{"scenario": *scenario}
    data, _ := json.Marshal(payload)

    resp, err := http.Post(url, "application/json", bytes.NewReader(data))
    if err != nil {
        fmt.Printf("Failed to connect to OHC Backend on port %s: %v\n", port, err)
        os.Exit(1)
    }
    defer resp.Body.Close()

    if resp.StatusCode == http.StatusOK {
        fmt.Println("✓ Mock Data seeded successfully!")
    } else {
        body, _ := io.ReadAll(resp.Body)
        fmt.Printf("✗ Failed to seed data. Server returned HTTP %d: %s\n", resp.StatusCode, string(body))
        os.Exit(1)
    }
}
