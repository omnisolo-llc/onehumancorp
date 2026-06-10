import { test, expect } from '@playwright/test';

test.describe('Onboarding Setup', () => {
  test('Save Draft button gives visual feedback', async ({ page }) => {
    const fileUrl = `file://${process.cwd()}/src/ui/tauri/src/ui/setup.html`;
    await page.goto(fileUrl);

    // Select "I'm a Baker" to move to the next step
    await page.locator('text="I\'m a Baker"').click();

    // We also need to click the Next button to actually go to the categories step
    // since clicking "I'm a Baker" just populates things
    await page.locator('#step-context .next-step-btn').click();

    // Now we should be on the category step where the Save Draft button is visible
    const saveDraftBtn = page.locator('#step-categories .save-draft-btn');
    await expect(saveDraftBtn).toBeVisible();

    await saveDraftBtn.click();

    // Expect the text to change to "Saved!"
    await expect(saveDraftBtn).toHaveText('Saved!');

    // Wait for the revert
    await expect(saveDraftBtn).toHaveText('Save Draft', { timeout: 3000 });
  });
});
