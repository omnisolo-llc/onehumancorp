import { test, expect } from './fixtures';

test.describe('Social Media Autoposting Flow', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });

  test('user can open referral sharing tools', async ({ page }) => {
    await page.getByRole('button', { name: 'Referrals' }).click();

    await expect(page.getByRole('heading', { name: 'Referral Dashboard' })).toBeVisible();
    await expect(page.getByText('Share Tools')).toBeVisible();
  });

  test('user can trigger Instagram sharing', async ({ page }) => {
    page.on('dialog', async dialog => {
      expect(dialog.message()).toContain('Sharing to IG');
      await dialog.accept();
    });

    await page.getByRole('button', { name: 'Referrals' }).click();
    await page.getByRole('button', { name: /Share to Instagram/ }).click();
  });

  test('user can copy the invite message', async ({ page }) => {
    await page.getByRole('button', { name: 'Referrals' }).click();
    await page.getByRole('button', { name: /Copy Invite Message/ }).click();

    await expect(page.getByText('Invite message copied!')).toBeVisible();
  });

  test('user can configure Manychat from dashboard integrations', async ({ page }) => {
    await page.getByRole('button', { name: 'Integrations' }).click();
    await expect(page.getByRole('heading', { name: /Manychat/ })).toBeVisible();
    await page.locator('#manychat-integration').getByRole('button', { name: 'Configure' }).click();

    await expect(page.getByRole('heading', { name: 'Customer Inbox' })).toBeVisible();
  });

  test('user can respond to a social inbox message', async ({ page }) => {
    await page.getByRole('button', { name: 'Check Messages' }).click();
    await expect(page.getByText('Facebook User')).toBeVisible();
    await expect(page.getByText('Instagram User')).toBeVisible();
    await expect(page.getByText('WhatsApp User')).toBeVisible();

    await page.locator('#reply-input').fill('Thanks for reaching out!');
    await page.getByRole('button', { name: 'Send' }).click();

    await expect(page.locator('#messages-list')).toContainText('Thanks for reaching out!');
  });
});
