import { test, expect } from './fixtures';

test.describe('Assistant Navigation & Routing', () => {
  test('should redirect onboarded user to assistant', async ({ page }) => {
    // Mock localStorage to simulate an onboarded user
    await page.goto('/');
    await page.evaluate(() => {
      localStorage.setItem('has_onboarded', 'true');
    });

    // Go to root again, should redirect to /assistant
    await page.goto('/');
    await expect(page).toHaveURL(/.*\/assistant/);
    await expect(page.getByRole('heading', { name: 'Jarvis Assistant' })).toBeVisible({ timeout: 15000 });
  });

  test('should redirect non-onboarded user to onboarding', async ({ page }) => {
    await page.goto('/');
    await page.evaluate(() => {
      localStorage.removeItem('has_onboarded');
    });

    await page.goto('/');
    await expect(page).toHaveURL(/.*\/onboarding/);
  });

  test('should display Assistant link in AppShell navigation', async ({ page }) => {
    await page.goto('/dashboard'); // Go somewhere with AppShell

    // Verify navigation link exists
    const assistantLink = page.getByRole('link', { name: 'Assistant' });
    await expect(assistantLink).toBeVisible({ timeout: 15000 });

    // Click it and verify navigation
    await assistantLink.click();
    await expect(page).toHaveURL(/.*\/assistant/);
    await expect(page.getByRole('heading', { name: 'Jarvis Assistant' })).toBeVisible({ timeout: 15000 });
  });
});
