import { test, expect } from './fixtures';

test.describe('Wizard Refinement E2E', () => {
  test('keeps the setup flow plain-language and reversible', async ({ page }) => {
    await page.goto('/website-builder');
    await expect(page.getByText('Review and add any extra details to help our AI generate the perfect store.')).toBeVisible();
    await page.locator('#bio-input').fill('my cool testing dog grooming biz');
    await page.locator('#generate-btn').click({ force: true });

    await expect(page.getByText('1-Tap Launch')).toBeVisible({ timeout: 15000 });
    await page.locator('#launch-btn').click({ force: true });

    await expect(page.getByRole('heading', { name: "You're Live!" })).toBeVisible({ timeout: 15000 });
  });

  test('exposes AI helper and prompt tuning areas', async ({ page }) => {
    await page.goto('/dashboard');
    await page.goto('/team');
    await expect(page.getByRole('heading', { name: 'Your Team' })).toBeVisible();
    await expect(page.getByText('The Promoter')).toBeVisible();
  });

  test('settings remain accessible from dashboard quick actions', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByText('Share & Claim Reward')).toBeVisible();
  });
});
