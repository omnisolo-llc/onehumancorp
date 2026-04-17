package main

import (
	"fmt"
	"github.com/onehumancorp/mono/services/wizard"
	"net/http"
)

func main() {
	http.HandleFunc("/api/wizard/configure", wizard.HandleConfigWizard)
	http.HandleFunc("/api/wizard/prompt_tuning", wizard.HandlePromptTuning)
	http.HandleFunc("/api/wizard/fix", wizard.HandleFixAgent)
	http.HandleFunc("/api/wizard/upgrade", wizard.HandleUpgrade)
	http.HandleFunc("/api/wizard/rollback", wizard.HandleRollback)
	fmt.Println("Wizard backend running on :8080")
	http.ListenAndServe(":8080", nil)
}
