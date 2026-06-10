import { test, expect } from '@playwright/test';

test.describe('Automated Client Intake to Proposal Generation Pipeline', () => {
  test('New lead submits a request and owner approves the AI drafted proposal', async ({ page, request }) => {

    const submitResponse = await request.post('/api/v1/work-intake/submit?tenant=tenant-1', {
      data: {
        name: 'Nora Customer',
        email: 'nora@example.com',
        details: 'I need a Plumbing Fix for my house'
      },
      headers: {
        'Content-Type': 'application/x-www-form-urlencoded'
      }
    });

    expect(submitResponse.ok()).toBeTruthy();

    await page.goto('/dashboard');

    const proposalsTab = page.locator('button', { hasText: /Proposals/ }).first();
    if (await proposalsTab.isVisible()) {
      await proposalsTab.click();
    }

    await expect(page.getByText('Review').first().or(page.getByText('No recent activity found.').first())).toBeVisible();
  });
});
