import { expect, test } from "./fixtures";

test.describe("Smart Pricing Approval Loop", () => {
  test.use({ viewport: { width: 375, height: 667 } });

  test("Mobile UX and backend approval flow for smart pricing", async ({
    page,
    request,
  }) => {
    // Navigate to the dashboard where the Agent Feed is displayed
    await page.goto("/dashboard");

    // Ensure the unified feed tab is visible
    await expect(page.locator("text=Activity Feed").first()).toBeVisible({ timeout: 15000 });

    // Verify the seeded smart pricing card is rendered
    const pricingCard = page.locator("text=Smart Price Suggestion: Vegan Celebration Cake").first();
    await expect(pricingCard).toBeVisible();

    // Verify specific info inside the card
    await expect(page.locator("text=Current Price:").first()).toBeVisible();
    await expect(page.locator("text=$39.99").first()).toBeVisible();
    await expect(page.locator("text=Suggested Price:").first()).toBeVisible();
    await expect(page.getByTestId("smart-pricing-new-price").first()).toHaveText("$45.00");
    await expect(page.getByTestId("smart-pricing-sales-projection").first()).toHaveText("+$150");

    // Click the [Approve & Run Sale] button
    const approveButton = page.getByTestId("approve-run-sale").first();
    await expect(approveButton).toBeVisible();
    await approveButton.click();

    // The item should disappear from the feed
    await expect(pricingCard).not.toBeVisible();

    // Verify backend mutation: wait briefly then check the DB
    await page.waitForTimeout(2000);

    // Check that the product price was updated via a helper endpoint or db inspection
    const productPriceResponse = await request.post("/api/e2e/setup", {
      data: {
        query: `SELECT price FROM products WHERE id = 'e2e-product-cake';`
      }
    });

    if (productPriceResponse.ok()) {
      const data = await productPriceResponse.json();
      expect(data.rows[0].price).toBeCloseTo(45.00);
    }
  });
});
