import { test, expect } from './fixtures';

test.describe('Virtual Meeting Room & UltraPlan Walkthrough', () => {
  test('starts walkthrough from Help Widget', async ({ page }) => {
    await page.goto('/builder');

    // Open the Help Widget
    const helpButton = page.locator('button[aria-label="Help"]');
    await expect(helpButton).toBeVisible();
    await helpButton.click();

    // Verify Help Center tab is active and visible
    await expect(page.getByRole('heading', { name: 'Help Center' })).toBeVisible();

    // Start the tour
    const tourButton = page.locator('button:has-text("Tour: Virtual Meeting Room & UltraPlan")');
    await expect(tourButton).toBeVisible();
    await tourButton.click();

    // Verify first step of walkthrough
    const bubbleText1 = page.locator('text=Agents join the Virtual Meeting Room to debate and plan before executing tasks.');
    await expect(bubbleText1).toBeVisible();

    // Go to next step
    const nextButton = page.locator('button:has-text("Next")');
    await expect(nextButton).toBeVisible();
    await nextButton.click();

    // Verify second step of walkthrough
    const bubbleText2 = page.locator('text=Phase 1: Brainstorming. Phase 2: Refinement. Phase 3: Consensus (UltraPlan protocol).');
    await expect(bubbleText2).toBeVisible();

    // Finish walkthrough
    const gotItButton = page.locator('button:has-text("Got it")');
    await expect(gotItButton).toBeVisible();
    await gotItButton.click();

    // Verify walkthrough is closed
    await expect(bubbleText2).not.toBeVisible();
  });
});
