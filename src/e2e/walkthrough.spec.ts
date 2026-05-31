import { test, expect } from './fixtures';

test.describe('Walkthrough System', () => {
  test('should display walkthrough steps', async ({ page }) => {
    // Navigate to a page with a walkthrough, for example kairos page
    await page.goto('/kairos?walkthrough=true');

    // Wait for the walkthrough bubble to appear
    const bubble = page.locator('.animate-pop-in');
    await expect(bubble).toBeVisible({ timeout: 5000 });

    // Verify it contains the walkthrough text
    await expect(bubble).toContainText("The Shared Task List is the 'Brain'");

    // Verify Next button works
    await page.getByRole('button', { name: 'Next' }).click();

    // Check next step text
    await expect(bubble).toContainText("This visualizes how your AI agents talk to each other");

    // Verify Finish button works
    await page.getByRole('button', { name: 'Finish' }).click();

    // Walkthrough should close
    await expect(bubble).toBeHidden();
  });
});
