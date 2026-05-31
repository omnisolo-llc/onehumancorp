import { test, expect } from '@playwright/test';

test.describe('Flyer Designer Growth Loop', () => {
  test('User can design and preview a business flyer', async ({ page }) => {
    // Navigate to Dashboard
    await page.goto('/dashboard');

    // Check for the Flyer Designer link and click it
    const flyerLink = page.getByRole('link', { name: 'Flyer Designer' });
    await expect(flyerLink).toBeVisible();
    await flyerLink.click();

    // Verify we are on the Flyers page
    await expect(page).toHaveURL(/\/flyers/);
    await expect(page.getByText('Flyer Designer')).toBeVisible();

    // Customize the flyer
    const nameInput = page.getByPlaceholder('Enter business name');
    await nameInput.clear();
    await nameInput.fill('Maya Cakes');

    const taglineInput = page.getByPlaceholder('Enter a catchy tagline');
    await taglineInput.clear();
    await taglineInput.fill('The sweetest treats!');

    // Select a color
    await page.locator('button[style*="background-color: rgb(239, 68, 68)"]').click();

    // Wait for preview to update (debounced)
    await page.waitForTimeout(1000);

    // Verify preview content (it should be an SVG)
    const previewContainer = page.locator('#flyer-preview-container');
    await expect(previewContainer).toBeVisible();

    // Check if SVG contains the customized text (using innerHTML check)
    const svgContent = await previewContainer.innerHTML();
    expect(svgContent).toContain('Maya Cakes');
    expect(svgContent).toContain('The sweetest treats!');
    expect(svgContent).toContain('SCAN TO SHOP');
    expect(svgContent).toContain('Powered by OHC');

    // Test soft paywall
    await page.getByText('Remove OHC Branding').click();
    await expect(page.getByText('Professional Flyers')).toBeVisible();
    await expect(page.getByText('Upgrade to Pro')).toBeVisible();
  });
});
