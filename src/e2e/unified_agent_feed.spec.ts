import { test, expect } from '@playwright/test';

test.describe('Unified Agent Feed', () => {
<<<<<<< HEAD
  test('should display agent feed and allow interaction', async ({ page }) => {

    // Ensure we are using the seeded e2e tenant explicitly to fetch the seed data
    await page.addInitScript(() => {
      localStorage.setItem('tenant_id', 'e2e-tenant');
      localStorage.setItem('user_id', 'e2e-admin-user');
    });

    // Go to dashboard
=======
  test('navigates to unified agent feed from dashboard', async ({ page }) => {
>>>>>>> 097f2d4a (Resolves #23879)
    await page.goto('/dashboard');

    // Check that the link is present on the dashboard
    const feedLink = page.locator('a[href="/unified-agent-feed"]');
    await expect(feedLink).toBeVisible();

    // Click and navigate
    await feedLink.click();

    // Verify we are on the right page
    await expect(page).toHaveURL(/\/unified-agent-feed/);
    await expect(page.getByRole('heading', { name: 'Unified Agent Feed' })).toBeVisible();
  });

<<<<<<< HEAD
    // Verify we are on dashboard and the Unified Agent Feed is present
    await expect(page.locator('button', { hasText: 'Proposals' }).first()).toBeVisible();

    await expect(page.getByText(/All caught up!|Requires Review|Loading Agent Proposals/).first()).toBeVisible();
    await page.getByRole('button', { name: 'Activity Feed' }).click();
    await expect(page.getByRole('button', { name: 'Activity Feed' })).toBeVisible();
=======
  test('displays proposals fetched from the API', async ({ page }) => {
    await page.goto('/unified-agent-feed');

    // Wait for the loading state to disappear
    await expect(page.getByTestId('loading-state')).not.toBeVisible({ timeout: 10000 });

    // Check for proposal titles
    await expect(page.getByText('Launch Summer Promo Campaign')).toBeVisible();
    await expect(page.getByText('Low Stock: Premium Fertilizer')).toBeVisible();

    // Check for agent types
    await expect(page.getByText('Marketing Agent')).toBeVisible();
    await expect(page.getByText('Operations Agent')).toBeVisible();
  });

  test('allows reviewing drafts', async ({ page }) => {
    await page.goto('/unified-agent-feed');

    await expect(page.getByTestId('loading-state')).not.toBeVisible({ timeout: 10000 });
    const reviewButton = page.getByRole('button', { name: 'Review Drafts' });
    await expect(reviewButton).toBeVisible();
  });

  test('allows approving orders', async ({ page }) => {
    await page.goto('/unified-agent-feed');

    await expect(page.getByTestId('loading-state')).not.toBeVisible({ timeout: 10000 });
    const approveOrderButton = page.getByRole('button', { name: 'Approve Order ($450)' });
    await expect(approveOrderButton).toBeVisible();
  });

  test('allows ignoring alerts', async ({ page }) => {
    await page.goto('/unified-agent-feed');

    await expect(page.getByTestId('loading-state')).not.toBeVisible({ timeout: 10000 });
    const ignoreButton = page.getByRole('button', { name: 'Ignore for now' });
    await expect(ignoreButton).toBeVisible();
>>>>>>> 097f2d4a (Resolves #23879)
  });
});
