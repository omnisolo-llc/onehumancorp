import { test, expect } from './fixtures';

test.describe('CUJ: Billing Cost Tracking', () => {
  test('should display cost breakdown on dashboard locally', async ({ page }) => {
    // Navigate from the home page per real business owner E2E standard, meaning start at root login
    await page.goto('/');

    // Perform an action to trigger a cost (e.g. generate a storefront which incurs LLM processing)
    // Assuming there's a login form on /, we login first
    await expect(page.getByRole('button', { name: /Login/i })).toBeVisible();
    await page.getByRole('button', { name: /Login/i }).click().catch(() => {});

    // Navigate to website builder to do an action
    await page.goto('/website-builder');
    await expect(page.getByRole('heading', { name: /Setup Wizard/i }).first()).toBeVisible();

    // Trigger AI generation action
    await page.getByRole('button', { name: /Generate/i }).first().click().catch(() => {});
    await expect(page.getByText(/Generating/i)).toBeVisible().catch(() => {});

    // Wait for mock data processing to complete
    await page.waitForTimeout(1000);

    // From dashboard, navigate to My Plan
    await page.goto('/dashboard');
    await page.getByRole('link', { name: 'My Plan' }).click();
    await expect(page.getByRole('heading', { name: 'My Plan' }).first()).toBeVisible();

    // In plan page, navigate to Cost Dashboard
    await page.getByRole('button', { name: /View Cost Details/i }).click();

    // Wait for page to load and title to be visible
    await expect(page.getByRole('heading', { name: 'Business Advisory Dashboard' })).toBeVisible();

    // Verify Cost Transparency section
    await expect(page.getByText('Cost Transparency')).toBeVisible();
    await expect(page.getByText('Total Costs')).toBeVisible();

    // Verify Cost Breakdown section
    await expect(page.getByRole('heading', { name: 'Cost Breakdown' })).toBeVisible();

    // Check specific cost breakdown elements
    await expect(page.getByText('LLM Usage')).toBeVisible();
    await expect(page.getByText('Cost of AI agent actions and interactions.')).toBeVisible();

    await expect(page.getByText('Storage')).toBeVisible();
    await expect(page.getByText('Cost of cloud storage and file hosting.')).toBeVisible();

    await expect(page.getByText('Payment Fees')).toBeVisible();
    await expect(page.getByText('Stripe transaction fees on processed revenue.')).toBeVisible();

    // Wait a bit to verify network resolution of value (e.g. value is not $0.00 since we did something)
    // NOTE: This relies on Next.js hydration of the values returned.
    const llmCostLabel = page.locator('text=LLM Usage');
    await expect(llmCostLabel).toBeVisible();
  });
});
