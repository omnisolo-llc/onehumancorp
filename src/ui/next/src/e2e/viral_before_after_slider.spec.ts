import { test, expect } from './fixtures';

test.describe('Viral Before & After Slider Widget Builder', () => {
  test('should navigate to widget builder, configure options, preview embed code, and trigger paywall for branding removal', async ({ page, loginAs, unlimitedAdminUser }) => {
    // 1. Navigate to the page
    await loginAs(unlimitedAdminUser.email, 'password123');
    await page.goto('/viral-before-after-slider');

    // 2. Verify Page Content
    await expect(page.getByRole('heading', { name: 'Before & After Slider' })).toBeVisible({ timeout: 15000 });
    await expect(page.getByText('Widget Title')).toBeVisible();

    // 3. Configure form options
    const titleInput = page.getByRole('textbox').first();
    await titleInput.fill('My Awesome Remodel');

    // The URLs inputs
    const beforeInput = page.getByRole('textbox').nth(1);
    await beforeInput.fill('https://example.com/before-test.jpg');

    const afterInput = page.getByRole('textbox').nth(2);
    await afterInput.fill('https://example.com/after-test.jpg');

    // 4. Verify preview iframe source update
    const previewIframe = page.locator('iframe[title="Widget Preview"]');
    await expect(previewIframe).toBeVisible();

    // Get the src and assert it contains encoded parts
    const iframeSrc = await previewIframe.getAttribute('src');
    expect(iframeSrc).toContain('title=My%20Awesome%20Remodel');
    expect(iframeSrc).toContain('before=https%3A%2F%2Fexample.com%2Fbefore-test.jpg');
    expect(iframeSrc).toContain('after=https%3A%2F%2Fexample.com%2Fafter-test.jpg');

    // 5. Test Embed Modal functionality
    const embedBtn = page.getByRole('button', { name: 'Get Widget Embed Code' });
    await expect(embedBtn).toBeVisible();
    await embedBtn.click();

    const modalHeading = page.getByRole('heading', { name: 'Embed Slider' });
    await expect(modalHeading).toBeVisible();

    // Close the embed modal
    await page.getByRole('button', { name: 'Close' }).click();
    await expect(modalHeading).toBeHidden();

    // 6. Test the "Powered by OHC" soft paywall
    const removeBrandingCheckbox = page.getByRole('checkbox');
    await expect(removeBrandingCheckbox).toBeVisible();

    // Set localStorage to simulate non-pro user if needed (in case the unlimitedAdminUser fixture does not provide pro access to this component's local storage)
    await page.evaluate(() => {
      localStorage.setItem('has_pro', 'false');
    });

    // Uncheck and check to ensure the paywall modal shows
    await removeBrandingCheckbox.click();

    // Check if the Paywall triggers
    const paywallHeading = page.getByRole('heading', { name: 'Upgrade to Remove Branding' });
    await expect(paywallHeading).toBeVisible();

    // Close the paywall modal
    await page.getByRole('button', { name: 'Cancel' }).click();
    await expect(paywallHeading).toBeHidden();
  });
});
