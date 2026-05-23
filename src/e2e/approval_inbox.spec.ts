import { test, expect } from "./fixtures";

test("AI Team Dashboard and Approval Inbox", async ({ page, request }) => {
  // 1. User opens the app, authenticates and navigates to the Team Dashboard
  await page.goto("/");

  // Login via UI (from global-setup login structure)
  // Assuming the user is already logged in via global-setup.ts
  await page.goto("/team");

  // Assert Team Dashboard elements (375px mobile-first)
  await expect(page.locator("text=The Ambassador")).toBeVisible();
  await expect(page.locator("text=The Promoter")).toBeVisible();

  // "The Ambassador" has pending approvals indicator (e.g., a badge)
  const ambassadorCard = page.locator("text=The Ambassador").locator("..");
  await expect(
    ambassadorCard.locator("text=1 item awaiting approval"),
  ).toBeVisible();

  // 2. User taps "The Ambassador" department
  await ambassadorCard.click();

  // Verify approval inbox view for The Ambassador
  await expect(
    page.locator("text=Draft email for review"),
  ).toBeVisible();

  // 3. User approves the action (Swipe right / Approve button)
  const approveBtn = page.locator("button", { hasText: "Approve" }).first();
  await approveBtn.click();

  // Wait for the action to be processed
  await expect(
    page.locator("text=Draft email for review"),
  ).not.toBeVisible();
});
