import { test, expect } from '@playwright/test';

test.describe('Help Center & Documentation Features', () => {
  test('User can open AI Help Chat from Dashboard', async ({ page }) => {
    // 1. Start from Home Page
    await page.goto('http://localhost:3000');
    await page.waitForLoadState('networkidle');

    // 2. Mock Login Process (Wait for UI to appear)
    const getStartedBtn = page.getByRole('button', { name: /Get Started/i });
    if (await getStartedBtn.isVisible()) {
        await getStartedBtn.click();
    }

    // 3. Complete Wizard if visible
    const nextBtn = page.getByRole('button', { name: /Next/i });
    while (await nextBtn.isVisible()) {
      await nextBtn.click();
      await page.waitForTimeout(500); // Wait for transition
    }
    const launchBtn = page.getByRole('button', { name: /Launch My Business/i });
    if (await launchBtn.isVisible()) {
        await launchBtn.click();
    }

    // 4. Verify we are on Dashboard
    await expect(page.getByText('Dashboard', { exact: true })).toBeVisible({ timeout: 15000 });

    // 5. Verify the AI Help Chat FAB is visible
    const aiHelpChatBtn = page.getByRole('button', { name: /Ask anything/i });
    await expect(aiHelpChatBtn).toBeVisible();

    // 6. Click the AI Help Chat button
    await aiHelpChatBtn.click();

    // 7. Verify the Chat Overlay is visible with correct text
    const chatOverlayHeader = page.getByText('AI Support Agent');
    await expect(chatOverlayHeader).toBeVisible();

    const botMessage = page.getByText('Hello! I am your AI Support Agent');
    await expect(botMessage).toBeVisible();

    // 8. Close the overlay
    const closeBtn = page.getByRole('button', { name: 'Close' });
    if (await closeBtn.isVisible()) {
        await closeBtn.click();
        await expect(chatOverlayHeader).not.toBeVisible();
    }
  });
});
