import { test, expect } from "@playwright/test";

test.describe("OnboardingWizard CUJ", () => {
  test.beforeEach(async ({ page, context }) => {
    // Clear local storage to ensure fresh state
    await page.addInitScript(() => {
      window.localStorage.clear();
    });
  });

  test("User can complete onboarding via Zero-Click Chat Agent", async ({
    page,
  }) => {
    await page.route("**/*api/onboarding/chat*", async (route) => {
      const request = route.request();
      const postData = JSON.parse(request.postData() || "{}");
      const messages = postData.messages || [];

      if (messages.length <= 1) {
        await route.fulfill({
          status: 200,
          contentType: "application/json",
          body: JSON.stringify({
            is_complete: false,
            reply:
              "Great! Could you provide an example photo or a little more detail about what you sell?",
            intake_data: null,
          }),
        });
      } else {
        await route.fulfill({
          status: 200,
          contentType: "application/json",
          body: JSON.stringify({
            is_complete: true,
            reply: "[COMPLETE] Give me a minute... I'm building your business.",
            intake_data: {
              business_name: "Mock Business",
              business_type: "Mock Type",
              categories: ["physical"],
              initial_products: [{ name: "Mock Product", price: "10" }],
            },
          }),
        });
      }
    });

    await page.route("**/*api/onboarding/start*", async (route) => {
      await route.fulfill({
        status: 200,
        json: { organization_id: "test-org-123" },
      });
    });

    await page.route("**/*api/onboarding/launch*", async (route) => {
      await route.fulfill({ status: 200, json: {} });
    });

    await page.goto("/onboarding");

    // We start at step 0 which is the chat UI
    await expect(
      page.getByText("What do you want to build or manage today?"),
    ).toBeVisible();

    // Click the predefined chip
    await page.getByText("Cake Shop", { exact: true }).click();

    // Send another message
    await page.getByPlaceholder("Type a message...").fill("Yes");
    await page.getByRole("button", { name: "Send" }).click();

    // Verify it reached "You're Live"
    await expect(page.getByText("You're Live!", { exact: false })).toBeVisible({
      timeout: 15000,
    });
  });
});
