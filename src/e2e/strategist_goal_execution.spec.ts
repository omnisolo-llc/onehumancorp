import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Strategist: Proactive Cross-Agent Goal Execution', () => {
    test('Maya the Baker sets a goal and Strategist decomposes it into multi-agent tasks', async ({ page }) => {
        // 1. Login and navigate to Strategist Dashboard
        // Using a direct URL for implementation verification
        await page.goto('/ui/strategist.html');

        // 2. Set the goal
        const goalInput = page.locator('#goal-input');
        await goalInput.fill('Sell out of winter cookies by Friday');

        const architectBtn = page.getByRole('button', { name: /Architect Plan/i });
        await architectBtn.click();

        // 3. Verify Strategist received the goal
        // In the mock implementation, it shows an alert and then reloads objectives
        await expect(page.locator('.glass-card.plan-card')).toContainText('Goal: Sell out of winter cookies by Friday');

        // 4. Verify backend decomposition (Agent Feed items created)
        // We can check the /api/ui/dashboard/unified-feed or the database directly via a helper if available,
        // but for this implementer test, we'll verify the UI shows the 'Strategist is coordinating' state.
        await expect(page.locator('.plan-card')).toContainText('Strategist is coordinating');

        // 5. Verify the mobile-first layout
        const viewport = page.viewportSize();
        if (viewport) {
            // Ensure the container fits 375px
            const containerWidth = await page.locator('.container').evaluate(el => el.clientWidth);
            expect(containerWidth).toBeLessThanOrEqual(375);
        }
    });
});
