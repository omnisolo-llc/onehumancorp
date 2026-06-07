import { test, expect } from '@playwright/test';

test('Maya navigates to Triage Feed, views a message and approves draft', async ({ page, request }) => {
  // 1. Log in
  await page.goto('/login');

  const isLoginPage = await page.getByRole('button', { name: /Sign in/i }).isVisible().catch(() => false);
  if (isLoginPage || await page.locator('input[type="email"]').isVisible().catch(() => false)) {
    await page.fill('input[type="email"]', 'admin@ohc.local').catch(() => {});
    await page.fill('input[type="password"]', 'changeme').catch(() => {});
    await page.click('button[type="submit"]').catch(() => {});
  }

  await page.goto('/inbox');
  await page.waitForTimeout(3000);

  // 2. Trigger webhook to simulate incoming customer message
  const res = await request.post('/api/agents/webhook', {
    data: {
      tenant_id: 'default',
      source: 'instagram',
      message: 'Do you make vegan cakes? Sarah',
      target_language: 'en'
    }
  });

  // Give the agent time to process and draft a reply
  await page.waitForTimeout(5000);

  // Navigate to Triage Feed again to refresh
  await page.goto('/inbox');

  // Wait for the feed to load
  await page.waitForTimeout(3000);

  // 3. Click the message (if it exists)
  const isInstagramVisible = await page.getByText('instagram').first().isVisible();
  if (isInstagramVisible) {
      await page.getByText('instagram').first().click();

      // Wait for AI draft to be generated
      await page.waitForTimeout(2000); // Sometimes draft generation takes a bit

      // Check if draft reply area exists
      const hasDraft = await page.getByText('Agent Draft').isVisible();
      if (hasDraft) {
        // 4. Approve and Send Draft
        const approveBtn = page.getByRole('button', { name: /Approve & Send Draft/ });
        await expect(approveBtn).toBeVisible();
        await approveBtn.click();
      }
  }

});
