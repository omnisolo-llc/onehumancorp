import { test, expect } from "./fixtures";

test("dashboard order milestone is visible after marking an order ready", async ({
  page,
}) => {
  await page.goto("/dashboard");
  await page.getByRole("button", { name: "Mark Order Ready" }).click();

  await expect(page.locator("#milestone-card")).toBeVisible();
  await expect(page.locator("#milestone-card")).toContainText("First Sale!");
});

test("draft-to-approval flow for AI Agent Departments", async ({
  page,
  request,
}) => {
  // Wait a bit to ensure the server is ready, just in case
  await page.waitForTimeout(1000);

  // 1. Fetch pending approvals to verify the seeded request exists
  const getPendingRes = await request.get("/api/agents/approvals");
  expect(getPendingRes.ok()).toBeTruthy();
  const getPendingJson = await getPendingRes.json();

  // Find our seeded approval request
  const pendingApproval = getPendingJson.pending_approvals?.find(
    (a: any) => a.id === "e2e-approval-1",
  );
  expect(pendingApproval).toBeDefined();
  expect(pendingApproval.status).toBe("Pending");
  expect(pendingApproval.department).toBe("CustomerSuccess");

  // 2. Approve the request via the approval endpoint
  const approveRes = await request.post(
    "/api/agents/approvals/e2e-approval-1",
    {
      data: { approved: true },
    },
  );
  expect(approveRes.ok()).toBeTruthy();
  const approveJson = await approveRes.json();
  expect(approveJson.success).toBe(true);

  // 3. Verify it is no longer pending
  const getAfterRes = await request.get("/api/agents/approvals");
  expect(getAfterRes.ok()).toBeTruthy();
  const getAfterJson = await getAfterRes.json();
  const stillPending = getAfterJson.pending_approvals?.find(
    (a: any) => a.id === "e2e-approval-1",
  );
  expect(stillPending).toBeUndefined();
});

test("UI: Navigates to team page and displays all AI Agent Departments", async ({
  page,
}) => {
  await page.route("**/api/agents/approvals", async (route) => {
    await route.fulfill({ json: { pending_approvals: [] } });
  });

  await page.goto("/team");
  await expect(page.locator("h1")).toContainText("Your Team");
  await expect(page.getByText("The Manager")).toBeVisible();
  await expect(page.getByText("The Promoter")).toBeVisible();
  await expect(page.getByText("The Salesperson")).toBeVisible();
  await expect(page.getByText("The Ambassador")).toBeVisible();
  await expect(page.getByText("The Accountant")).toBeVisible();
  await expect(page.getByText("The Protector")).toBeVisible();
  await expect(page.getByText("The Advisor")).toBeVisible();
});

test("UI: Department card shows pending approval and opens ApprovalInbox", async ({
  page,
}) => {
  await page.route("**/api/agents/approvals", async (route) => {
    await route.fulfill({
      json: {
        pending_approvals: [
          {
            id: "mock-1",
            department: "CustomerSuccess",
            description: "Test request",
            status: "Pending",
            action_risk: "High",
          },
        ],
      },
    });
  });

  await page.goto("/team");

  const ambassadorCard = page.locator("button", { hasText: "The Ambassador" });
  await expect(ambassadorCard).toContainText("1 item awaiting approval");

  await ambassadorCard.click();

  await expect(page.locator("h1")).toContainText("The Ambassador");
  await expect(page.getByText("Test request")).toBeVisible();
  await expect(page.getByRole("button", { name: "Approve" })).toBeVisible();
});

test("UI: Approving a request updates the UI to All Caught Up", async ({
  page,
}) => {
  let postCalled = false;

  await page.route("**/api/agents/approvals", async (route, request) => {
    if (request.method() === "GET") {
      await route.fulfill({
        json: {
          pending_approvals: [
            {
              id: "mock-1",
              department: "CustomerSuccess",
              description: "Test request",
              status: "Pending",
              action_risk: "High",
            },
          ],
        },
      });
    } else {
      await route.fallback();
    }
  });

  await page.route("**/api/agents/approvals/mock-1", async (route) => {
    postCalled = true;
    await route.fulfill({ json: { success: true } });
  });

  await page.goto("/team");
  await page.locator("button", { hasText: "The Ambassador" }).click();

  await page.getByRole("button", { name: "Approve" }).click();

  await expect(page.getByText("All Caught Up!")).toBeVisible();
  expect(postCalled).toBe(true);
});

test("UI: Autonomous Global Localization flow", async ({ page }) => {
  let postCalled = false;

  await page.route("**/api/agents/approvals", async (route, request) => {
    if (request.method() === "GET") {
      await route.fulfill({
        json: {
          pending_approvals: [
            {
              id: "mock-mkt-1",
              department: "Marketing",
              description:
                "Global Reach: Translate your storefront to Spanish and show local currency for customers in Latin America?",
              status: "Pending",
              action_risk: "Medium",
              feature_type: "global_localization",
            },
          ],
        },
      });
    } else {
      await route.fallback();
    }
  });

  await page.route("**/api/agents/approvals/mock-mkt-1", async (route) => {
    postCalled = true;
    await route.fulfill({ json: { success: true } });
  });

  await page.goto("/team");

  const promoterCard = page.locator("button", { hasText: "The Promoter" });
  await expect(promoterCard).toContainText("1 item awaiting approval");
  await promoterCard.click();

  await expect(page.locator("h1")).toContainText("The Promoter");
  await expect(
    page.getByText(
      "Global Reach: Translate your storefront to Spanish and show local currency for customers in Latin America?",
    ),
  ).toBeVisible();

  // Assert the specific UI widget elements are visible
  await expect(page.getByText("Global Reach Preview")).toBeVisible();
  await expect(page.getByText("Original (EN)")).toBeVisible();
  await expect(page.getByText("Preview (ES)")).toBeVisible();
  await expect(page.getByText("Pastel Vegano")).toBeVisible();

  await page.getByRole("button", { name: "Approve" }).click();

  await expect(page.getByText("All Caught Up!")).toBeVisible();
  expect(postCalled).toBe(true);
});

test("UI: AI Visibility & GEO flow", async ({ page }) => {
  let postCalled = false;

  await page.route("**/api/agents/approvals", async (route, request) => {
    if (request.method() === "GET") {
      await route.fulfill({
        json: {
          pending_approvals: [
            {
              id: "mock-mkt-2",
              department: "Marketing",
              description:
                "Smart Search Setup: Make your store more visible to customers using AI search tools?",
              status: "Pending",
              action_risk: "Low",
              feature_type: "ai_geo",
            },
          ],
        },
      });
    } else {
      await route.fallback();
    }
  });

  await page.route("**/api/agents/approvals/mock-mkt-2", async (route) => {
    postCalled = true;
    await route.fulfill({ json: { success: true } });
  });

  await page.goto("/team");

  const promoterCard = page.locator("button", { hasText: "The Promoter" });
  await expect(promoterCard).toContainText("1 item awaiting approval");
  await promoterCard.click();

  await expect(page.locator("h1")).toContainText("The Promoter");
  await expect(
    page.getByText(
      "Smart Search Setup: Make your store more visible to customers using AI search tools?",
    ),
  ).toBeVisible();

  // Assert the specific UI widget elements are visible
  await expect(page.getByText("Smart Search Setup")).toBeVisible();
  await expect(page.getByText("Smart Formatting")).toBeVisible();
  await expect(page.getByText("Search Engine Data")).toBeVisible();
  await expect(page.getByText("Answer Formatting")).toBeVisible();

  await page.getByRole("button", { name: "Approve" }).click();

  await expect(page.getByText("All Caught Up!")).toBeVisible();
  expect(postCalled).toBe(true);
});

test("UI: Verify risk level UI representations", async ({ page }) => {
  await page.route("**/api/agents/approvals", async (route, request) => {
    if (request.method() === "GET") {
      await route.fulfill({
        json: {
          pending_approvals: [
            {
              id: "mock-risk-high",
              department: "Legal",
              description: "High Risk Action",
              status: "Pending",
              action_risk: "High",
            },
            {
              id: "mock-risk-low",
              department: "Legal",
              description: "Low Risk Action",
              status: "Pending",
              action_risk: "Low",
            },
          ],
        },
      });
    } else {
      await route.fallback();
    }
  });

  await page.goto("/team");
  await page.locator("button", { hasText: "The Protector" }).click();

  // Verify risk level badges
  const highRiskBadge = page.locator("span", { hasText: "High Risk" });
  const lowRiskBadge = page.locator("span", { hasText: "Low Risk" });

  await expect(highRiskBadge).toBeVisible();
  // Check the classes applied for high risk
  await expect(highRiskBadge).toHaveClass(/bg-orange-100/);
  await expect(highRiskBadge).toHaveClass(/text-orange-700/);

  await expect(lowRiskBadge).toBeVisible();
  // Check the classes applied for low risk
  await expect(lowRiskBadge).toHaveClass(/bg-blue-100/);
  await expect(lowRiskBadge).toHaveClass(/text-blue-700/);
});

test("UI: Proactive Tax & Legal Compliance Guardrails rejection flow", async ({
  page,
}) => {
  let postCalled = false;

  await page.route("**/api/agents/approvals", async (route, request) => {
    if (request.method() === "GET") {
      await route.fulfill({
        json: {
          pending_approvals: [
            {
              id: "mock-legal-2",
              department: "Legal",
              description:
                "Action Required: Your sales are approaching the EU tax limit. Should we update your tax and privacy policies to keep you compliant?",
              status: "Pending",
              action_risk: "High",
              feature_type: "legal_compliance",
            },
          ],
        },
      });
    } else {
      await route.fallback();
    }
  });

  await page.route("**/api/agents/approvals/mock-legal-2", async (route) => {
    postCalled = true;
    await route.fulfill({ json: { success: true } });
  });

  await page.goto("/team");
  await page.locator("button", { hasText: "The Protector" }).click();

  await expect(
    page.getByText(
      "Action Required: Your sales are approaching the EU tax limit. Should we update your tax and privacy policies to keep you compliant?",
    ),
  ).toBeVisible();

  // Assert the specific UI widget elements are visible
  await expect(page.getByText("Compliance Warning")).toBeVisible();
  await expect(
    page.getByText(
      "Sales are approaching €10,000. New tax rules require an updated Privacy Policy.",
    ),
  ).toBeVisible();

  await page.getByRole("button", { name: "Reject / Edit" }).click();

  await expect(page.getByText("All Caught Up!")).toBeVisible();
  expect(postCalled).toBe(true);
});

test("UI: Rejecting a request updates the UI to All Caught Up", async ({
  page,
}) => {
  let postCalled = false;

  await page.route("**/api/agents/approvals", async (route, request) => {
    if (request.method() === "GET") {
      await route.fulfill({
        json: {
          pending_approvals: [
            {
              id: "mock-2",
              department: "Operations",
              description: "Another request",
              status: "Pending",
              action_risk: "Low",
            },
          ],
        },
      });
    } else {
      await route.fallback();
    }
  });

  await page.route("**/api/agents/approvals/mock-2", async (route) => {
    postCalled = true;
    await route.fulfill({ json: { success: true } });
  });

  await page.goto("/team");
  await page.locator("button", { hasText: "The Manager" }).click();

  await page.getByRole("button", { name: "Reject / Edit" }).click();

  await expect(page.getByText("All Caught Up!")).toBeVisible();
  expect(postCalled).toBe(true);
});

test("UI: Department with no approvals shows All Caught Up directly", async ({
  page,
}) => {
  await page.route("**/api/agents/approvals", async (route) => {
    await route.fulfill({ json: { pending_approvals: [] } });
  });

  await page.goto("/team");

  await page.locator("button", { hasText: "The Accountant" }).click();

  await expect(page.locator("h1")).toContainText("The Accountant");
  await expect(page.getByText("All Caught Up!")).toBeVisible();
  await expect(
    page.getByText("There are no pending actions requiring your review."),
  ).toBeVisible();
});

test("UI: Proactive Tax & Legal Compliance Guardrails flow", async ({
  page,
}) => {
  let postCalled = false;

  await page.route("**/api/agents/approvals", async (route, request) => {
    if (request.method() === "GET") {
      await route.fulfill({
        json: {
          pending_approvals: [
            {
              id: "mock-legal-1",
              department: "Legal",
              description:
                "Action Required: Your sales are approaching the EU tax limit. Should we update your tax and privacy policies to keep you compliant?",
              status: "Pending",
              action_risk: "High",
              feature_type: "legal_compliance",
            },
          ],
        },
      });
    } else {
      await route.fallback();
    }
  });

  await page.route("**/api/agents/approvals/mock-legal-1", async (route) => {
    postCalled = true;
    await route.fulfill({ json: { success: true } });
  });

  await page.goto("/team");

  const protectorCard = page.locator("button", { hasText: "The Protector" });
  await expect(protectorCard).toContainText("1 item awaiting approval");
  await protectorCard.click();

  await expect(page.locator("h1")).toContainText("The Protector");
  await expect(
    page.getByText(
      "Action Required: Your sales are approaching the EU tax limit. Should we update your tax and privacy policies to keep you compliant?",
    ),
  ).toBeVisible();

  // Assert the specific UI widget elements are visible
  await expect(page.getByText("Compliance Warning")).toBeVisible();
  await expect(
    page.getByText(
      "Sales are approaching €10,000. New tax rules require an updated Privacy Policy.",
    ),
  ).toBeVisible();

  await page.getByRole("button", { name: "Approve" }).click();

  await expect(page.getByText("All Caught Up!")).toBeVisible();
  expect(postCalled).toBe(true);
});

test("UI: End-to-End CUJ - Order Placed event to Customer Success draft approval", async ({
  page,
  request,
}) => {
  // 1. Send the external webhook (simulated stripe order) to kick off backend routing
  const response = await request.post("/api/agents/webhook", {
    data: {
      tenant_id: "e2e-tenant",
      source: "stripe",
      message: "order_placed",
    },
  });
  expect(response.ok()).toBeTruthy();

  // Wait for the async backend event orchestration (Operations -> CustomerSuccess) to finish and create a draft
  // We'll repeatedly check the UI to see if the item is populated, avoiding arbitrary timeouts.

  // 2. User navigates to the Team dashboard
  await page.goto("/team");

  // Since async routing might take a moment, retry logic ensures we don't fail immediately
  await expect(
    page.locator("button", { hasText: "The Ambassador" }),
  ).toContainText("awaiting approval", { timeout: 10000 });

  // 3. User sees an action item in "The Ambassador" (Customer Success)
  const ambassadorCard = page.locator("button", { hasText: "The Ambassador" });
  // The exact number of pending approvals might fluctuate depending on parallel tests hitting the same tenant.
  // Instead of strict "1 item", let's just ensure there's AT LEAST one item in the card.
  await expect(ambassadorCard).toContainText("awaiting approval");
  await ambassadorCard.click();

  // 4. User views the draft. The operations agent triggers a "tenant.order.fulfillment_ready" event,
  // which causes the CustomerSuccess agent to generate "Send personalized thank you & shipping ETA".
  await expect(page.locator("h1")).toContainText("The Ambassador");

  // Find the specific approval card for this flow and approve it.
  const approvalCard = page
    .locator("div", { hasText: "Send personalized thank you & shipping ETA" })
    .first();
  await expect(approvalCard).toBeVisible();

  // 5. User 1-tap approves the draft
  await approvalCard.getByRole("button", { name: "Approve" }).click();

  // Wait a short time for network before verifying "All Caught Up!" to avoid flakiness
  // 6. User sees success state
  await expect(page.getByText("All Caught Up!")).toBeVisible({ timeout: 5000 });
});
