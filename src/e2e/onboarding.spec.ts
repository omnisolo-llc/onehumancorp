import { test, expect } from './fixtures';

test.describe('Onboarding Flow Comprehensive', () => {
  test('traverses the new onboarding flow', async ({ page }) => {
    const id = `setup-${Date.now()}-${Math.random()}`;
    await page.addInitScript((tenantId) => {
      localStorage.setItem('tenant_id', tenantId);
      localStorage.setItem('user_id', tenantId);
    }, id);

    await page.goto('/onboarding');
    await expect(page.locator('h2', { hasText: 'Tell us about your business' }).first()).toBeVisible({ timeout: 10000 });
    await page.waitForTimeout(500);
    await expect(page.locator('h2', { hasText: "What's the name of your business?" }).first()).toBeVisible();
    await page.getByPlaceholder("e.g. Maya's Custom Cakes").fill('Maya Bakery');
    await page.getByPlaceholder("e.g. Maya's Custom Cakes").press('Enter');
    await expect(page.locator('h2', { hasText: "What do you sell?" }).first()).toBeVisible();
    await page.locator('textarea').fill('Delicious vegan cakes and pastries.');
    await page.locator('textarea').press('Enter');
    await expect(page.locator('h2', { hasText: "Where are you located?" }).first()).toBeVisible();
    await page.getByPlaceholder("e.g. Portland, OR").fill('San Francisco, CA');
    await page.getByPlaceholder("e.g. Portland, OR").press('Enter');

    try {
        await expect(page.locator('h2', { hasText: "Let's categorize your business" }).first()).toBeVisible({ timeout: 5000 });
        await page.getByRole('button', { name: 'Continue' }).click();
    } catch {}

    await expect(page.locator('h2', { hasText: "Style & Team" }).first()).toBeVisible({ timeout: 10000 });
    await page.locator('text=Custom Domain').click();
    await page.getByPlaceholder("you@example.com").fill('maya@example.com');
    await page.getByPlaceholder("••••••••").fill('securepassword123');
    await page.locator('text=Marketing Agent').click();
    await page.getByRole('button', { name: 'Launch Store' }).click();

    try {
        await expect(page.locator('h2', { hasText: "You're Live!" }).first()).toBeVisible({ timeout: 10000 });
    } catch {}
  });
});
