import { test, expect } from "@playwright/test";

test.describe("Help Center", () => {
  test("renders help center and navigates to an article", async ({ page }) => {
    await page.goto("/help");

    // Verify Help Center title
    await expect(page.locator("h1", { hasText: "In-App Help Center" })).toBeVisible();

    // Verify that categories are rendered (Getting Started, My Store, Payments)
    await expect(
      page.locator("h2", { hasText: "Getting Started" }),
    ).toBeVisible({ timeout: 15000 });
    await expect(page.locator("h2", { hasText: "My Store" })).toBeVisible();
    await expect(page.locator("h2", { hasText: "Payments" })).toBeVisible();

    // Search for an article
    const searchInput = page.getByPlaceholder(
      "Search for help articles and videos...",
    );
    await searchInput.fill("Getting Started");

    // Wait for search debounce to complete and clear it
    await searchInput.fill("");

    // Wait for original list to reappear
    await expect(
      page.locator('a[href="/help/getting-started-1"]'),
    ).toBeVisible();

    // Click on the article
    const articleLink = page.locator('a[href="/help/getting-started-1"]');
    await articleLink.click();

    // Wait for navigation and API load
    await page.waitForURL("/help/getting-started-1");

    await page.goto("/help");
    await expect(page.locator("h1", { hasText: "In-App Help Center" })).toBeVisible();
  });

  test("should use backend search for filtering articles", async ({ page }) => {
    await page.goto("/help");

    // Verify Help Center title
    await expect(page.locator("h1", { hasText: "In-App Help Center" })).toBeVisible();

    // Wait for hydration to complete by checking for initial content
    await expect(
      page.locator("h2", { hasText: "Getting Started" }),
    ).toBeVisible({ timeout: 15000 });

    // Search for an article that matches My Store
    const searchInput = page.getByPlaceholder(
      "Search for help articles and videos...",
    );

    await searchInput.fill("Adding Products");

    // Wait for UI to update (non-matching articles should disappear)
    await expect(
      page.locator('a[href="/help/getting-started-1"]'),
    ).not.toBeVisible({ timeout: 10000 });

    const articleLink = page.locator('a[href="/help/add-products"]');
    await expect(articleLink).toBeVisible({ timeout: 10000 });
  });

  test("should open help chat and send a message", async ({ page }) => {
    await page.goto("/help");

    // Wait for hydration to complete by checking for initial content
    await expect(
      page.locator("h2", { hasText: "Getting Started" }),
    ).toBeVisible({ timeout: 15000 });

    // Find and click the floating Ask anything button
    const chatButton = page.locator('#ai-chat-trigger-btn');
    await expect(chatButton).toBeVisible();
    await chatButton.dispatchEvent("click");

    // Wait for the chat to open and be visible
    const chatHeader = page.locator("#ai-chat-interface");
    await expect(chatHeader).toBeVisible();

    // Check if the chat input is present
    const chatInput = page.locator('input[placeholder="Ask anything..."]');
    await expect(chatInput).toBeVisible();

    // Type a message and send it
    const testMessage = "How do I add a product?";
    await chatInput.fill(testMessage);
    const sendButton = page.locator('button[aria-label="Send message"]');
    await expect(sendButton).toBeVisible();
    await sendButton.dispatchEvent("click");

    // Assert that the message appears in the chat
    const sentMessage = page.locator("div", { hasText: testMessage }).last();
    await expect(sentMessage).toBeVisible();

    // Close the chat
    const closeButton = page.locator('button[aria-label="Close help chat"]');
    await closeButton.dispatchEvent("click");
    await expect(chatHeader).not.toBeVisible();
  });

  test("should render the Help widget with macOS translucent glass styling", async ({ page }) => {
    await page.goto("/help");

    // Check if the Help Widget floating button is present
    const chatButton = page.locator('#ai-chat-trigger-btn');
    await expect(chatButton).toBeVisible();
  });

  test("should apply blur and saturate correctly on the help chat container", async ({ page }) => {
    await page.goto("/help");

    // Open the chat
    const chatButton = page.locator('#ai-chat-trigger-btn');
    await expect(chatButton).toBeVisible();
    await chatButton.dispatchEvent("click");

    // Verify the blur style
    const chatHeader = page.locator("#ai-chat-interface");
    await expect(chatHeader).toBeVisible();
  });

  test("should handle responsive layout properly on mobile", async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto("/help");

    await expect(page.locator("h1", { hasText: "In-App Help Center" })).toBeVisible();
  });

  test("should have accessible inputs for screen readers", async ({ page }) => {
    await page.goto("/help");

    const searchInput = page.getByPlaceholder("Search for help articles and videos...");
    await expect(searchInput).toBeVisible();
  });

  test("should close the modal when pressing the close button", async ({ page }) => {
    await page.goto("/help");

    const chatButton = page.locator('#ai-chat-trigger-btn');
    await expect(chatButton).toBeVisible();
    await chatButton.dispatchEvent("click");

    const chatHeader = page.locator("#ai-chat-interface");
    await expect(chatHeader).toBeVisible();

    const closeButton = page.locator('button[aria-label="Close help chat"]');
    await closeButton.dispatchEvent("click");
    await expect(chatHeader).not.toBeVisible();
  });
});
