import { test, expect } from '@playwright/test';

test.describe('Autonomous AI Cashflow Forecasting Engine', () => {

  test('Cashflow Dashboard Card displays shortfall and allows 1-tap resolution', async ({ page }) => {
    // Intercept the API call to return a mock shortfall to make test deterministic and isolated
    await page.route('/api/finance/forecast', async route => {
      const json = {
        forecast_cents: -35000,
        alert_message: "You might have a $350.00 shortfall next month. Let's resolve it.",
        type: "shortfall",
        current_balance: 10000,
        monthly_revenue_cents: 15000,
        monthly_expenses_cents: 60000
      };
      await route.fulfill({ json });
    });

    await page.route('/api/finance/resolve-gap', async route => {
      const json = {
        success: true,
        message: "Advance of $500 approved. Funds are available instantly."
      };
      await route.fulfill({ json });
    });

    // Navigate to the dashboard where the CashflowCard is rendered
    await page.goto('/dashboard');

    // Wait for the CashflowCard to load
    const cashflowCard = page.getByTestId('cashflow-card');
    await expect(cashflowCard).toBeVisible({ timeout: 15000 });

    // Verify shortfall alert is visible
    await expect(page.getByText(/shortfall next month/i)).toBeVisible();
    await expect(page.getByText('Send Invoice Reminders')).toBeVisible();
    await expect(page.getByText('Take Cash Advance')).toBeVisible();

    // The owner decides to take a cash advance to bridge the gap
    await page.getByText('Take Cash Advance').click();

    // Verify success message after resolving
    await expect(page.getByText(/Advance of \$500 approved/i)).toBeVisible();

    // Verify the alert message is no longer shown
    await expect(page.getByText(/shortfall next month/i)).toBeHidden();
  });

});
