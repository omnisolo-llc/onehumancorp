import { test, expect } from '@playwright/test';

test('Finance Tax Vault E2E', async ({ page }) => {
  // Mock API calls
  await page.route('/api/v1/finance/balances', async (route) => {
    await route.fulfill({
      status: 200,
      contentType: 'application/json',
      body: JSON.stringify({
        balances: [
          { wallet_type: 'MAIN_BALANCE', balance_cents: 420000, currency: 'USD' },
          { wallet_type: 'TAX_VAULT', balance_cents: 85000, currency: 'USD' },
        ]
      })
    });
  });

  await page.goto('/dashboard');

  // Verify Dashboard elements
  await expect(page.getByText('Embedded Finance', { exact: true })).toBeVisible();
  await expect(page.getByText('Available Cash', { exact: true })).toBeVisible();
  await expect(page.getByText('$4,200.00', { exact: true })).toBeVisible();
  await expect(page.getByRole('heading', { name: 'Tax Vault' })).toBeVisible();
  await expect(page.getByText('$850.00', { exact: true })).toBeVisible();

  // Verify The Accountant agent
  await expect(page.getByText('The Accountant:', { exact: false })).toBeVisible();
  await expect(page.getByText("Carlos paid his $200 deposit for Tuesday's job.", { exact: false })).toBeVisible();
});
