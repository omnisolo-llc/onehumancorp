import { test, expect } from './fixtures';

test.describe('Viral Review Wall Generator', () => {
  test('can configure and generate viral review wall widget', async ({ page, adminUser, loginAs }) => {
    // 1. Log in as admin
    await loginAs(page, adminUser);

    // 2. Go to dashboard and navigate to the generator
    await page.goto('/dashboard.html');

    // Wait for the link to be visible and click it
    const wallLink = page.locator('a[href="viral-review-wall-generator.html"]');
    await expect(wallLink).toBeVisible();
    await wallLink.click();

    // 3. Verify we are on the generator page
    await expect(page.locator('h1')).toContainText('Viral Review Wall Generator');

    // 4. Test live preview updates
    const titleInput = page.locator('#wall-title');
    await titleInput.fill('');
    await titleInput.fill('What our fans say');

    await expect(page.locator('#preview-title')).toContainText('What our fans say');

    // 5. Test paywall for branding removal (without pro)
    await page.evaluate(() => {
        localStorage.setItem('has_pro', 'false');
    });

    const removeBrandingCheckbox = page.locator('#remove-branding');
    await removeBrandingCheckbox.check();

    // Paywall modal should appear
    const paywallModal = page.locator('#paywall-modal');
    await expect(paywallModal).toHaveClass(/active/);
    await page.locator('#close-paywall').click();

    // 6. Test embed code generation
    const getCodeBtn = page.locator('#get-code-btn');
    await getCodeBtn.click();

    // Embed modal should appear
    const embedModal = page.locator('#embed-modal');
    await expect(embedModal).toHaveClass(/active/);

    // Embed code should be populated
    const embedCode = page.locator('#embed-code');
    await expect(embedCode).toHaveValue(/<iframe src=".*\/api\/v1\/growth\/viral-review-wall\?tenant=[^"]*&theme=light&title=What%20our%20fans%20say&hideBranding=false" width="100%" height="500"/);

    // Test copy button
    const copyCodeBtn = page.locator('#copy-code-btn');
    await copyCodeBtn.click();
    await expect(copyCodeBtn).toContainText('Copied!');
  });
});
