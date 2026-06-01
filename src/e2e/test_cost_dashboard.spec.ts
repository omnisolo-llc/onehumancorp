import { expect, test } from './fixtures';

test('current embedded app smoke: test_cost_dashboard', async ({ page }) => {
    // Navigate from the home page per real business owner E2E standard
    // The previous test logic for this assumed the user started at `/` (which redirects)
    await page.goto('/');

    // Assuming there's a login form on /, we login first
    await expect(page.getByRole('button', { name: /Login/i })).toBeVisible();
    await page.getByRole('button', { name: /Login/i }).click().catch(() => {});

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
});
