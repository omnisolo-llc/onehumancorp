import { test, expect } from './fixtures';


test('referral link copies with correct tenant_id', async ({ page, context }) => {
  await page.goto('/');
  await page.evaluate(() => localStorage.setItem('tenant_id', 'copy-tenant-456'));
  await page.evaluate(() => (window as any).showScreen('referral-dashboard-screen'));

  // Ensure clip board permissions are given
  await context.grantPermissions(['clipboard-read', 'clipboard-write']);

  // Click the first copy button
  const copyBtn = page.getByText('Copy', { exact: true }).first();
  // Override alert to not block
  await page.evaluate(() => { window.alert = () => {}; });
  await copyBtn.click();

  // Verify clipboard content
  const clip1 = await page.evaluate(() => navigator.clipboard.readText());
  expect(clip1).toBe('ohc://join?ref=copy-tenant-456');

  // Click second copy button (Copy Invite Message)
  const copyInviteBtn = page.getByText('Copy Invite Message').first();
  await copyInviteBtn.click();
  const clip2 = await page.evaluate(() => navigator.clipboard.readText());
  expect(clip2).toBe('ohc://join?ref=copy-tenant-456');

  // Verify msg displayed
  await expect(page.locator('#invite-copied-msg')).toBeVisible();
});
