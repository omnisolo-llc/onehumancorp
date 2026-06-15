import { test, expect } from "./fixtures";

test.describe("Hybrid Landing Page", () => {
  test("should display Local Sovereignty and Cloud Convenience options", async ({ page }) => {
    await page.goto("/hybrid-landing");

    await expect(page.getByText("OHC Hybrid OS")).toBeVisible();

    await expect(
      page.getByRole("heading", { name: "Local Sovereignty" }),
    ).toBeVisible();
    await expect(page.getByText("Zero Data Leakage:")).toBeVisible();
    await expect(page.getByText("Air-Gapped Autonomy:")).toBeVisible();
    await expect(page.getByText("Self-Hosted LLMs:")).toBeVisible();

    await expect(
      page.getByRole("heading", { name: "Cloud Convenience" }),
    ).toBeVisible();
    await expect(page.getByText("Team Collaboration:")).toBeVisible();
    await expect(page.getByText("Anywhere Access:")).toBeVisible();
    await expect(page.getByText("Fully Managed:")).toBeVisible();

    await expect(
      page.getByRole("button", { name: "Download Desktop" }),
    ).toBeVisible();
    await expect(
      page.getByRole("link", { name: "Start Web Trial" }),
    ).toBeVisible();
  });
});
