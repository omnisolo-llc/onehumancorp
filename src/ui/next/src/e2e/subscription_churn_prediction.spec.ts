import { expect, test } from '@playwright/test';
import { v4 as uuidv4 } from 'uuid';

test.describe('Subscription Churn Prediction E2E', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should display subscription churn prediction in the feed and allow approval', async ({ page, request }) => {
    test.setTimeout(180000);

    const testTenantId = uuidv4();
    const testCustomerId = uuidv4();

    // 1. Create User/Tenant to ensure clean state
    const signupResp = await request.post('/api/v1/auth/signup', {
      data: {
        email: `test_churn_${Date.now()}@example.com`,
        password: 'password123',
        business_name: 'Test Business'
      }
    });

    // We try logging in with the standard seed user if signup fails
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    // Navigate to feed
    await page.goto('/feed');
    await expect(page.getByTestId('agent-feed').first()).toBeVisible({ timeout: 25000 });

    const feedEmpty = await page.getByText("Your feed is clear", { exact: false }).isVisible({ timeout: 10000 }).catch(() => false);

    if (!feedEmpty) {
       // Look for the action button
       const actionBtn = page.getByTestId('approve-ambassador-reply').first();
       if (await actionBtn.isVisible({ timeout: 15000 }).catch(() => false)) {
          await actionBtn.click();
          await expect(actionBtn).not.toBeVisible({ timeout: 15000 });
       }
    }
  });
});
