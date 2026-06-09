import { test, expect } from "./fixtures";

test.describe("Hybrid Landing Page", () => {
  test("should display Local-First and Cloud options", async ({ page }) => {
    await page.goto("/dashboard");

    await expect(
      page.getByRole("heading", { name: "Dashboard" }),
    ).toBeVisible();
    await expect(
      page.getByRole("heading", { name: "Business Analytics" }),
    ).toBeVisible();
    await expect(page.getByText("Operations Map")).toBeVisible();
    await expect(page.getByText("Action Required")).toBeVisible();
    await expect(
      page.getByRole("link", { name: "Integrations" }),
    ).toBeVisible();
  });
  test("should display Cloud Team Bridge and generate invite link", async ({ page }) => {
    await page.goto("/dashboard");

    // In our test environment, we test the Tauri app interface by directly visiting the dashboard
    // Wait for the specific Cloud Team Bridge card
    await expect(page.locator('.ohc-growth-card:has-text("Cloud Team Bridge")')).toBeVisible();

    const getLinkBtn = page.getByRole("button", { name: "Get My Invite Link" });
    await expect(getLinkBtn).toBeVisible();
    await getLinkBtn.click();

    await expect(getLinkBtn).toBeHidden();

    const linkInput = page.locator("#referral-link");
    await expect(linkInput).toBeVisible();

    const linkValue = await linkInput.inputValue();
    expect(linkValue).toContain("cloud.ohc.network/invite");
  });
});
