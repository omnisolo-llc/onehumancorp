import { test, expect } from "./fixtures";

test.describe("AI Agent Department UI Mocks", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
  });

  test("draft-to-approval flow for AI Agent Departments", async ({
    page,
  }) => {
    await page.goto("/team");

    const ambassadorCard = page.locator("button", { hasText: "The Ambassador" });
    await expect(ambassadorCard).toContainText("awaiting approval");
    await ambassadorCard.click();

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

    await page.locator("button", { hasText: "The Salesperson" }).click();
    const highRiskBadge = page.locator("span", { hasText: "High Risk" }).first();
    await expect(highRiskBadge).toBeVisible();
    await expect(highRiskBadge).toHaveClass(/bg-orange-100/);
    await expect(highRiskBadge).toHaveClass(/text-orange-700/);
    await page.goto("/team");

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
    const response = await request.post("/api/agents/webhook", {
      data: {
        tenant_id: "e2e-tenant",
        source: "stripe",
        message: "order_placed",
      },
    });
    expect(response.ok()).toBeTruthy();

    await page.goto("/team");

    // Operations ("The Manager") should receive the event and automatically execute.
    // And it chains to Customer Success ("The Ambassador") which drafts an approval.
    // Wait for the Ambassador to get the drafted approval:
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

  test("UI: End-to-End CUJ - Respects tenant throttling limits and safely returns 429", async ({
    page,
    request,
  }) => {
    let exhausted = false;
    // We send up to 105 requests. Since the default budget is 100, it should eventually return 429.
    for (let i = 0; i < 105; i++) {
        const res = await request.post("/api/agents/webhook", {
            data: { tenant_id: "e2e-tenant", source: "stripe", message: "order_placed" },
        });

        if (res.status() === 429) {
            exhausted = true;
            break;
        } else if (res.status() !== 200) {
            throw new Error(`Unexpected status code: ${res.status()}`);
        }
    }

    expect(exhausted).toBe(true);

    await page.goto("/team");
    await expect(page.locator("h1")).toContainText("Your Team");
    // System should not crash
    await expect(page.locator("button", { hasText: "The Ambassador" })).toBeVisible();
  });
});

  test("UI: Check empty inbox behavior", async ({ page }) => {
    await page.goto("/team");
    const legalCard = page.locator("button", { hasText: "The Protector" });
    await legalCard.click();
    await expect(page.locator("h1")).toContainText("The Protector");
    await expect(page.getByText("All Caught Up!")).toBeVisible();
  });

  test("UI: Check back button works in ApprovalInbox", async ({ page }) => {
    await page.goto("/team");
    const promoterCard = page.locator("button", { hasText: "The Promoter" });
    await promoterCard.click();
    await expect(page.locator("h1")).toContainText("The Promoter");
    await page.getByRole("button", { name: "Back" }).first().click();
    await expect(page.locator("h1")).toContainText("Your Team");
  });

  test("UI: Handle error on approve gracefully", async ({ page }) => {
    // Intercept to mock error response
    await page.route('**/api/agents/approvals/*', route => route.abort());
    await page.goto("/team");
    const promoterCard = page.locator("button", { hasText: "The Promoter" });
    await promoterCard.click();
    await page.getByRole("button", { name: "Approve" }).click();
    // It should handle gracefully, possibly reverting the UI or maintaining state
    // In our implementation, it removes optimistically and adds back or refetches on error
    await expect(page.getByText("Generated 7-day social media plan for Vegan Celebration Cake")).toBeVisible();
  });

  test("UI: Check empty inbox behavior", async ({ page }) => {
    await page.goto("/team");
    const legalCard = page.locator("button", { hasText: "The Protector" });
    await legalCard.click();
    await expect(page.locator("h1")).toContainText("The Protector");
    await expect(page.getByText("All Caught Up!")).toBeVisible();
  });

  test("UI: Check back button works in ApprovalInbox", async ({ page }) => {
    await page.goto("/team");
    const promoterCard = page.locator("button", { hasText: "The Promoter" });
    await promoterCard.click();
    await expect(page.locator("h1")).toContainText("The Promoter");
    await page.getByRole("button", { name: "Your Team" }).first().click();
    await expect(page.locator("h1")).toContainText("Your Team");
  });

  test("UI: Handle error on approve gracefully", async ({ page }) => {
    // Intercept to mock error response
    await page.route('**/api/agents/approvals/*', route => route.abort());
    await page.goto("/team");
    const promoterCard = page.locator("button", { hasText: "The Promoter" });
    await promoterCard.click();
    await page.getByRole("button", { name: "Approve" }).click();
    await expect(page.getByText("Generated 7-day social media plan for Vegan Celebration Cake")).toBeVisible();
  });
