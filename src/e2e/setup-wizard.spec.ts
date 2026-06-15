import { test, expect } from './fixtures';

test.describe('Setup Wizard 375px Flow', () => {
    test.use({ viewport: { width: 375, height: 812 } });

    test('should render properly and allow selection', async ({ page }) => {
        await page.goto('/onboarding');
        await page.waitForLoadState('domcontentloaded');

        // Check if step-initial is active
        const startBtn = page.getByRole('button', { name: /Start My Business/i });
        await expect(startBtn).toBeVisible();

        // Click Start My Business
        await startBtn.click();

        // Chat Step 1
        const nameInput = page.getByPlaceholder(/Maya's Custom Cakes/i);
        await expect(nameInput).toBeVisible();
        await nameInput.fill('Maya Bakery');

        const nextBtn1 = page.getByRole('button', { name: /Next/i });
        await nextBtn1.click();
    });
});
