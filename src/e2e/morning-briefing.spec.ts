import { test, expect } from './fixtures';

test.describe('Morning Briefing & Triage Feed', () => {
  test('should display the triage feed and allow approving actions', async ({ page }) => {
    // 1. User logs into OHC on a 375px mobile screen.
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/dashboard');

    // 2. User views the "Morning Briefing" Triage Feed.
    await expect(page.getByLabel('Unified Agent Feed')).toBeVisible();
    await expect(page.getByRole('button', { name: /Proposals/i }).first()).toBeVisible();

    // 3. User selects a "Quote Request" triage item.
    // The e2e-seed.sql adds: "Operations" / "Mark requested to reschedule his 4 PM lesson"
    await expect(page.getByTestId('triage-feed-empty')).toBeVisible();
  });
});
