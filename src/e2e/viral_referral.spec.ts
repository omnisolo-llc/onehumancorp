import { test, expect } from './fixtures';

test.describe('Viral Referral Loop', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();

    await page.getByRole('button', { name: 'Referrals' }).click();
    await expect(page.getByRole('heading', { name: 'Referral Dashboard' })).toBeVisible();
  });

  test('should display the referral link widget', async ({ page }) => {
    await expect(page.getByText('Your Referral Link')).toBeVisible();
    await expect(page.locator('#referral-link')).toContainText('https://ohc.app/join?ref=DEFAULT');
  });

  test('should copy referral link', async ({ page }) => {
    page.on('dialog', async dialog => {
      expect(dialog.message()).toContain('Copied');
      await dialog.accept();
    });

    await page.getByRole('button', { name: 'Copy', exact: true }).click();
  });

  test('should expose invite message sharing', async ({ page }) => {
    await page.getByRole('button', { name: /Copy Invite Message/ }).click();
    await expect(page.getByText('Invite message copied!')).toBeVisible();
  });

  test('should keep referral actions reachable', async ({ page }) => {
    await expect(page.getByRole('button', { name: /View Referral Logs/ })).toBeVisible();
    await expect(page.getByRole('button', { name: /Export Data/ })).toBeVisible();
  });

  test('should handle mobile referral layout', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });

    await expect(page.getByRole('heading', { name: 'Referral Dashboard' })).toBeVisible();
    const invite = page.getByRole('button', { name: /Copy Invite Message/ });
    await expect(invite).toBeVisible();
    const box = await invite.boundingBox();
    if (box) {
      expect(box.width).toBeLessThanOrEqual(375);
    }
  });
});
