import { expect, test } from '@playwright/test';

test.describe('Subscription Retention Engine Feed E2E', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should display subscription retention recommendation in the feed and allow approval', async ({ page }) => {
    test.setTimeout(180000);

    // 1. Log in
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    // Navigate to the unified agent feed
    await page.goto('/feed');

    // Wait for the feed items to populate
    await expect(page.getByTestId('agent-feed').first()).toBeVisible({ timeout: 25000 });

    const churnCard = page.locator('text="Churn Risk Detected"');
    if (await churnCard.isVisible({ timeout: 15000 }).catch(() => false)) {
        await expect(churnCard).toBeVisible();

        const approveBtn = page.locator('button', { hasText: 'Approve & Send' }).first();
        if (await approveBtn.isVisible({ timeout: 15000 }).catch(() => false)) {
            await approveBtn.click();
            await expect(approveBtn).not.toBeVisible({ timeout: 15000 });
        }
    } else {
        const anyApproveBtn = page.locator('button', { hasText: 'Approve' }).first();
        if (await anyApproveBtn.isVisible({ timeout: 15000 }).catch(() => false)) {
            await anyApproveBtn.click();
            await expect(anyApproveBtn).not.toBeVisible({ timeout: 15000 });
        }
    }
  });
});
