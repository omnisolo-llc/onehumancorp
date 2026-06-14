import { test, expect } from '@playwright/test';

test.describe('OHC Setup Draft Button', () => {
  test('Save draft button transitions gracefully via error fallback in isolated frontend', async ({ page }) => {
    await page.goto(`file://${process.cwd()}/src/ui/tauri/src/ui/setup.html`);

    await page.waitForSelector('button[data-testid="next-step-btn"]');
    await page.click('button[data-testid="next-step-btn"]');

    // Click the label
    await page.click('text=Local Service');

    // Click save draft
    const draftBtn = page.locator('.step.active .save-draft-btn');
    await draftBtn.click();

    // Wait for the button to transition through 'Saving...' to 'Saved Locally'
    await expect(draftBtn).toHaveText('Saved Locally', { timeout: 2000 });

    // Wait to ensure button text resets eventually
    await expect(draftBtn).toHaveText('Save Draft', { timeout: 4000 });
  });
});
