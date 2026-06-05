import { test, expect } from './fixtures';

test.describe('Pricing Page Dashboard', () => {

  test('should display the Pricing Page header and title', async ({ page }) => {
    await page.goto('/pricing');
    await expect(page.locator('h1', { hasText: 'Pricing Plans' })).toBeVisible({ timeout: 10000 });
  });

  test('should display Free Tier details correctly', async ({ page }) => {
    await page.goto('/pricing');

    const freeTierCard = page.locator('div', { hasText: 'Free' }).filter({ hasText: '$0' }).first();
    await expect(freeTierCard).toBeVisible();
    await expect(freeTierCard.locator('li', { hasText: '1 Agent Limit' })).toBeVisible();
    await expect(freeTierCard.locator('li', { hasText: '100 AI actions / month' })).toBeVisible();

    // Assert Downgrade button is present
    await expect(freeTierCard.getByRole('button', { name: 'Downgrade' })).toBeVisible();
  });

  test('should display Starter Tier details correctly and navigate to checkout on upgrade', async ({ page }) => {
    await page.goto('/pricing');

    const starterTierCard = page.locator('div', { hasText: 'Starter' }).filter({ hasText: '$29' }).first();
    await expect(starterTierCard).toBeVisible();
    await expect(starterTierCard.locator('li', { hasText: '3 Agents Limit' })).toBeVisible();
    await expect(starterTierCard.locator('li', { hasText: '1,000 AI actions / month' })).toBeVisible();

    const upgradeButton = starterTierCard.getByRole('button', { name: 'Upgrade to Starter via Stripe' });
    await expect(upgradeButton).toBeVisible();

    // Click and assert navigation pushes to checkout
    await upgradeButton.click();
    await expect(page).toHaveURL(/\/checkout\?tier=Starter/);
  });

  test('should display Pro Tier details correctly', async ({ page }) => {
    await page.goto('/pricing');

    const proTierCard = page.locator('div', { hasText: 'Pro' }).filter({ hasText: '$79' }).first();
    await expect(proTierCard).toBeVisible();
    await expect(proTierCard.locator('li', { hasText: '10 Agents Limit' })).toBeVisible();
  });

  test('should display Business Tier details correctly', async ({ page }) => {
    await page.goto('/pricing');

    const businessTierCard = page.locator('div', { hasText: 'Business' }).filter({ hasText: '$299' }).first();
    await expect(businessTierCard).toBeVisible();
    await expect(businessTierCard.locator('li', { hasText: 'Unlimited Agents' })).toBeVisible();
  });

  test('should trigger downgrade alert dialog when Downgrade is clicked', async ({ page }) => {
    await page.goto('/pricing');

    const freeTierCard = page.locator('div', { hasText: 'Free' }).filter({ hasText: '$0' }).first();
    const downgradeButton = freeTierCard.getByRole('button', { name: 'Downgrade' });

    // Setup dialog handler
    let dialogMessage = '';
    page.on('dialog', async dialog => {
      dialogMessage = dialog.message();
      await dialog.accept();
    });

    await downgradeButton.click();
    expect(dialogMessage).toBe('You are downgrading to Free.');
  });
});
