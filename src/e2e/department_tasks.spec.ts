import { test, expect } from "./fixtures";

test.describe("AI Agent Department UI Mocks", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
  });

  test("draft-to-approval flow for AI Agent Departments", async ({
    page,
    request,
  }) => {
    // Navigate naturally to team dashboard
    await page.goto("/team");

    const ambassadorCard = page.locator("button", { hasText: "The Ambassador" });
    await expect(ambassadorCard).toContainText("awaiting approval");
    await ambassadorCard.click();

    // Verify draft UI
    await expect(page.locator("h1")).toContainText("The Ambassador");
    await expect(page.getByText("Draft email for review")).toBeVisible();

    await page.getByRole("button", { name: "Approve" }).click();

    await expect(page.getByText("All Caught Up!")).toBeVisible();
  });

  test("UI: Navigates to team page and displays all AI Agent Departments", async ({
    page,
  }) => {
    await page.goto("/team");
    await expect(page.locator("h1")).toContainText("Your Team");

    const expectedDepartments = [
      "The Manager",
      "The Promoter",
      "The Salesperson",
      "The Ambassador",
      "The Accountant",
      "The Protector",
      "The Advisor",
    ];

    for (const dept of expectedDepartments) {
      await expect(page.locator(`text=${dept}`)).toBeVisible();
    }
  });

  test("UI: Department card shows pending approval and opens ApprovalInbox", async ({
    page,
  }) => {
    await page.goto("/team");

    const ambassadorCard = page.locator("button", { hasText: "The Ambassador" });
    await expect(ambassadorCard).toContainText("1 item awaiting approval");

    await ambassadorCard.click();

    await expect(page.locator("h1")).toContainText("The Ambassador");
    await expect(page.getByText("Draft email for review")).toBeVisible();
  });

  test("UI: Approving a request updates the UI to All Caught Up", async ({
    page,
  }) => {
    await page.goto("/team");

    const promoterCard = page.locator("button", { hasText: "The Promoter" });
    await expect(promoterCard).toContainText("1 item awaiting approval");
    await promoterCard.click();

    await expect(page.locator("h1")).toContainText("The Promoter");
    await expect(
      page.getByText("Generated 7-day social media plan for Vegan Celebration Cake"),
    ).toBeVisible();

    await page.getByRole("button", { name: "Approve" }).click();

    await expect(page.getByText("All Caught Up!")).toBeVisible();
  });

  test("UI: Verify risk level UI representations", async ({ page }) => {
    await page.goto("/team");

    // Check High Risk
    await page.locator("button", { hasText: "The Salesperson" }).click();
    const highRiskBadge = page.locator("span", { hasText: "High Risk" }).first();
    await expect(highRiskBadge).toBeVisible();
    await expect(highRiskBadge).toHaveClass(/bg-orange-100/);
    await expect(highRiskBadge).toHaveClass(/text-orange-700/);
    await page.goto("/team");

    // Check Low Risk
    await page.locator("button", { hasText: "The Promoter" }).click();
    const lowRiskBadge = page.locator("span", { hasText: "Low Risk" }).first();
    await expect(lowRiskBadge).toBeVisible();
    await expect(lowRiskBadge).toHaveClass(/bg-blue-100/);
    await expect(lowRiskBadge).toHaveClass(/text-blue-700/);
  });

  test("UI: Rejecting a request updates the UI to All Caught Up", async ({
    page,
  }) => {
    await page.goto("/team");

    const salesCard = page.locator("button", { hasText: "The Salesperson" });
    await expect(salesCard).toContainText("1 item awaiting approval");
    await salesCard.click();

    await page.getByRole("button", { name: "Reject / Edit" }).click();

    await expect(page.getByText("All Caught Up!")).toBeVisible();
  });

  test("UI: Department with no approvals shows All Caught Up directly", async ({
    page,
  }) => {
    await page.goto("/team");

    await page.locator("button", { hasText: "The Accountant" }).click();

    await expect(page.locator("h1")).toContainText("The Accountant");
    await expect(page.getByText("All Caught Up!")).toBeVisible();
    await expect(
      page.getByText("There are no pending actions requiring your review."),
    ).toBeVisible();
  });

  test("UI: End-to-End CUJ - Order Placed event to Customer Success draft approval", async ({
    page,
    request,
  }) => {
    // In order for the E2E CUJ to run again we need a fresh tenant to ensure fresh events, or we assert the existing webhook
    // is able to create another draft approval in The Ambassador
    const response = await request.post("/api/agents/webhook", {
      data: {
        tenant_id: "e2e-tenant",
        source: "stripe",
        message: "order_placed",
      },
    });
    expect(response.ok()).toBeTruthy();

    await page.goto("/team");

    await expect(
      page.locator("button", { hasText: "The Ambassador" }),
    ).toContainText("awaiting approval", { timeout: 10000 });

    const ambassadorCard = page.locator("button", { hasText: "The Ambassador" });
    await expect(ambassadorCard).toContainText("awaiting approval");
    await ambassadorCard.click();

    await expect(page.locator("h1")).toContainText("The Ambassador");

    const approvalCard = page
      .locator("div", { hasText: "Send personalized thank you & shipping ETA" })
      .first();
    await expect(approvalCard).toBeVisible();

    await approvalCard.getByRole("button", { name: "Approve" }).click();
    await expect(page.getByText("All Caught Up!")).toBeVisible({ timeout: 5000 });
  });
});
