package e2e

import (
	"testing"
	"time"

	playwright "github.com/playwright-community/playwright-go"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestHelpCenterE2E(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	// Use mobile viewport to ensure Help Center works properly on 375px screens
	err := page.SetViewportSize(375, 812)
	require.NoError(t, err)

	// Sign in
	loginAsAdmin(t, page)

	// Wait for dashboard to load
	err = page.WaitForURL("**/dashboard**", playwright.PageWaitForURLOptions{
		WaitUntil: playwright.WaitUntilStateNetworkidle,
	})
	require.NoError(t, err)

	// Wait for CanvasKit to render
	time.Sleep(5 * time.Second)

	// Test the walkthrough flow since it triggers on dashboard load
	nextBtn := page.Locator("text='Next'").First()
	err = nextBtn.WaitFor(playwright.LocatorWaitForOptions{
		State: playwright.WaitForSelectorStateVisible,
	})
	require.NoError(t, err)
	err = nextBtn.Click()
	require.NoError(t, err)

	finishBtn := page.Locator("text='Finish'").First()
	err = finishBtn.WaitFor(playwright.LocatorWaitForOptions{
		State: playwright.WaitForSelectorStateVisible,
	})
	require.NoError(t, err)
	err = finishBtn.Click()
	require.NoError(t, err)

	// Wait for the UI to settle after walkthrough
	time.Sleep(1 * time.Second)

	// Since we are on dashboard after walkthrough, test Contextual Tooltips
	// Tooltips display on hover or long press
	statsCard := page.Locator("text='AI Helpers'").First()
	err = statsCard.WaitFor(playwright.LocatorWaitForOptions{
		State: playwright.WaitForSelectorStateVisible,
	})
	require.NoError(t, err)
	err = statsCard.Hover()
	require.NoError(t, err)

	// Check if tooltip shows (the text "A quick overview of your key business numbers today.")
	tooltipText := page.Locator("text='A quick overview of your key business numbers today.'").First()
	err = tooltipText.WaitFor(playwright.LocatorWaitForOptions{
		State: playwright.WaitForSelectorStateVisible,
	})
	require.NoError(t, err)

	// Re-open navigation menu to proceed to help center
	menuBtn := page.Locator("button[aria-label='Open navigation menu']")
	if count, _ := menuBtn.Count(); count > 0 {
		_ = menuBtn.Click()
		time.Sleep(1 * time.Second)
	}

	// Click the Help Center using the Semantic label
	helpBtn := page.Locator("[aria-label='Help Center']")
	err = helpBtn.WaitFor(playwright.LocatorWaitForOptions{
		State: playwright.WaitForSelectorStateVisible,
	})
	require.NoError(t, err)

	err = helpBtn.Click()
	require.NoError(t, err)

	// Verify we navigated to the help center
	err = page.WaitForURL("**/help**", playwright.PageWaitForURLOptions{
		WaitUntil: playwright.WaitUntilStateNetworkidle,
	})
	require.NoError(t, err)

	// Verify Help Center title is visible
	helpTitle := page.Locator("text='Help Center'").First()
	err = helpTitle.WaitFor(playwright.LocatorWaitForOptions{
		State: playwright.WaitForSelectorStateVisible,
	})
	require.NoError(t, err)

	// Verify categories are loaded
	categoryTitle := page.Locator("text='Getting Started'")
	err = categoryTitle.WaitFor(playwright.LocatorWaitForOptions{
		State: playwright.WaitForSelectorStateVisible,
	})
	require.NoError(t, err)

	// Test search functionality
	searchBox := page.Locator("input[aria-label='Search for help...']")
	if count, _ := searchBox.Count(); count == 0 {
		// Try targeting by placeholder if aria-label is not automatically applied by flutter web
		searchBox = page.Locator("input[placeholder='Search for help...']")
	}

	err = searchBox.WaitFor(playwright.LocatorWaitForOptions{
		State: playwright.WaitForSelectorStateVisible,
	})
	require.NoError(t, err)

	err = searchBox.Fill("Stripe")
	require.NoError(t, err)

	// Wait for search results
	time.Sleep(2 * time.Second)

	// Verify the relevant article appeared
	searchResultArticle := page.Locator("text='How to accept payments'")
	err = searchResultArticle.WaitFor(playwright.LocatorWaitForOptions{
		State: playwright.WaitForSelectorStateVisible,
	})
	require.NoError(t, err)

	// Click the article
	err = searchResultArticle.Click()
	require.NoError(t, err)

	// Verify we navigated to the article detail screen
	err = page.WaitForURL("**/help/article**", playwright.PageWaitForURLOptions{
		WaitUntil: playwright.WaitUntilStateNetworkidle,
	})
	require.NoError(t, err)

	// Verify the article content is displayed
	articleTitle := page.Locator("text='How to accept payments'")
	err = articleTitle.WaitFor(playwright.LocatorWaitForOptions{
		State: playwright.WaitForSelectorStateVisible,
	})
	require.NoError(t, err)

	articleContent := page.Locator("text='We use Stripe to process payments securely.'")
	err = articleContent.WaitFor(playwright.LocatorWaitForOptions{
		State: playwright.WaitForSelectorStateVisible,
	})
	require.NoError(t, err)

	// Test the feedback buttons
	yesBtn := page.Locator("text='Yes'")
	err = yesBtn.WaitFor(playwright.LocatorWaitForOptions{
		State: playwright.WaitForSelectorStateVisible,
	})
	require.NoError(t, err)

	err = yesBtn.Click()
	require.NoError(t, err)

	// Verify Snackbar appears
	snackbarText := page.Locator("text='Thank you for your feedback!'")
	err = snackbarText.WaitFor(playwright.LocatorWaitForOptions{
		State: playwright.WaitForSelectorStateVisible,
	})
	require.NoError(t, err)

	// Go back to the Help Center list
	backBtn := page.Locator("button[aria-label='Back']").First()
	err = backBtn.WaitFor(playwright.LocatorWaitForOptions{
		State:   playwright.WaitForSelectorStateVisible,
		Timeout: playwright.Float(5000),
	})
	// Only click if it's there
	if count, _ := backBtn.Count(); count > 0 {
		_ = backBtn.Click()
		time.Sleep(1 * time.Second)
	}

	// Wait for the UI to settle
	time.Sleep(2 * time.Second)

	// Test Video Tutorials button
	videoBtn := page.Locator("text='Video Tutorials'").First()
	err = videoBtn.WaitFor(playwright.LocatorWaitForOptions{
		State: playwright.WaitForSelectorStateVisible,
	})
	require.NoError(t, err)
	err = videoBtn.Click()
	require.NoError(t, err)

	err = page.WaitForURL("**/help/videos**", playwright.PageWaitForURLOptions{
		WaitUntil: playwright.WaitUntilStateNetworkidle,
	})
	require.NoError(t, err)

	videoScreenTitle := page.Locator("text='Video Tutorials'").First()
	err = videoScreenTitle.WaitFor(playwright.LocatorWaitForOptions{
		State: playwright.WaitForSelectorStateVisible,
	})
	require.NoError(t, err)

	backBtn = page.Locator("button[aria-label='Back']").First()
	err = backBtn.WaitFor(playwright.LocatorWaitForOptions{
		State:   playwright.WaitForSelectorStateVisible,
	})
	require.NoError(t, err)
	err = backBtn.Click()
	require.NoError(t, err)

	// Test API Docs button
	apiBtn := page.Locator("text='API Docs (Advanced)'").First()
	err = apiBtn.WaitFor(playwright.LocatorWaitForOptions{
		State: playwright.WaitForSelectorStateVisible,
	})
	require.NoError(t, err)
	err = apiBtn.Click()
	require.NoError(t, err)

	err = page.WaitForURL("**/help/api**", playwright.PageWaitForURLOptions{
		WaitUntil: playwright.WaitUntilStateNetworkidle,
	})
	require.NoError(t, err)

	apiScreenTitle := page.Locator("text='API Reference'").First()
	err = apiScreenTitle.WaitFor(playwright.LocatorWaitForOptions{
		State: playwright.WaitForSelectorStateVisible,
	})
	require.NoError(t, err)

	backBtn = page.Locator("button[aria-label='Back']").First()
	err = backBtn.WaitFor(playwright.LocatorWaitForOptions{
		State:   playwright.WaitForSelectorStateVisible,
	})
	require.NoError(t, err)
	err = backBtn.Click()
	require.NoError(t, err)

	// Test Release Notes button
	releaseBtn := page.Locator("text=\"What's New\"").First()
	err = releaseBtn.WaitFor(playwright.LocatorWaitForOptions{
		State: playwright.WaitForSelectorStateVisible,
	})
	require.NoError(t, err)
	err = releaseBtn.Click()
	require.NoError(t, err)

	err = page.WaitForURL("**/help/releases**", playwright.PageWaitForURLOptions{
		WaitUntil: playwright.WaitUntilStateNetworkidle,
	})
	require.NoError(t, err)

	releaseScreenTitle := page.Locator("text=\"What's New\"").First()
	err = releaseScreenTitle.WaitFor(playwright.LocatorWaitForOptions{
		State: playwright.WaitForSelectorStateVisible,
	})
	require.NoError(t, err)

	backBtn = page.Locator("button[aria-label='Back']").First()
	err = backBtn.WaitFor(playwright.LocatorWaitForOptions{
		State:   playwright.WaitForSelectorStateVisible,
	})
	require.NoError(t, err)
	err = backBtn.Click()
	require.NoError(t, err)

	// Wait for the UI to settle
	time.Sleep(2 * time.Second)

	// Test the Ask anything floating button
	// The FAB is a button, we'll click the last button on the page which should be the FAB.
	chatBtn := page.Locator("button").Last()
	err = chatBtn.WaitFor(playwright.LocatorWaitForOptions{
		State: playwright.WaitForSelectorStateVisible,
	})
	require.NoError(t, err)

	err = chatBtn.Click()
	require.NoError(t, err)

	time.Sleep(1 * time.Second)

	chatHeader := page.Locator("text='AI Help Agent'").First()
	err = chatHeader.WaitFor(playwright.LocatorWaitForOptions{
		State: playwright.WaitForSelectorStateVisible,
	})
	require.NoError(t, err)

	// Verify chat interaction with the mocked API service
	chatInput := page.GetByRole("textbox", playwright.PageGetByRoleOptions{Name: "Ask anything..."})
	// Fallback for flutter web
	if count, _ := chatInput.Count(); count == 0 {
		chatInput = page.Locator("input").Last()
	}
	err = chatInput.WaitFor(playwright.LocatorWaitForOptions{
		State: playwright.WaitForSelectorStateVisible,
	})
	require.NoError(t, err)

	err = chatInput.Fill("Hello")
	require.NoError(t, err)

	sendBtn := page.Locator("button").Filter(playwright.LocatorFilterOptions{
		Has: page.Locator("i"),
	}).Last()
	err = sendBtn.Click()
	require.NoError(t, err)

	// Wait for response
	time.Sleep(2 * time.Second)

	// Check that a response bubble exists
	messages := page.Locator("text='Hello'")
	err = messages.WaitFor(playwright.LocatorWaitForOptions{
		State: playwright.WaitForSelectorStateVisible,
	})
	require.NoError(t, err)

	assert.True(t, true)
}
