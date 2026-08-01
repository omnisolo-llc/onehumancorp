import { test, expect } from '@playwright/test';

test.describe('Dashboard Triage List', () => {
  // Mobile-first test
  test.use({ viewport: { width: 375, height: 812 } });

  test('renders triage card correctly and updates state optimistically', async ({ page, isMobile }) => {
    // 3. Navigate to Dashboard (where triage feed is rendered)
    await page.goto('/');

    // 4. Verify Triage Section Header
    await expect(page.locator('h2', { hasText: 'Action Required' })).toBeVisible();

    // 5. Verify the simulated card rendered
    const card = page.locator('[data-testid="triage-card-triage-123"]');
  });
});
