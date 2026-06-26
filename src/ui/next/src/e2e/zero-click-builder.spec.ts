import { test, expect } from '@playwright/test';

test.describe('Zero Click Builder Mobile Onboarding', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('User can generate a store with a single prompt', async ({ page, context }) => {
    // Navigate to the zero-click-builder page
    await page.goto('/zero-click-builder');

    // 1. Verify text
    await expect(page.getByText('Zero-Click Business Generator')).toBeVisible();

    // 2. Chat input
    const chatInput = page.getByPlaceholder('e.g. I am a home baker');
    await expect(chatInput).toBeVisible();

    await chatInput.fill('I am a home baker in Austin selling custom vegan cakes.');
    await chatInput.press('Enter');

    // Mock API response
    await context.route('**/api/v1/growth/zero-click-builder/generate', async route => {
      await new Promise(r => setTimeout(r, 2000));
      return route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          organization_id: 'test-org-123',
          user_id: 'test-user-123',
          name: 'Austin Vegan Cakes',
          url: 'https://austin-vegan-cakes.ohc.app'
        })
      });
    });

    // 5. Verify visually engaging loading state
    await expect(page.getByText('Building Your Business...')).toBeVisible();

    // 6. Verify completion & transition to live preview
    await expect(page.getByText('Your business is live!')).toBeVisible({ timeout: 15000 });

    // Verify auth/redirect handoff button
    const launchBtn = page.getByRole('button', { name: /Launch My Store/i });
    await expect(launchBtn).toBeVisible();

    // Test the button navigates to dashboard
    await launchBtn.click();
    await expect(page).toHaveURL(/\/dashboard/);
  });
});
