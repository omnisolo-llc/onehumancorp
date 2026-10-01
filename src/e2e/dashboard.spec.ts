import { test, expect } from './fixtures';

test.describe('Dashboard Core', () => {
  test('loads the dashboard and business snapshot', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
    await expect(page.getByText('Total Sales').first()).toBeVisible({ timeout: 15000 });
    await expect(page.getByRole('heading', { name: 'Business Analytics' })).toBeVisible();

    // Assert Growth Hub is present
    await expect(page.getByRole('heading', { name: 'Growth & Virality' })).toBeVisible();
    await expect(page.getByRole('link', { name: /Referrals/i }).first()).toBeVisible();
    await expect(page.getByRole('link', { name: /Milestones/i })).toBeVisible();
  });

  test('navigates to login and agents screens', async ({ anonymousPage, page }) => {
    await anonymousPage.goto('/login');
    await expect(anonymousPage.getByRole('heading', { name: 'Login' })).toBeVisible();

    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'AI Departments' })).toBeVisible();
  });

  test('opens setup from dashboard quick actions', async ({ page }) => {
    await page.goto('/dashboard');
    await page.getByRole('button', { name: 'Launch Site' }).click();
    await expect(page).toHaveURL(/\/onboarding\/?(?:\?.*)?$/);
    // The verified owner's saved draft may resume beyond the welcome screen.
    // This heading is mounted only after the owned setup state has loaded.
    await expect(page.getByRole('heading', { name: 'Setup', exact: true, level: 1 })).toBeVisible();
    await expect(page.getByRole('status', { name: 'Loading onboarding' })).toHaveCount(0);
  });

  test('opens the proposal editor from its dashboard card', async ({ page }) => {
    await page.goto('/dashboard');
    await page.getByRole('link', { name: /Proposals.*Proposal Draft/i }).click();
    await expect(page).toHaveURL(/\/proposals\/new\/?(?:\?.*)?$/);
    await expect(page.getByRole('heading', { name: 'AI Proposal Generator', exact: true })).toBeVisible();
    await expect(page.getByRole('textbox', { name: 'Project Brief / Topic' })).toBeEditable();
  });

  test('opens and dismisses dashboard quick actions', async ({ page }) => {
    await page.goto('/dashboard');
    const toggle = page.getByRole('button', { name: 'Quick Actions', exact: true });
    await expect(page.getByRole('link', { name: '📦 New Product', exact: true })).toHaveCount(0);
    await toggle.click();
    await expect(page.getByRole('link', { name: '📦 New Product', exact: true })).toHaveAttribute('href', '/products/new');
    await expect(page.getByRole('link', { name: /Snap Receipt/ })).toBeVisible();
    await toggle.click();
    await expect(page.getByRole('link', { name: '📦 New Product', exact: true })).toHaveCount(0);
  });
});
