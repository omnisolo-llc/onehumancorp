import { test, expect } from '@playwright/test';

test.describe('Omnichannel Customer Memory Context (Mobile)', () => {
  test.use({ viewport: { width: 375, height: 667 } }); // Mobile-first constraint

  test('displays assistant memory with translucent glass styling and timeline', async ({ page }) => {
    // Navigating to the customer memory graph directly.
    await page.goto('/customer/memory-graph?tenantId=default-tenant-id&customerId=default-customer-id');

    // Wait for the memory card title to show
    const header = page.locator('h1', { hasText: 'Customer Context' });
    await expect(header).toBeVisible({ timeout: 10000 });

    // Assert the presence of the frosted glass component
    // Assuming the component has these classes for translucent glass layout.
    const memoryCard = page.locator('.backdrop-blur-\\[30px\\]').first();
    await expect(memoryCard).toBeVisible();
    await expect(memoryCard).toHaveCSS('backdrop-filter', /blur/);

    // Verify Agent summary is rendered
    const agentSummary = page.locator('h3', { hasText: 'Agent Summary' });
    await expect(agentSummary).toBeVisible();

    // Verify Timeline section is rendered
    const timelineHeader = page.locator('h2', { hasText: 'Timeline' });
    await expect(timelineHeader).toBeVisible();

    // The timeline should have some recorded interactions or empty state
    const interactions = page.locator('.group.is-active');
    const emptyState = page.locator('text=No interaction history found.');

    // Assert that we either show interactions or empty state truthfully.
    await expect(
        interactions.first().isVisible().then(v => v || emptyState.isVisible())
    ).resolves.toBeTruthy();
  });
});
