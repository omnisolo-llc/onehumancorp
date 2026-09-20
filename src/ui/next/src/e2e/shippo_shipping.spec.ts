import { test, expect } from "@playwright/test";

test.describe("Shippo Shipping Integration", () => {
  const authFile = "src/e2e/.auth/user.json";

  test.use({ storageState: authFile });

  test("should render the order details page and shipping section", async ({ page }) => {
    // Route mock for order details
    await page.route("**/api/v1/ui/orders", async (route) => {
      const json = [
        {
          id: "ord_123",
          customer_name: "Priya",
          total_amount: 15.0,
          status: "Ready",
          created_at: "2024-01-01T12:00:00Z",
        },
      ];
      await route.fulfill({ json });
    });

    await page.goto("/orders/ord_123");

    await expect(page.getByText("Order ord_123")).toBeVisible();
    await expect(page.getByRole("heading", { name: "Shipping" })).toBeVisible();
    await expect(page.getByLabel("Package weight in ounces")).toBeVisible();
    await expect(page.getByLabel("Package dimensions")).toBeVisible();
    await expect(page.getByRole("button", { name: "Get Shipping Rates" })).toBeVisible();
  });

  test("should show error for invalid dimensions", async ({ page }) => {
    // Route mock for order details
    await page.route("**/api/v1/ui/orders", async (route) => {
      const json = [{ id: "ord_123", status: "Ready" }];
      await route.fulfill({ json });
    });

    await page.goto("/orders/ord_123");

    await page.fill("input[aria-label='Package weight in ounces']", "-5");
    await page.fill("input[aria-label='Package dimensions']", "invalid");
    await page.click("button:has-text('Get Shipping Rates')");

    await expect(page.getByText("Enter a valid positive weight and dimensions such as 10x8x6.")).toBeVisible();
  });

  test("should fetch and display shipping rates", async ({ page }) => {
    // Route mock for order details
    await page.route("**/api/v1/ui/orders", async (route) => {
      const json = [{ id: "ord_123", status: "Ready" }];
      await route.fulfill({ json });
    });

    // Route mock for shipping rates
    await page.route("**/api/v1/shipping/rates", async (route) => {
      const json = {
        rates: [
          {
            id: "rate_abc",
            carrier: "USPS",
            service: "Priority Mail",
            amount: "7.50",
            days: 2,
          },
        ],
      };
      await route.fulfill({ json });
    });

    await page.goto("/orders/ord_123");

    await page.fill("input[aria-label='Package weight in ounces']", "16");
    await page.fill("input[aria-label='Package dimensions']", "10x8x6");
    await page.click("button:has-text('Get Shipping Rates')");

    await expect(page.getByText("USPS Priority Mail · 2 days")).toBeVisible();
    await expect(page.getByText("$7.50")).toBeVisible();
  });

  test("should purchase label successfully", async ({ page }) => {
    // Route mock for order details
    await page.route("**/api/v1/ui/orders", async (route) => {
      const json = [{ id: "ord_123", status: "Ready" }];
      await route.fulfill({ json });
    });

    // Route mock for shipping rates
    await page.route("**/api/v1/shipping/rates", async (route) => {
      const json = {
        rates: [
          {
            id: "rate_abc",
            carrier: "USPS",
            service: "Priority Mail",
            amount: "7.50",
            days: 2,
          },
        ],
      };
      await route.fulfill({ json });
    });

    // Route mock for shipping label
    await page.route("**/api/v1/shipping/label", async (route) => {
      const json = {
        success: true,
        labelUrl: "https://shippo-delivery.s3.amazonaws.com/label.pdf",
        trackingNumber: "1234567890",
        carrier: "USPS",
      };
      await route.fulfill({ json });
    });

    await page.goto("/orders/ord_123");

    await page.fill("input[aria-label='Package weight in ounces']", "16");
    await page.fill("input[aria-label='Package dimensions']", "10x8x6");
    await page.click("button:has-text('Get Shipping Rates')");

    await page.check("input[name='shipping-rate']");
    await page.click("button:has-text('Buy Label')");

    await expect(page.getByText("USPS tracking:")).toBeVisible();
    await expect(page.getByText("1234567890")).toBeVisible();
    await expect(page.getByRole("link", { name: "Open Shipping Label" })).toHaveAttribute("href", "https://shippo-delivery.s3.amazonaws.com/label.pdf");
  });

  test("should handle missing order gracefully", async ({ page }) => {
    // Route mock for order details - returning empty or no match
    await page.route("**/api/v1/ui/orders", async (route) => {
      const json = [{ id: "ord_999", status: "Ready" }];
      await route.fulfill({ json });
    });

    await page.goto("/orders/ord_123");

    await expect(page.getByText("This order was not found.")).toBeVisible();
  });
});
