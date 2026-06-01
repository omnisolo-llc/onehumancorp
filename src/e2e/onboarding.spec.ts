import { currentAppSmoke } from './current_app_smoke';
import { expect, test } from './fixtures';

currentAppSmoke('onboarding');

test.describe('Zero-Click AI Storefront Setup', () => {
  test('should generate storefront from AI prompt and verify zero-click setup', async ({ page }) => {
    await page.goto('/website-builder');

    // Switch to AI setup mode
    const instantBuildButton = page.locator('button', { hasText: 'Instant Build (AI)' });
    await expect(instantBuildButton).toBeVisible();
    await instantBuildButton.click();

    // Input business description
    const aiInput = page.locator('#step-ai-prompt');
    await expect(aiInput).toBeVisible();
    await aiInput.fill("I sell custom cakes in Seattle");

    // Wait for the backend response from the real application stack
    const responsePromise = page.waitForResponse('**/api/v1/builder/generate', { timeout: 30000 });

    // Submit prompt
    const generateBtn = page.locator('button', { hasText: 'Generate Storefront' });
    await expect(generateBtn).toBeVisible();
    await generateBtn.click();

    // The frontend should show generating step then navigate to builder
    await expect(page.locator('#step-generating')).toBeVisible();

    await responsePromise;

    // Wait until we reach storefront builder screen
    await expect(page.locator('#storefront-builder-screen')).toBeVisible({ timeout: 20000 });

    // Wait for builder preview elements to be rendered by the actual AI engine
    await expect(page.locator('.builder-block')).not.toHaveCount(0);
  });
});
