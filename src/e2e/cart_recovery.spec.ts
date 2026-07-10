import { test, expect } from './fixtures';

test('cart recovery workflow', async ({ page, request, loginAs, adminUser }) => {
    // 1. Log in
    await loginAs(page, adminUser);

    await page.goto('/dashboard.html');

    // 2. Wait for widget
    const widget = page.locator('#cart-recovery-widget');
    await widget.waitFor({ state: 'visible', timeout: 15000 });

    // Check count is visible
    const count = await page.locator('#dashboard-cart-count').innerText();
    expect(Number(count)).toBeGreaterThanOrEqual(1);

    // 3. Click CTA to go to recovery screen
    await page.getByRole('link', { name: 'Recover Carts' }).click();

    // 4. We should be on the recovery page
    await expect(page).toHaveURL(/.*cart-recovery\.html/);
    await expect(page.getByRole('heading', { name: 'Abandoned Cart Recovery' })).toBeVisible();

    // Simulate Pro User bypass
    await page.evaluate(() => {
        localStorage.setItem('has_pro', 'true');
        // Because cart-recovery.html reads it globally on load:
        window.hasPro = true;
    });

    // 5. Fill out the form
    await page.fill('#customer-name', 'Test Customer');
    await page.fill('#cart-value', '$99.99');

    // 6. Generate the AI campaign
    const generateBtn = page.locator('#generate-btn');
    await generateBtn.click();

    // 7. Assert that the generated draft appears
    const draftPreview = page.locator('#draft-preview');
    await expect(draftPreview).not.toHaveText(/Click "Generate AI Campaign"/, { timeout: 15000 });

    const draftText = await draftPreview.innerText();
    expect(draftText).toContain('Test Customer');
    expect(draftText).toContain('$99.99');
});
