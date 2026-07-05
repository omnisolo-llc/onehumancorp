import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Work Triage Mobile Feed CUJ', () => {
    test.use({ viewport: { width: 375, height: 812 } });

    test('Maya receives and handles an inquiry via the triage feed on mobile', async ({ adminPage: page }) => {
        // 1. Owner logs into OHC and navigates to the Work Triage Feed.
        await page.goto('/ui/triage.html');

        // Wait for page load
        await expect(page.locator('text=Unified Agent Feed')).toBeVisible();
        await expect(page.locator('#triage-list')).toBeVisible();

        // 2. Owner simulates receiving an external inquiry.
        // We use the new button we added to the UI
        const simulateBtn = page.getByTestId('simulate-missed-lead-btn');
        await expect(simulateBtn).toBeVisible();
        await simulateBtn.click();

        // Wait for the simulated item to appear
        // The mock from the backend creates a request for vegan chocolate cake
        const triageCard = page.locator('.triage-card').filter({ hasText: 'vegan chocolate cake' }).first();
        await expect(triageCard).toBeVisible({ timeout: 10000 });

        // Ensure it's rendered within a 375px bounding box correctly
        const cardBox = await triageCard.boundingBox();
        expect(cardBox.width).toBeLessThanOrEqual(375);

        // 3. Owner taps to expand the item
        await triageCard.locator('.triage-header').click();

        // 4. Owner taps "Review Draft" to see the AI's proposed response
        const reviewBtn = triageCard.getByTestId(/triage-review-btn-.*/);
        await expect(reviewBtn).toBeVisible();
        await reviewBtn.click();

        // Check that the edit text area is visible and contains the draft reply
        const editArea = triageCard.getByTestId(/triage-edit-textarea-.*/);
        await expect(editArea).toBeVisible();
        await expect(editArea).toHaveValue(/Yes, we have 2 vegan chocolate cakes left/);

        // 5. Owner taps "Save & Send"
        const saveSendBtn = triageCard.getByTestId(/triage-save-btn-.*/);
        await expect(saveSendBtn).toBeVisible();

        // Verify minimum touch target for the button
        const btnBox = await saveSendBtn.boundingBox();
        expect(btnBox.height).toBeGreaterThanOrEqual(44);
        expect(btnBox.width).toBeGreaterThanOrEqual(44);

        await saveSendBtn.click();

        // 6. Assert the action completes and the card disappears (backend mutation success)
        await expect(triageCard).toBeHidden({ timeout: 5000 });
    });
});
