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
});
