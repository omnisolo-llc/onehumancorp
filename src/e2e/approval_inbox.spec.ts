import { test, expect } from "./fixtures";

test("AI Team Dashboard and Approval Inbox", async ({ page, request }) => {
  // Mock the API for testing the UI specifically
  await page.route("/api/agents/approvals", async (route) => {
    const json = {
      pending_approvals: [
        {
          id: "e2e-approval-mock-1",
          tenant_id: "mock-tenant",
          department: "CustomerSuccess",
          description: "Draft email for review: Maya ordered a vegan cake",
          status: "Pending",
          action_risk: "High",
        },
        {
          id: "e2e-approval-mock-2",
          tenant_id: "mock-tenant",
          department: "Marketing",
          description: "Draft Instagram Post: New vegan cakes available!",
          status: "Pending",
          action_risk: "Low",
        },
        {
          id: "e2e-approval-mock-3",
          tenant_id: "mock-tenant",
          department: "Marketing",
          description:
            "Generated 7-day social media plan for Vegan Celebration Cake",
          status: "Pending",
          action_risk: "Low",
          feature_type: "social_calendar",
        },
        {
          id: "e2e-approval-mock-4",
          tenant_id: "mock-tenant",
          department: "Sales",
          description: "Abandoned cart recovery: 10% discount for Sarah",
          status: "Pending",
          action_risk: "High",
          feature_type: "abandoned_cart",
        },
      ],
    };
    await route.fulfill({ json });
  });

  await page.route("/api/agents/approvals/*", async (route) => {
    await route.fulfill({ json: { success: true } });
  });

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
    page.locator("text=Draft email for review: Maya ordered a vegan cake"),
  ).toBeVisible();

  // 3. User approves the action (Swipe right / Approve button)
  const approveBtn = page.locator("button", { hasText: "Approve" }).first();
  await approveBtn.click();

  // Wait for the action to be processed (mocking the UI removal)
  await expect(
    page.locator("text=Draft email for review: Maya ordered a vegan cake"),
  ).not.toBeVisible();
});

test("Approval Inbox Special UI feature cards", async ({ page }) => {
  await page.route("/api/agents/approvals", async (route) => {
    const json = {
      pending_approvals: [
        {
          id: "e2e-approval-mock-3",
          tenant_id: "mock-tenant",
          department: "Marketing",
          description:
            "Generated 7-day social media plan for Vegan Celebration Cake",
          status: "Pending",
          action_risk: "Low",
          feature_type: "social_calendar",
        },
        {
          id: "e2e-approval-mock-4",
          tenant_id: "mock-tenant",
          department: "Sales",
          description: "Abandoned cart recovery: 10% discount for Sarah",
          status: "Pending",
          action_risk: "High",
          feature_type: "abandoned_cart",
        },
      ],
    };
    await route.fulfill({ json });
  });

  await page.goto("/team");
  const promoterCard = page.locator("text=The Promoter").locator("..");
  await promoterCard.click();

  await expect(
    page.locator("text=7-Day Social Calendar Generated"),
  ).toBeVisible();
  await expect(page.locator("text=Mon")).toBeVisible();
  await expect(page.locator("text=Sun")).toBeVisible();

  await page.goto("/team");
  const salesCard = page.locator("text=The Salesperson").locator("..");
  await salesCard.click();

  await expect(page.locator("text=Abandoned Cart Detected")).toBeVisible();
  await expect(
    page.locator("text=Sarah left a $45 Vegan Chocolate Cake in her cart."),
  ).toBeVisible();
});
