import { test, expect } from './fixtures';

test.describe('Store Wrap-Up Viral Loop', () => {
  test('verify store wrap-up presentation and viral sharing flow', async ({ page }) => {
    test.setTimeout(90000);

    // Navigate to dashboard
    await page.goto('/dashboard');

    // Verify Growth & Virality section exists
    const growthHeading = page.locator('h2', { hasText: 'Growth & Virality' });
    await expect(growthHeading).toBeVisible({ timeout: 15000 });

    // Find and click the Store Wrap-Up link
    const storeWrapLink = page.locator('a[href="/store-wrap"]');
    await expect(storeWrapLink).toBeVisible();
    await storeWrapLink.click();

    // Wait for the Wrap-Up page to load
    await page.waitForURL('**/store-wrap', { timeout: 15000 });

    // Verify first slide appears
    await expect(page.locator('h2', { hasText: 'Your Year in Review' })).toBeVisible({ timeout: 15000 });

    // The component has transparent overlays on the right (2/3 width) that triggers nextSlide()
    // Click on the right side of the screen 3 times to advance to the final slide
    const rightSide = page.locator('div.absolute.inset-y-0.right-0.w-2\\/3.z-20.cursor-pointer');

    // Slide 1 -> 2
    await rightSide.click();
    await expect(page.locator('h3', { hasText: 'Happy Customers' })).toBeVisible({ timeout: 10000 });

    // Slide 2 -> 3
    await rightSide.click();
    await expect(page.locator('h3', { hasText: 'Total Revenue' })).toBeVisible({ timeout: 10000 });

    // Slide 3 -> 4 (Final slide)
    await rightSide.click();
    await expect(page.locator('h2', { hasText: 'Share Your Success' })).toBeVisible({ timeout: 10000 });

    // Verify viral sharing buttons
    const copyButton = page.locator('button', { hasText: 'Copy Invite Link' });
    await expect(copyButton).toBeVisible();

    const postToX = page.locator('a', { hasText: 'Post to X' });
    await expect(postToX).toBeVisible();

    const whatsapp = page.locator('a', { hasText: 'WhatsApp' });
    await expect(whatsapp).toBeVisible();

    // Click Copy Invite Link and verify success message
    await copyButton.click();
    await expect(page.locator('button', { hasText: 'Link Copied!' })).toBeVisible();

    // Verify "Powered by OHC" branding is present on the final slide
    const branding = page.locator('div', { hasText: 'Powered by OHC' }).last();
    await expect(branding).toBeVisible();
  });
});
