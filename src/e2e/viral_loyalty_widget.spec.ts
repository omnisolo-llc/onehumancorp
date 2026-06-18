import { test, expect } from './fixtures';

test.describe('Viral Loyalty Widget Loop', () => {
  test('should allow owner to create a loyalty program and get a share link', async ({ page }) => {
    // 1. Navigate to dashboard
    await page.goto('/dashboard');

    // 2. Find and click the Loyalty Engine link
    const loyaltyLink = page.locator('a[href="viral-loyalty-widget.html"]');
    await expect(loyaltyLink).toBeVisible();
    await loyaltyLink.click();

    // Verify page content
    await expect(page.getByRole('heading', { name: 'Viral Loyalty Widget Generator' })).toBeVisible();
    await expect(page.getByText('Every 5th purchase is free')).toBeVisible();

    // Wait for JS to attach
    await page.waitForTimeout(500);

    // Mock storage so the tenant context is there
    await page.evaluate(() => { localStorage.setItem('tenant_id', 'e2e-tenant'); window.dispatchEvent(new Event('storage')); });

    // 3. Click generate link
    const generateBtn = page.getByRole('button', { name: 'Generate Loyalty Program' });
    await expect(generateBtn).toBeEnabled();

    // We can also wait for the mock API call
    // We can also wait for the mock API call
    const [request] = await Promise.all([
        page.waitForRequest(req => req.url().includes('/api/v1/growth/referrals/generate') && req.method() === 'POST'),
        generateBtn.click()
    ]);

    // 4. Capture the URL and check visibility
    await expect(page.getByText('Program Generated Successfully!')).toBeVisible();
    const linkInput = page.locator('input.loyalty-share-link[readonly]');
    const generatedUrl = await linkInput.inputValue();
    expect(generatedUrl).toContain('/loyalty/join?ref=');

    // 5. Verify "Back to Dashboard" footer link
    const backLink = page.locator('a.back-link', { hasText: 'Back to Dashboard' });
    await expect(backLink).toBeVisible();
    await backLink.click();

    // Verify we're back
    await expect(page.locator('a[href="viral-loyalty-widget.html"]')).toBeVisible();
  });
});
