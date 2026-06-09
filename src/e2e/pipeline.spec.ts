import { test, expect } from './fixtures';

test.describe('Opportunity Pipeline Engine', () => {
  const tenantId = 'test-tenant';

  test('Owner reviews opportunities and changes their stage', async ({ page }) => {
    // Navigate to Pipeline
    await page.goto('/pipeline');

    // Wait for the pipeline page to load
    await expect(page.locator('h2').filter({ hasText: 'Deal Pipeline' })).toBeVisible();

    // Verify seed data appears correctly
    const brandingCard = page.locator('text=Branding Design');
    await expect(brandingCard).toBeVisible();
    await expect(page.locator('text=$1,500.00')).toBeVisible(); // 150000 cents

    const marketingCard = page.locator('text=Marketing Consultation');
    await expect(marketingCard).toBeVisible();
    await expect(page.locator('text=$500.00')).toBeVisible(); // 50000 cents

    // Change stage of "Branding Design" from "Proposal" to "Negotiation"
    const stageSelect = page.locator('[data-testid="stage-select-opp-test-1"]');
    await expect(stageSelect).toBeVisible();

    await stageSelect.selectOption('Negotiation');

    // Verify optimistic update logic
    await expect(stageSelect).toHaveValue('Negotiation');
  });
});
