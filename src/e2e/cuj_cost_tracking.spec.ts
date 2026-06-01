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

  test('Owner checks current plan and views cost dashboard', async ({ page }) => {
    // Start from dashboard
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();

    // Check elements dynamically populated on My Plan screen by clicking "Billing"
    await page.getByRole('button', { name: 'Billing', exact: true }).click();

    // Verify My Plan Screen
    await expect(page.locator('#my-plan-screen')).toBeVisible();
    await expect(page.getByRole('heading', { name: 'My Plan' })).toBeVisible();

    // Check elements dynamically populated
    await expect(page.locator('#my-plan-name')).toContainText('Plan:');
    await expect(page.locator('#my-plan-next-bill')).toContainText('Estimated Next Bill:');

    // View Cost Details
    await page.getByRole('button', { name: 'View Cost Details' }).click();

    // Verify Cost Dashboard Screen
    await expect(page.locator('#cost-dashboard-screen')).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Cost Transparency Dashboard' })).toBeVisible();

    // Verify dynamic metrics are populated
    await expect(page.locator('#cost-dashboard-period')).toContainText('Period:');
    await expect(page.locator('#cost-dashboard-total')).toBeVisible();
    await expect(page.locator('#cost-dashboard-llm')).toBeVisible();

    // Back to My Plan
    await page.getByRole('button', { name: 'Back to My Plan' }).click();
    await expect(page.locator('#my-plan-screen')).toBeVisible();

    // View Upgrade Plans
    await page.getByRole('button', { name: 'View Upgrade Plans' }).click();

    // Verify Pricing Screen
    await expect(page.locator('#pricing-screen')).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Pricing Plans' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Free' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Starter' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Pro' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Business' })).toBeVisible();
  });
});
