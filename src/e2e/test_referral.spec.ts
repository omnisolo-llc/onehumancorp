import { test, expect } from './fixtures';


test('referral link is properly updated based on tenant_id', async ({ page }) => {
  // Go to home and sign in
  await page.goto('/');

  // Navigate to referral-dashboard
  await page.evaluate(() => (window as any).showScreen('referral-dashboard-screen'));

  // The default seeded user's tenant_id might be set or we can set it
  await page.evaluate(() => localStorage.setItem('tenant_id', 'test-tenant-123'));
  await page.evaluate(() => (window as any).showScreen('referral-dashboard-screen')); // re-trigger to update link

  // Verify link text
  await expect(page.locator('#referral-link')).toContainText('ohc://join?ref=test-tenant-123');
});
