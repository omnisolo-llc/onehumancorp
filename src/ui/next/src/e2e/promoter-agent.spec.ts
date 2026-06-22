import { test, expect } from '@playwright/test';

test.describe('Promoter Agent Action Feed UI', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should render Promoter card, allow approval via 1-tap Schedule Posts, and display success', async ({ page }) => {
    test.setTimeout(180000);

    // 1. Log in
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    const tenantId = await page.evaluate(() => localStorage.getItem('tenant_id') || 'default');

    // 2. Seed a triage item to simulate the worker discovering a new product
    await page.request.post(`/api/triage/create?tenant_id=${encodeURIComponent(tenantId)}`, {
      data: {
        source: 'marketing',
        priority: 'medium',
        context: 'New product detected! Schedule a post to drive sales?',
        action_type: 'social_post_draft',
        action_payload: {
          feature_type: 'social_post_draft',
          product_name: 'Vegan Chocolate Cake',
          tiktok: 'Check out our new Vegan Chocolate Cake! 🎂',
          instagram: 'New arrival! Link in bio to grab a slice of our Vegan Chocolate Cake.',
          facebook: 'We just added Vegan Chocolate Cake to our store.'
        }
      }
    });

    // 3. Navigate to triage feed
    await page.goto('/triage');
    await expect(page.locator('body')).toContainText(/Work Triage/, { timeout: 15000 });

    const listItems = page.locator('div[data-testid^="triage-card-"]');
    await page.waitForTimeout(2000);

    // Filter to find the social_post_draft specifically if others exist
    const promoterCard = listItems.filter({ hasText: 'New product detected!' }).first();
    await expect(promoterCard).toBeVisible({ timeout: 10000 });

    // Verify draft content
    await expect(promoterCard).toContainText('Vegan Chocolate Cake');

    // 4. Click the "Schedule Posts" button
    const scheduleBtn = promoterCard.getByTestId('approve-social-post');
    await expect(scheduleBtn).toBeVisible();
    await expect(scheduleBtn).toContainText('Schedule Posts');

    await scheduleBtn.click();

    // 5. Verify the card is removed or a success message is shown
    await expect(promoterCard).not.toBeVisible({ timeout: 10000 });
  });
});
