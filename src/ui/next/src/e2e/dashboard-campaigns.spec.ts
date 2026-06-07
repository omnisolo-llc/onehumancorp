import { expect, test } from "@playwright/test";

test.describe("Dashboard campaign orchestration", () => {
  test("navigates from Dashboard to campaign workflows", async ({ page }) => {
    await page.goto("/dashboard");

    await page.getByRole("link", { name: /Campaign Orchestration/i }).first().click();

    await expect(page.getByRole("heading", { name: "Campaign Orchestration" })).toBeVisible();
    await expect(page.getByText("Campaign Command Queue")).toBeVisible();
    await expect(page.getByRole("link", { name: /Open review workflow/i })).toHaveAttribute("href", "/review-campaigns");
    await expect(page.getByRole("link", { name: /Open referral workflow/i })).toHaveAttribute("href", "/referrals");
    await expect(page.getByRole("link", { name: /Back to Dashboard/i })).toHaveAttribute("href", "/dashboard");
  });
});
