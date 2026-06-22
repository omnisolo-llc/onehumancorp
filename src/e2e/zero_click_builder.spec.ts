import { test, expect } from './fixtures';

test.describe('Zero Click Builder Viral Growth Loop', () => {
  test('should allow an owner to generate a store via chat and see viral share option', async ({ page, request, loginAs, adminUser }) => {
    // Navigate to the new growth feature
    await loginAs(page, adminUser);

    await page.goto('/api/ui/zero-click-builder.html');

    // Verify mobile-first layout
    await page.setViewportSize({ width: 375, height: 812 });

    // Verify title
    await expect(page.locator('h1', { hasText: 'Zero-Click Business Generator' })).toBeVisible({ timeout: 15000 });

    // Verify "Powered by OHC" branding is present (viral loop)
    await expect(page.getByText('⚡ Powered by OHC')).toBeVisible();

    // Verify the first message from the agent
    await expect(page.getByText("Hi! I'm the Zero-Click Onboarding Agent")).toBeVisible();

    // Step 1: Send business name
    await page.fill('input[id="prompt"]', 'My Seattle Coffee Roasters');
    await page.locator('button[id="send-btn"]').click();

    // Check that user message is displayed
    await expect(page.getByText('My Seattle Coffee Roasters')).toBeVisible();

    // Check that agent replies
    await expect(page.getByText('Great! Can you upload a photo')).toBeVisible();

    // Step 2: Send description
    await page.fill('input[id="prompt"]', 'I sell freshly roasted coffee beans.');
    await page.locator('button[id="send-btn"]').click();

    // Wait for the loading state to complete and the result to appear
    await expect(page.getByText('Your business is live!')).toBeVisible({ timeout: 20000 });

    // Verify the generated preview iframe is visible
    const previewIframe = page.locator('iframe[title="Live Storefront Preview"]');
    await expect(previewIframe).toBeVisible();

    // Verify the launch button is present
    const launchBtn = page.getByRole('button', { name: /Launch My Store/i });
    await expect(launchBtn).toBeVisible();

    // Share button
    const shareBtn = page.getByRole('button', { name: /Share on X/i });
    await expect(shareBtn).toBeVisible();

    // Click the launch button to verify redirect
    await launchBtn.click();
    await expect(page).toHaveURL(/\/dashboard/);
  });
});
