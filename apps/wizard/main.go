package main

import (
	"fmt"
	"net/http"
	"github.com/onehumancorp/mono/services/wizard"
)

func main() {
    http.HandleFunc("/api/wizard/configure", wizard.HandleConfigWizard)
    fmt.Println("Wizard backend running on :8080")
    http.ListenAndServe(":8080", nil)
}
