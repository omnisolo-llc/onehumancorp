import { test, expect } from '@playwright/test';

test.describe('Viral Waitlist Loop', () => {
  test('should allow user to join waitlist and share', async ({ page }) => {
    // Navigate to waitlist page
    await page.goto('/waitlist');

    // Fill out the form
    await page.fill('input[id="email"]', 'test@example.com');

    // Submit the form
    await Promise.all([
      page.waitForResponse(resp => resp.url().includes('/api/v1/growth/waitlist') && resp.status() === 200),
      page.click('button[type="submit"]')
    ]);

    // Verify success message
    await expect(page.locator("text=You're on the list!")).toBeVisible();
  });
});
