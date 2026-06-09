import { test, expect } from '@playwright/test';

test.describe('Cost Dashboard Interaction Loop', () => {
  test('Cost dashboard interaction links and buttons verify', async ({ page }) => {
    // Navigate to the dashboard page
    await page.goto('/cost-dashboard');

    // Wait for the main heading to appear, indicating successful load
    await expect(page.locator('h1', { hasText: 'Business Advisory Dashboard' })).toBeVisible({ timeout: 10000 });

    // Verify hover interaction on the "Back to My Plan" button changes its visual state if needed
    // However, playwright handles clicking directly
    const backToPlanBtn = page.locator('button', { hasText: 'Back to My Plan' });
    await expect(backToPlanBtn).toBeVisible();
    await expect(backToPlanBtn).toBeEnabled();

    // Verify Cost Breakdown components existence and interactability (even if they are just static display for now, checking they render properly)
    const costBreakdownSection = page.locator('h2', { hasText: 'Cost Breakdown' });
    await expect(costBreakdownSection).toBeVisible();

    const llmUsageElement = page.locator('span', { hasText: 'LLM Usage' });
    await expect(llmUsageElement).toBeVisible();

    const storageElement = page.locator('span', { hasText: 'Storage' });
    await expect(storageElement).toBeVisible();

    // Click Back to My Plan and verify navigation
    await backToPlanBtn.click();
    await expect(page).toHaveURL('/plan');
  });
});
