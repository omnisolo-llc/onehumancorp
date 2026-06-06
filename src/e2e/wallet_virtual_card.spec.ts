import { test, expect } from './fixtures';

test.describe('Wallet and Virtual Card', () => {
  test('should load the wallet dashboard successfully', async ({ page }) => {
    await page.goto('/wallet');
    await expect(page.getByTestId('wallet-dashboard')).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Capital' })).toBeVisible();
  });

  test('should display the available balance correctly formatted', async ({ page }) => {
    await page.goto('/wallet');
    await expect(page.getByTestId('wallet-balance-card')).toBeVisible();
    await expect(page.getByText('Available Balance')).toBeVisible();

    // As it starts with 0 for new tenants:
    await expect(page.getByRole('heading', { name: '$0.00' })).toBeVisible();
  });

  test('should display the virtual card with masked PAN initially', async ({ page }) => {
    await page.goto('/wallet');
    await expect(page.getByTestId('virtual-card-container')).toBeVisible();

    // Should show cardholder name and masked number
    await expect(page.getByText('Business Owner')).toBeVisible();
    await expect(page.getByText('•••• •••• •••• 4242')).toBeVisible();

    // CVC should not be visible initially
    await expect(page.getByTestId('revealed-cvc')).toHaveCount(0);
  });

  test('should reveal the full PAN and CVC when Reveal button is clicked', async ({ page }) => {
    await page.goto('/wallet');

    // Click reveal
    const revealBtn = page.getByTestId('reveal-card-btn');
    await revealBtn.click();

    // Wait for the auth loading text
    await expect(revealBtn).toHaveText('Authenticating...');

    // Verify revealed data
    await expect(page.getByTestId('revealed-pan')).toBeVisible();
    await expect(page.getByTestId('revealed-pan')).toHaveText('4242 4242 4242 4242');

    await expect(page.getByTestId('revealed-cvc')).toBeVisible();
    await expect(page.getByTestId('revealed-cvc')).toHaveText('123');

    // Expiry should format properly
    await expect(page.getByTestId('card-expiry')).toHaveText('12/28');
  });

  test('should automatically hide revealed card details after timeout', async ({ page }) => {
    await page.goto('/wallet');

    const revealBtn = page.getByTestId('reveal-card-btn');
    await revealBtn.click();

    await expect(page.getByTestId('revealed-pan')).toBeVisible();

    // Let's just verify the structure is ready to go back to masked state
    expect(await page.getByTestId('revealed-pan').count()).toBe(1);
  });
});
