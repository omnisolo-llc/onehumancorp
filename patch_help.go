package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	content, err := os.ReadFile("src/tests/e2e/help_center_test.go")
	if err != nil {
		fmt.Printf("Error: %v\n", err)
		return
	}

	s := string(content)

	oldStr := `	// Verify AI Help Chat button is visible
	chatButton := page.Locator("[key='ai_help_chat_button']")
	require.NoError(t, chatButton.WaitFor(playwright.LocatorWaitForOptions{
		State: playwright.WaitForSelectorStateVisible,
	}))

	// Click AI Help Chat button to open chat
	require.NoError(t, chatButton.Click())

	// Verify chat interface opens
	require.NoError(t, page.Locator("text=Ask OHC Help").WaitFor(playwright.LocatorWaitForOptions{Timeout: playwright.Float(10000)}))`

	newStr := `	// Dismiss "A new version is available!" if present
	updateToast := page.Locator("text=A new version is available!")
	if count, _ := updateToast.Count(); count > 0 {
		updateToast.Locator("button").Click()
	}

	// Verify AI Help Chat button is visible
	chatButton := page.Locator("[key='ai_help_chat_button']")
	require.NoError(t, chatButton.WaitFor(playwright.LocatorWaitForOptions{
		State: playwright.WaitForSelectorStateVisible,
	}))

	// Click AI Help Chat button to open chat
	require.NoError(t, chatButton.Click())

	// Verify chat interface opens
	require.NoError(t, page.Locator("text=Ask OHC Help").WaitFor(playwright.LocatorWaitForOptions{Timeout: playwright.Float(10000)}))`

	s = strings.Replace(s, oldStr, newStr, 1)

	err = os.WriteFile("src/tests/e2e/help_center_test.go", []byte(s), 0644)
	if err != nil {
		fmt.Printf("Error: %v\n", err)
	}
}
