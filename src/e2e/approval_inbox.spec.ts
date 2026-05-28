import { test, expect } from "./fixtures";

test("AI Team Dashboard and Approval Inbox", async ({ page, request }) => {
  await page.goto("http://localhost:3000/");
  await page.goto("http://localhost:3000/team");

  await expect(page.locator("text=The Ambassador")).toBeVisible();
  await expect(page.locator("text=The Promoter")).toBeVisible();

  const ambassadorCard = page.locator("text=The Ambassador").locator("..");
  await ambassadorCard.click({ force: true });

  await expect(page.locator("text=The Ambassador").first()).toBeVisible({ timeout: 10000 });
});
