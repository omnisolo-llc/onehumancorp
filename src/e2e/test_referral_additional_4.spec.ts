import { test, expect } from './fixtures';


test('second copy invite message works with unique ID', async ({ page, context }) => {
  await page.goto('/');
  await page.evaluate(() => localStorage.setItem('tenant_id', 'invite-tenant-789'));
  await page.evaluate(() => (window as any).showScreen('referral-dashboard-screen'));

  await context.grantPermissions(['clipboard-read', 'clipboard-write']);

  // Click the third copy button (Copy Invite Message - the second one on the page)
  const copyInviteBtn2 = page.locator('button:has-text("Copy Invite Message")').nth(1);
  await copyInviteBtn2.click();

  const clip = await page.evaluate(() => navigator.clipboard.readText());
  expect(clip).toBe('Join OHC using my link! ohc://join?ref=invite-tenant-789');

  // Verify msg displayed
  await expect(page.locator('#invite-copied-msg-2')).toBeVisible();
});
