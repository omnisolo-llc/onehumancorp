import { test, expect } from './fixtures';

test.describe('Aider: RepoMap for large codebases', () => {
  test('should allow user to generate repomap', async ({ page }) => {
    // Navigate to the Aider RepoMap page
    await page.goto('/aider-repomap');

    // Verify UI elements are present
    const heading = page.locator('h1', { hasText: 'Aider: RepoMap Explorer' });
    await expect(heading).toBeVisible();

    const generateBtn = page.locator('button', { hasText: 'Generate RepoMap' });
    await expect(generateBtn).toBeVisible();
    await expect(generateBtn).toBeEnabled();

    // The instruction says ZERO MOCK DATA.
    await generateBtn.click();

    // Verify loading state
    await expect(page.locator('button', { hasText: 'Scanning Repository...' })).toBeVisible();

    // Wait for the result to appear
    const successMessage = page.locator('[data-testid="success-message"]');
    await expect(successMessage).toBeVisible({ timeout: 15000 });

    // Verify the RepoMap structure is returned
    const mapContent = await successMessage.textContent();
    expect(mapContent?.length).toBeGreaterThan(10);
    expect(mapContent).toContain('├──'); // Typical tree output indicator
  });
});