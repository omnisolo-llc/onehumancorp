// category: "fabricated business payload"
import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.use({ storageState: { cookies: [], origins: [] } });

test.describe('Work Triage Flow', () => {
  adminPage('Owner can view and approve a triage item', async ({ page }) => {
    await page.goto('/ui/triage.html');
    await expect(page.locator('text=Needs Attention')).toBeVisible();

    const res = await page.request.post('/api/v1/webhook/triage/e2e-tenant', {
        data: {
            source: 'instagram',
            content: 'Urgent: I need a cake by tomorrow!'
        },
        headers: {
            'Authorization': 'Bearer default_secure_webhook_token_for_tests'
        }
    });
    expect(res.ok()).toBeTruthy();

    await page.reload();
    await expect(page.locator('text=Needs Attention')).toBeVisible();
    await expect(page.locator('text=instagram').first()).toBeVisible();

    await expect(page.getByTestId(/triage-approve-/).first()).toBeVisible();
    const approveBtn = page.getByTestId(/triage-approve-/).first();

    await approveBtn.click();
    await expect(approveBtn).toBeHidden();
  });
});
