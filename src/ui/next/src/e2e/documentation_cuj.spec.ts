import { test, expect } from "../../../../e2e/fixtures";

test.describe("Documentation User Journey", () => {
  test("Maya navigates the Help Center and views the Changelog", async ({
    page,
  }) => {
    await page.goto("/changelog");

    // Verify Changelog is loaded
    await expect(
      page.locator("h1", { hasText: "Release Notes & Changelog" }),
    ).toBeVisible();

    // Now Maya navigates to the Help Center
    await page.goto("/help");

    // Verify Help Center is loaded
    await expect(page.locator("h1", { hasText: "In-App Help Center" })).toBeVisible();

    // Verify Categories loaded from the real backend
    await expect(
      page.locator("h2", { hasText: "Getting Started" }),
    ).toBeVisible();
    await expect(page.locator("h2", { hasText: "My Store" })).toBeVisible();
    await expect(page.locator("h2", { hasText: "Payments" })).toBeVisible();

    // Verify Videos list loads
    await expect(
      page.locator("h2", { hasText: "Video Tutorials" }),
    ).toBeVisible({ timeout: 10000 });

    const searchInput = page.locator(
      'input[placeholder="Search for help articles and videos..."]',
    );

    // Maya searches for "products" to learn how to add products
    await searchInput.fill("products");

    // Click on the article
    const myStoreLink = page.getByRole("link", { name: /Adding Products/ });
    await expect(myStoreLink).toBeVisible({ timeout: 10000 });
    await myStoreLink.click();
    await page.waitForURL("/help/add-products");
    await expect(page.getByRole("heading", { name: "Managing My Store" })).toBeVisible();
  });

  test("Maya opens the Help Chat and asks a question", async ({ page }) => {
    await page.goto("/help");

    // Verify the Help Chat floating button is visible
    const chatButton = page.locator('button[aria-label="Open help chat"]');
    await expect(chatButton).toBeVisible();

    // Open the Help Chat
    await chatButton.focus();
    await page.keyboard.press("Enter");

    await page.getByRole("button", { name: "Ask anything" }).click();

    // Locate the chat input and send button
    const chatInput = page.locator('input[placeholder="Ask anything..."]');
    const sendButton = page.locator('button[aria-label="Send message"]');

    // Type a message and send it
    await chatInput.fill("How do I add a product?");
    await sendButton.click();

    // Verify that the user message appears in the chat
    const widget = page.locator("#omnisolo-floating-help-widget");
    await expect(widget.getByText("How do I add a product?", { exact: true })).toBeVisible();

    // Verify AI response from the real backend
    await expect(
      widget.getByText(/To set up your storefront/),
    ).toBeVisible({ timeout: 15000 });
  });
});
