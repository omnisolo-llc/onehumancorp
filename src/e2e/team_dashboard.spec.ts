import { test, expect } from './fixtures';

test.describe('🎨 Canvas: Team Dashboard UI & Approvals', () => {
  test('CUJ 1: Render team page and show departments', async ({ page }) => {
    // Navigate to team dashboard
    await page.goto('/team');

    // Check main title
    await expect(page.getByRole('heading', { name: 'Your Team' })).toBeVisible();

    // The Manager (Operations)
    await expect(page.getByRole('heading', { name: 'The Manager' })).toBeVisible();

    // The Promoter (Marketing)
    await expect(page.getByRole('heading', { name: 'The Promoter' })).toBeVisible();

    // Verify Daily Brief is present
    await expect(page.getByRole('heading', { name: 'Daily Brief' })).toBeVisible();
  });

  test('CUJ 2: Test Department navigation', async ({ page }) => {
    await page.goto('/team');
    await page.waitForLoadState('networkidle');

    const theManagerButton = page.getByRole('button').filter({ hasText: /The Manager/ });
    await theManagerButton.click();

    await expect(page.getByRole('button', { name: 'Back to Team' })).toBeVisible({ timeout: 10000 });
    await expect(page.getByRole('heading', { name: 'The Manager' })).toBeVisible();
  });

  test('CUJ 3: Test Department fallback text when empty', async ({ page }) => {
    await page.goto('/team');
    await page.waitForLoadState('networkidle');

    const theManagerButton = page.getByRole('button').filter({ hasText: /The Manager/ });
    await theManagerButton.click();

    // It should display 'Inbox Zero' when there are no approvals
    await expect(page.getByText('Inbox Zero')).toBeVisible();
  });

  test('CUJ 4: Verify chat functionality navigation', async ({ page }) => {
    await page.goto('/team');
    await page.waitForLoadState('networkidle');

    const chatButton = page.getByRole('button', { name: 'Team Chat' });
    await chatButton.click();

    await expect(page).toHaveURL(/.*team\/chat/);
  });

  test('CUJ 5: Test API fallback behavior', async ({ page }) => {
    await page.goto('/team');
    await expect(page.getByRole('heading', { name: 'Daily Brief' })).toBeVisible();
  });
});
