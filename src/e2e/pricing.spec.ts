import { test, expect } from './fixtures';

test.describe('Pricing Page', () => {
  test('should display all pricing tiers', async ({ page }) => {
    await page.goto('/pricing');
    await expect(page.locator('#pricing-screen')).toBeVisible();

    await expect(page.locator('h3', { hasText: 'Free' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Starter' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Pro' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Business' })).toBeVisible();
  });

  test('should display prices correctly', async ({ page }) => {
    await page.goto('/pricing');

    await expect(page.locator('p', { hasText: '$0' })).toBeVisible();
    await expect(page.locator('p', { hasText: '$29' })).toBeVisible();
    await expect(page.locator('p', { hasText: '$79' })).toBeVisible();
    await expect(page.locator('p', { hasText: '$299' })).toBeVisible();
  });

  test('should display specific features for Starter plan', async ({ page }) => {
    await page.goto('/pricing');

    const starterCard = page.locator('div', { has: page.locator('h3', { hasText: 'Starter' }) }).first();
    await expect(starterCard.locator('li', { hasText: '3 Agents Limit' })).toBeVisible();
    await expect(starterCard.locator('li', { hasText: '1,000 AI actions / month' })).toBeVisible();
    await expect(starterCard.locator('li', { hasText: '5GB Storage Quota' })).toBeVisible();
    await expect(starterCard.locator('li', { hasText: '100 Products Limit' })).toBeVisible();
  });

  test('should display specific features for Business plan', async ({ page }) => {
    await page.goto('/pricing');

    const businessCard = page.locator('div', { has: page.locator('h3', { hasText: 'Business' }) }).first();
    await expect(businessCard.locator('li', { hasText: 'Unlimited Agents' })).toBeVisible();
    await expect(businessCard.locator('li', { hasText: 'Unlimited AI actions' })).toBeVisible();
    await expect(businessCard.locator('li', { hasText: '500GB Storage Quota' })).toBeVisible();
    await expect(businessCard.locator('li', { hasText: 'Unlimited Products' })).toBeVisible();
  });

  test('should navigate to checkout on upgrade click', async ({ page }) => {
    await page.goto('/pricing');

    const starterButton = page.locator('button', { hasText: 'Upgrade to Starter via Stripe' });
    await starterButton.click();

    await page.waitForURL(/\/checkout\?tier=Starter/);
    expect(page.url()).toContain('/checkout?tier=Starter');
  });
});
