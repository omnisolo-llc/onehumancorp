import { test, expect } from './fixtures';
import * as path from 'path';
import * as fs from 'fs';

test.describe('CUJ: Billing Cost Tracking', () => {
  test('Owner navigates to the My Plan page and sees the active plan details', async ({ page }) => {
    await page.goto('/plan');
    await expect(page.locator('#my-plan-screen')).toBeVisible();
    await expect(page.locator('#my-plan-name')).toContainText('Plan:');
    await expect(page.locator('#my-plan-next-bill')).toContainText('Estimated Next Bill:');
    await expect(page.getByRole('button', { name: 'Upgrade Plan' })).toBeVisible();
  });

  test('Owner verifies the usage section showing AI actions and storage limits', async ({ page }) => {
    await page.goto('/plan');
    await expect(page.locator('text=Your Current Usage')).toBeVisible();
    await expect(page.locator('text=AI Actions Used')).toBeVisible();
    await expect(page.locator('text=Storage Used')).toBeVisible();
  });

  test('Owner navigates to the Cost Dashboard by clicking the "View Cost Details" button', async ({ page }) => {
    await page.goto('/plan');
    await page.getByRole('button', { name: 'View Cost Details' }).click();
    await expect(page).toHaveURL(/.*\/cost-dashboard/);
    await expect(page.locator('#cost-dashboard-screen')).toBeVisible();
    await expect(page.locator('h1')).toContainText('Cost Transparency Dashboard');
  });

  test('Owner sees the breakdown of total costs, LLM usage, storage, and payment fees on the Cost Dashboard', async ({ page }) => {
    await page.goto('/cost-dashboard');
    await expect(page.locator('#cost-dashboard-screen')).toBeVisible();
    await expect(page.locator('#cost-dashboard-period')).toBeVisible();
    await expect(page.locator('#cost-dashboard-total')).toBeVisible();
    await expect(page.locator('#cost-dashboard-llm')).toBeVisible();
    await expect(page.locator('text=Storage')).toBeVisible();
    await expect(page.locator('text=Payment Fees')).toBeVisible();
  });

  test('Owner successfully navigates back to My Plan from the Cost Dashboard using the back button', async ({ page }) => {
    await page.goto('/cost-dashboard');
    await expect(page.locator('#cost-dashboard-screen')).toBeVisible();
    await page.getByRole('button', { name: 'Back to My Plan' }).click();
    await expect(page).toHaveURL(/.*\/plan/);
    await expect(page.locator('#my-plan-screen')).toBeVisible();
  });
});
