import { test, expect } from '@playwright/test';
import { setupAuthContext, cleanupAuthContext } from './fixtures';

test.describe('Zero Click Onboarding Flow - Maya Persona', () => {
  let orgId: string;
  let userId: string;

  test.beforeEach(async () => {
    const ctx = await setupAuthContext();
    orgId = ctx.orgId;
    userId = ctx.userId;
  });

  test.afterEach(async () => {
    await cleanupAuthContext(orgId, userId);
  });

  test('Maya can generate a storefront from a single prompt', async ({ page }) => {
    await page.goto('/onboarding');
    await expect(page.getByText('Tell me about your business...')).toBeVisible();
    await page.getByTestId('zero-click-prompt').fill("I am Maya. I run a home bakery selling custom vegan cakes in Austin via Instagram. I need to take deposits for my custom orders.");
    await page.getByTestId('zero-click-launch-btn').click();
    await expect(page.getByText('Building Your Business...')).toBeVisible();
    await expect(page.getByText('You\'re Live!')).toBeVisible({ timeout: 15000 });
    await expect(page.getByText('Open Assistant')).toBeVisible();
    await expect(page.getByText('Preview Storefront')).toBeVisible();
  });
});
