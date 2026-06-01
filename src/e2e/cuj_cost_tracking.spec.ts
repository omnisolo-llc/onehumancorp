import { test, expect } from './fixtures';

test.describe('CUJ: Billing Cost Tracking', () => {
  test('should display cost breakdown on dashboard locally', async ({ page }) => {
    // Navigate to Login Page
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();

    // Fill in the form and click login
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('testuser@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password');
    await page.locator('button:has-text("Login")').click();

    // Verify navigation to dashboard
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();

    // The dashboard contains a Link to /plan with text implicitly inside. Let's find it using href attribute
    await page.locator('a[href="/plan"]').click();
    await expect(page.getByRole('heading', { name: 'My Plan' })).toBeVisible();

    // From plan page, navigate to Cost Details as a real user would
    await page.getByRole('button', { name: 'View Cost Details' }).click();

    // Verify Cost Dashboard loaded
    await expect(page.getByRole('heading', { name: 'Business Advisory Dashboard' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Cost Transparency' })).toBeVisible();
    await expect(page.getByText('Total Costs')).toBeVisible();
    await expect(page.getByText('LLM Usage')).toBeVisible();
    await expect(page.getByText('Storage')).toBeVisible();
    await expect(page.getByText('Payment Fees')).toBeVisible();
  });
});
