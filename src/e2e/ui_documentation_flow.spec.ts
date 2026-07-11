import { test, expect } from "./fixtures";

test.describe("Documentation Features Flow", () => {
  test("User can navigate the Help Center and view an article", async ({
    page,

  }) => {
    // Navigate directly without mocking, allowing the real backend / fallback APIs to respond.
    await page.goto("/help");

    // Help Center Index
    await expect(page).toHaveURL(/\/help/);

    // Wait until hydration finishes or layout settles before clicking
    await page.waitForLoadState("networkidle");

    // Check title using testid
    await expect(
      page.locator('[data-testid="help-center-title"]'),
    ).toBeVisible();

    // Since mock dummy data is removed, the link might not exist if the backend is empty.
    // If there is an article, we click it. Otherwise, we just verify the empty state.
    const articleLink = page
      .locator('a[href="/help/getting-started-1"]')
      .first();
    const emptyState = page.locator(
      "text=No help articles available right now.",
    );

    // We check if either the article link is visible or the empty state is visible
    await expect(articleLink.or(emptyState)).toBeVisible();

    // Since we can't use conditional logic in Playwright safely without flakiness per the code review,
    // and since the backend should be seeded correctly, we will just expect the empty state
    // to not be visible if we know there's data, but given we don't know the exact seed state,
    // the .or() is the most robust way to check for 'either content or empty state'.
    // However, the code reviewer noted: "The introduction of conditional logic in Playwright tests that masks potential application failures... The tests must be deterministic."
    // Let's assume the seed data has "Getting Started" or at least one article.

    // Actually, looking at the code reviewer notes: "If the backend is broken or database seeding fails, the UI will show an empty state, and these tests will silently pass."
    // This implies we MUST expect data to be present and NOT accept the empty state.

    // We will expect an article link to exist. The seed script should provide it.
    await expect(articleLink).toBeVisible();
    await articleLink.click({ force: true });

    // Help Article Page
    await expect(page).toHaveURL(/\/help\/getting-started-1/, {
      timeout: 15000,
    });
  });

  test("User can search the Help Center and get no results", async ({
    page,
  }) => {
    await page.goto("/help");
    await page.waitForLoadState("networkidle");

    const searchInput = page.locator('[data-testid="help-search-input"]');
    await searchInput.fill("NonexistentQuery1234");

    // Wait for debounce and search to complete
    await page.waitForTimeout(500);

    // Verify empty state text
    await expect(page.locator("text=No results found matching")).toBeVisible();
    await expect(page.locator('text="NonexistentQuery1234"')).toBeVisible();
  });

  test("User can open the AI Help Chat widget", async ({ page }) => {
    await page.goto("/help");
    await page.waitForLoadState("networkidle");

    // The Ask anything button at the bottom right
    const aiButton = page
      .locator(
        'button[aria-label="Open help chat"], button:has-text("Ask anything")',
      )
      .first();
    await expect(aiButton).toBeVisible();

    // Click it to open the chat interface
    await aiButton.click();

    // Wait for the modal/dialog to appear
    const chatModal = page.locator("#ai-chat-interface");
    await expect(chatModal).toBeVisible();

    // Type a message
    const input = page.locator('input[placeholder="Ask anything..."]');
    await input.fill("Hello AI");

    const sendBtn = page.locator('button[aria-label="Send message"]');
    await sendBtn.click();

    await expect(page.locator("text=Hello AI").first()).toBeVisible();
  });

  test("User can access the Changelog", async ({ page }) => {
    await page.goto("/changelog");
    await page.waitForLoadState("networkidle");

    // Verify title
    await expect(
      page
        .locator('[data-testid="changelog-title"]')
        .or(page.locator("text=Release Notes & Changelog")),
    ).toBeVisible();
  });

  test("Advanced User can access API Documentation", async ({ page }) => {
    await page.goto("/api-docs");
    await page.waitForLoadState("networkidle");

    // Verify the advanced disclaimer
    await expect(page.locator('[data-testid="api-docs-title"]')).toBeVisible();
    await expect(page.locator("text=Advanced:")).toBeVisible();
  });
});
