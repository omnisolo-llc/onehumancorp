import { test, expect } from '@playwright/test';
import { memberPage as page } from './fixtures';

test('referral link defaults to DEFAULT when no tenant_id', async ({ page, context }) => {
  await page.goto('/');
  await page.evaluate(() => localStorage.removeItem('tenant_id'));
  await page.evaluate(() => (window as any).showScreen('referral-dashboard-screen'));

  // Verify link text
  await expect(page.locator('#referral-link')).toContainText('ohc://join?ref=DEFAULT');

  await context.grantPermissions(['clipboard-read', 'clipboard-write']);

  // Click the first copy button
  const copyBtn = page.getByText('Copy', { exact: true }).first();
  await page.evaluate(() => { window.alert = () => {}; });
  await copyBtn.click();

  const clip1 = await page.evaluate(() => navigator.clipboard.readText());
  expect(clip1).toBe('ohc://join?ref=DEFAULT');
});
