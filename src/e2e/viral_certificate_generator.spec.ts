import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('viral_certificate_generator_smoke', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'viral_certificate_generator_smoke');
});

test.describe('Viral Certificate Generator', () => {
    test('should allow owner to generate a certificate embed code and user sees viral loop', async ({ page, context }) => {
        // 1. Navigate to dashboard
        await page.goto('/dashboard.html');

        // 2. Find and click the Certificate Generator link
        const certificateLink = page.locator('a[href="viral-certificate-generator.html"]');
        await expect(certificateLink).toBeVisible();
        await certificateLink.click();

        // Verify page content
        await expect(page.getByRole('heading', { name: 'Viral Certificate Generator 🎓' })).toBeVisible();

        // 3. Fill out the configuration
        const tenantInput = page.locator('#tenant');
        await tenantInput.fill('e2e-demo-tenant');

        const titleInput = page.locator('#cert-title');
        await titleInput.fill('Certificate of Achievement');

        const recipientInput = page.locator('#recipient');
        await recipientInput.fill('John E2E Doe');

        const courseInput = page.locator('#course-name');
        await courseInput.fill('E2E Mastery Course');

        // Wait for preview to update
        await page.waitForTimeout(500);

        // Verify the live preview frame URL updated
        const previewFrame = page.locator('#preview-frame');
        await expect(previewFrame).toBeVisible();
        const frameSrc = await previewFrame.getAttribute('src');
        expect(frameSrc).toContain('title=Certificate%20of%20Achievement');
        expect(frameSrc).toContain('recipient=John%20E2E%20Doe');

        // 4. Click generate link
        const generateBtn = page.getByRole('button', { name: 'Generate Widget' });
        await expect(generateBtn).toBeEnabled();
        await generateBtn.click();

        // 5. Capture the Embed Code
        const resultArea = page.locator('#result-area');
        await expect(resultArea).toBeVisible();

        const embedCode = await page.locator('#embed-code').innerText();
        expect(embedCode).toContain('<iframe');
        expect(embedCode).toContain('api/v1/growth/certificate/embed');

        // Extract the URL from the iframe code
        const srcMatch = embedCode.match(/src="([^"]+)"/);
        expect(srcMatch).not.toBeNull();
        const generatedUrl = srcMatch![1];

        // 6. Navigate to the generated public URL
        // Open a new page context to simulate a public user
        const publicPage = await context.newPage();
        await publicPage.goto(generatedUrl);

        // Verify the public certificate content
        await expect(publicPage.getByRole('heading', { name: 'Certificate of Achievement' })).toBeVisible();
        await expect(publicPage.locator('.recipient', { hasText: 'John E2E Doe' })).toBeVisible();
        await expect(publicPage.locator('h2', { hasText: 'E2E Mastery Course' })).toBeVisible();

        // Verify "Powered by OHC" viral loop footer
        const footerLink = publicPage.locator('a', { hasText: '⚡ Powered by OHC' }).first();
        await expect(footerLink).toBeVisible();
        const footerHref = await footerLink.getAttribute('href');
        expect(footerHref).toContain('/onboarding');
        expect(footerHref).toContain('ref=e2e-demo-tenant');

        await publicPage.close();
    });
});
