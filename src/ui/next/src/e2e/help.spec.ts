import { test, expect } from "@playwright/test";

test.describe("Help Center", () => {
  test.beforeEach(async ({ page }) => {
    // Mock backend API responses for tests
    await page.route("**/api/help", async (route) => {
      await route.fulfill({
        status: 200,
        contentType: "application/json",
        body: JSON.stringify([
          {
            category: "Getting Started",
            id: "getting-started-1",
            title: "Getting Started with Your Store",
            desc: "Welcome to OneHumanCorp!",
            link: "/help/getting-started-1",
          },
          {
            category: "My Store",
            id: "my-store",
            title: "My Store",
            desc: "My store info",
            link: "/help/my-store",
          },
          {
            category: "Payments",
            id: "accept-payments",
            title: "Accepting Payments",
            desc: "Learn how to accept credit cards.",
            link: "/help/accept-payments",
          },
        ]),
      });
    });

    await page.route("**/api/help/search*", async (route) => {
      const url = new URL(route.request().url());
      const query = url.searchParams.get("q") || "";
      if (query.toLowerCase().includes("my store")) {
        await route.fulfill({
          status: 200,
          contentType: "application/json",
          body: JSON.stringify([
            {
              category: "My Store",
              id: "my-store",
              title: "My Store",
              desc: "My store info",
              link: "/help/my-store",
            },
          ]),
        });
      } else {
        await route.fulfill({
          status: 200,
          contentType: "application/json",
          body: JSON.stringify([]),
        });
      }
    });
  });

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

    // The mock backend search for "Getting Started" returns [] based on our mock setup, so we don't click it via search.
    // Instead we just click the visible link from the initial load.
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

    await searchInput.fill("My Store");

    // Wait for UI to update (non-matching articles should disappear)
    await expect(
      page.locator('a[href="/help/getting-started-1"]'),
    ).not.toBeVisible({ timeout: 10000 });

    const articleLink = page.locator('a[href="/help/my-store"]');
    await expect(articleLink).toBeVisible({ timeout: 10000 });
  });

  test("should open help chat and send a message", async ({ page }) => {
    await page.goto("/help");

    // Wait for hydration to complete by checking for initial content
    await expect(
      page.locator("h2", { hasText: "Getting Started" }),
    ).toBeVisible({ timeout: 15000 });

    // Find and click the floating Ask anything button
    const chatButton = page.locator('button[aria-label="Open help chat"]');
    await expect(chatButton).toBeVisible();
    await chatButton.dispatchEvent("click");

    // Wait for the chat to open and be visible
    const chatHeader = page.locator("#ai-chat-header");
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
});
