import { test, expect } from './fixtures';

test.describe('Voice Agent Settings CUJ', () => {
  test('Persona: Business Owner can configure Voice Agent', async ({ page }) => {
    // 1. Owner navigates to dashboard
    await page.goto('/dashboard');
    await page.waitForLoadState('domcontentloaded');

    // 2. Find Voice Agent Settings Card and verify it renders
    const voiceCard = page.getByTestId('voice-agent-card');
    await voiceCard.waitFor({ state: 'attached', timeout: 30000 });

    // We expect the dashboard to have loaded and show the Voice Agent settings
    await expect(page.locator('h1').filter({ hasText: 'Dashboard' }).first()).toBeVisible();
    await expect(voiceCard.getByRole('heading', { name: 'Voice Agent' })).toBeVisible();
  });
});
