
import { test, expect } from './fixtures';
import { Client } from 'pg';

test.describe('Silent Ambassador Action Feed', () => {
  test('verify Silent Ambassador UI renders and handles incoming messages correctly', async ({ page }) => {
    // 1. Sign in
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.getByRole('button', { name: /Login|Sign In/i }).filter({ visible: true }).first().click();

    await page.waitForURL('**/*');

    await expect(page.getByText("Action Required")).toBeVisible({ timeout: 15000 });
    await expect(page.getByText("Customer Message")).toBeVisible({ timeout: 15000 });

    await expect(page.getByText("Do you have vegan options for birthday cakes?")).toBeVisible();

    // 4. Click the Approve & Send button
    const approveSendBtn = page.getByRole('button', { name: 'Approve & Send' });
    await expect(approveSendBtn).toBeVisible();
    await approveSendBtn.click();

    // 5. Verify it's removed from UI
    await expect(page.getByText("Do you have vegan options for birthday cakes?")).not.toBeVisible();
  });
});
