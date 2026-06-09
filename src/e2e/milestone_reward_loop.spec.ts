import { test, expect } from './fixtures';

test.describe('Milestone-based Viral Reward Loop', () => {
  test('User can reach a milestone, share it, and claim Pro features', async ({ page }) => {
    // 1. Navigate to dashboard
    await page.goto('/dashboard');

    // 2. Success Milestone Alert should be visible (seeded with first_sale)
    const alert = page.locator('[data-testid="success-milestone-alert"]');
    await expect(alert).toBeVisible({ timeout: 15000 });
    await expect(alert).toContainText('First Sale!');

    // 3. Click Claim Reward
    const claimBtn = page.locator('[data-testid="milestone-share-btn"]');
    await expect(claimBtn).toContainText('Claim 7-day Pro Extension');
    await claimBtn.click();

    // 4. Should navigate to Milestones page
    await expect(page).toHaveURL(/\/milestones\?claim=first_sale/);
    await expect(page.locator('h1')).toContainText('Success Milestones');

    // 5. Preview card should be visible
    await expect(page.locator('img[alt*="First Sale!"]')).toBeVisible();

    // 6. Click Share & Unlock
    await page.addInitScript(() => {
        (window as any).open = () => null;
        (window as any).alert = (msg: string) => console.log('Alert:', msg);
    });

    const shareAndUnlockBtn = page.locator('button', { hasText: 'Share & Unlock 7-day Pro Extension' });
    await expect(shareAndUnlockBtn).toBeVisible();
    await shareAndUnlockBtn.click();

    // 7. Success state should show "Active"
    await expect(page.locator('div', { hasText: '7-day Pro Extension Active' })).toBeVisible({ timeout: 15000 });

    // 8. Go back to dashboard and verify alert is gone (as it's now claimed)
    await page.goto('/dashboard');
    await expect(page.locator('[data-testid="success-milestone-alert"]')).not.toBeVisible();
  });

  test('Milestones page responsiveness at 375px', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/milestones');

    await expect(page.locator('h1')).toBeVisible();
    const main = page.locator('main');
    await expect(main).toHaveClass(/flex-col/);
  });
});
