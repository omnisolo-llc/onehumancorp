import { test, expect } from './fixtures';

test.describe('Viral Customer Poll Loop', () => {
  test('should allow owner to generate poll embed and user to vote', async ({ page, context }) => {
    // 1. Navigate to dashboard
    await page.goto('/dashboard');

    // We mock localStorage so the script works in E2E environment
    await page.evaluate(() => { localStorage.setItem('tenant', 'e2e-test-store'); window.dispatchEvent(new Event('storage')); });

    // 2. Find and click the Customer Poll Generator link
    const pollLink = page.locator('a[href="viral-customer-poll.html"]');
    await expect(pollLink).toBeVisible();
    await pollLink.click();

    // Verify page content
    await expect(page.getByRole('heading', { name: /Viral Customer Poll Generator/i })).toBeVisible();

    // 3. Fill out the poll configuration
    const questionInput = page.locator('#poll-question');
    await questionInput.fill('How was your experience today?');

    const optionsInput = page.locator('#poll-options');
    await optionsInput.fill('Excellent, OK, Bad');

    // 4. Click generate link
    const generateBtn = page.getByRole('button', { name: 'Generate Poll Embed' });
    await expect(generateBtn).toBeEnabled();
    await generateBtn.click();

    // Wait for generation
    await expect(page.locator('#result-area')).toBeVisible({ timeout: 5000 });

    // 5. Capture the embed code
    const embedInput = page.locator('#embed-code');
    const embedCode = await embedInput.inputValue();

    expect(embedCode).toContain('How was your experience today?');
    expect(embedCode).toContain('Excellent');
    expect(embedCode).toContain('customer-poll/index.html');
    expect(embedCode).toContain('Powered by OHC');

    // Verify "Powered by OHC" footer is in the preview
    const previewArea = page.locator('#preview-area');
    await expect(previewArea).toContainText('Powered by OHC');
    const brandingLink = previewArea.locator('a', { hasText: '⚡ Powered by OHC' });
    await expect(brandingLink).toBeVisible();

    // 6. Navigate to the hosted public poll page simulating a user clicking an option
    // Open a new page context to simulate a public user
    const publicPage = await context.newPage();
    const voteUrl = `/customer-poll/index.html?q=${encodeURIComponent('How was your experience today?')}&vote=${encodeURIComponent('Excellent')}&tenant=e2e-test-store`;
    await publicPage.goto(voteUrl);

    // Verify the public entry page content
    await expect(publicPage.getByRole('heading', { name: 'Thank you for voting!' })).toBeVisible();
    await expect(publicPage.locator('#question-desc')).toContainText('How was your experience today?');
    await expect(publicPage.locator('#voted-option')).toContainText('Excellent');

    // Verify "Powered by OHC" footer
    const footerLink = publicPage.locator('a', { hasText: '⚡ Powered by OHC' });
    await expect(footerLink).toBeVisible();
    const footerHref = await footerLink.getAttribute('href');
    expect(footerHref).toContain('/setup.html?ref=e2e-test-store');

    await publicPage.close();
  });
});
