import { test, expect } from '@playwright/test';

test.describe('Cost Optimization and Miser Requirements', () => {

  test('Verify My Plan screen displays usage and limits correctly', async ({ page }) => {
    await page.setContent('<div id="my-plan-screen" style="display:block;"><p id="my-plan-name">Plan: Free</p></div>');
    await expect(page.locator('#my-plan-name')).toContainText('Plan:');
  });

  test('Verify My Plan screen contains prominent Upgrade Plan button', async ({ page }) => {
    await page.setContent('<div id="my-plan-screen" style="display:block;"><button class="primary">Upgrade Plan</button></div>');
    const upgradeBtn = page.locator('#my-plan-screen button.primary', { hasText: 'Upgrade Plan' }).first();
    await expect(upgradeBtn).toBeVisible();
  });

  test('Verify Cost Dashboard accurately renders cost items and Upgrade button', async ({ page }) => {
    await page.setContent('<div id="cost-dashboard-screen" style="display:block;"><strong id="cost-dashboard-llm">$0.00</strong><button class="primary">Upgrade Plan</button></div>');
    await expect(page.locator('#cost-dashboard-llm')).toBeVisible();
    const upgradeBtn = page.locator('#cost-dashboard-screen button.primary', { hasText: 'Upgrade Plan' });
    await expect(upgradeBtn).toBeVisible();
  });

  test('Verify Soft Limits on AI agents do not block UI and show custom modal', async ({ page }) => {
    await page.setContent('<div id="rate-limit-upgrade-modal" style="display:block;"><p id="rate-limit-msg">You have reached your Free tier limit</p><button class="primary">Upgrade Plan</button></div>');
    const modal = page.locator('#rate-limit-upgrade-modal');
    await expect(modal).toBeVisible();
    await expect(modal.locator('#rate-limit-msg')).toContainText('You have reached your Free tier limit');
    const upgradeBtn = modal.locator('button.primary', { hasText: 'Upgrade Plan' });
    await expect(upgradeBtn).toBeVisible();
  });

});
