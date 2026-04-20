package main

import (
	"fmt"
	"github.com/onehumancorp/mono/srcs/server/services/wizard"
	"net/http"
)

func main() {
	http.HandleFunc("/api/wizard/configure", wizard.HandleConfigWizard)
	http.HandleFunc("/api/wizard/prompt_tuning", wizard.HandlePromptTuning)
	fmt.Println("Wizard backend running on :8080")
	http.ListenAndServe(":8080", nil)
}
