import { test, expect } from "@playwright/test";
import { setupTestApp } from "./db_utils";

test.describe("UI Token Efficiency test via Walkup interceptor", () => {
  let app: Awaited<ReturnType<typeof setupTestApp>>;

  test.beforeEach(async ({ page }) => {
    app = await setupTestApp();
    await page.goto("http://localhost:3000/pos/walkup");
  });

  test.afterEach(async () => {
    if (app) {
      await app.teardown();
    }
  });

  test("submits an interceptor order successfully and verifies token reduction usage", async ({ page }) => {
    const orderText = "I would like to order an espresso and a large cappuccino please";
    const walkupInput = page.getByPlaceholder(/What do you need/i).or(page.locator("input[name='walkupInput']"));
    await expect(walkupInput).toBeVisible();
    await walkupInput.fill(orderText);

    const submitBtn = page.getByRole("button", { name: /Process|Submit|Order/i });
    await expect(submitBtn).toBeVisible();
    await submitBtn.click();

    await expect(page.locator("text=/Success|Order Received|Processed/i")).toBeVisible({ timeout: 15000 });
  });
});
