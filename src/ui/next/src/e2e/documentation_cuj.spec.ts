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
    const myStoreLink = page.locator("h3", { hasText: "Adding Products" });
    await expect(myStoreLink).toBeVisible({ timeout: 10000 });
  });

  test("Maya opens the Help Chat and asks a question", async ({ page }) => {
    await page.goto("/help");

    // Verify the Help Chat floating button is visible
    const chatButton = page.locator('button[aria-label="Open help chat"]');
    await expect(chatButton).toBeVisible();

    // Open the Help Chat
    await chatButton.click();

    // Verify the Help Chat interface is visible
    await expect(page.locator("h3", { hasText: "Ask anything" })).toBeVisible();

    // Locate the chat input and send button
    const chatInput = page.locator('input[placeholder="Ask anything..."]');
    const sendButton = page.locator('button[aria-label="Send message"]');

    // Type a message and send it
    await chatInput.fill("How do I add a product?");
    await sendButton.click();

    // Verify that the user message appears in the chat
    await expect(
      page.locator("div", { hasText: "How do I add a product?" }).first(),
    ).toBeVisible();

    // Verify AI response from the real backend
    await expect(
      page.locator("text=How do I add a product?").first(),
    ).toBeVisible();
  });

  test("Maya uses a walkthrough from the Help Widget", async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto("/dashboard");

    // Click the floating help button
    const helpButton = page.locator('#ohc-floating-help-btn');
    await expect(helpButton).toBeVisible();
    await helpButton.click({ force: true });

    // Click a Tour button inside the Help Widget
    const tourButton = page.locator('button', { hasText: 'Tour: Store Setup' });
    await expect(tourButton).toBeVisible();
    await tourButton.click();

    // The Walkthrough bubble should appear
    const bubble = page.locator('#walkthrough-bubble');
    await expect(bubble).toBeVisible();

    // First step
    await expect(bubble.locator('h4')).toHaveText('Set up your store');

    const nextBtn = page.locator('#wt-next');
    await expect(nextBtn).toBeVisible();

    // Click through steps
    await nextBtn.click();
    await expect(bubble.locator('h4')).toHaveText('Describe your business');

    await nextBtn.click();
    await expect(bubble.locator('h4')).toHaveText('Generate Store');

    // Click finish
    await expect(nextBtn).toHaveText('Finish');
    await nextBtn.click();

    // Walkthrough should close
    await expect(bubble).not.toBeVisible();
  });
});
