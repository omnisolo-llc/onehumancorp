import { test, expect } from './fixtures';

test.describe('Viral Share to Unlock Loop', () => {
  test('Dashboard allows owner to create share-to-unlock campaign and user to unlock it', async ({ page, context, loginAs, adminUser }) => {
    await loginAs(page, adminUser);

    // 1. Navigate to dashboard
    await page.goto('/dashboard');

    // 2. Find and click the Share-to-Unlock Generator link
    // Wait for the dashboard to load fully
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 15000 });

    const generatorLink = page.locator('a[href="share-to-unlock-generator.html"]');
    await expect(generatorLink).toBeVisible();
    await generatorLink.click();

    // Verify generator page content
    await expect(page.getByRole('heading', { name: 'Share-to-Unlock Generator' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Campaign Settings' })).toBeVisible();

    // 3. Fill out the configuration
    const titleInput = page.getByLabel('Campaign Title');
    await titleInput.fill('');
    await titleInput.pressSequentially('Epic Summer Sale');

    const rewardInput = page.getByLabel('Reward Description');
    await rewardInput.fill('');
    await rewardInput.pressSequentially('50% Off Select Items');

    const codeInput = page.getByLabel('Hidden Discount Code');
    await codeInput.fill('');
    await codeInput.pressSequentially('SUMMER50');

    // 4. Click generate link
    const generateBtn = page.getByRole('button', { name: 'Generate Link' });
    await expect(generateBtn).toBeEnabled();
    await generateBtn.click();

    // 5. Capture the URL
    await expect(page.getByText('Your Share-to-Unlock Link is Ready!')).toBeVisible();
    const linkInput = page.locator('#generated-url');
    const generatedUrl = await linkInput.inputValue();
    expect(generatedUrl).toContain('/share-to-unlock/index.html');
    expect(generatedUrl).toContain('SUMMER50');
    expect(generatedUrl).toContain('Epic+Summer+Sale');

    // 6. Navigate to the generated public URL in a new context
    const publicPage = await context.newPage();
    await publicPage.goto(generatedUrl);

    // Verify the public entry page content
    await expect(publicPage.getByRole('heading', { name: 'Epic Summer Sale' })).toBeVisible();
    await expect(publicPage.getByText('50% Off Select Items')).toBeVisible();

    // Code is locked initially
    const codeBox = publicPage.locator('#discount-code');
    await expect(codeBox).toHaveClass(/locked-code-box/);
    await expect(codeBox).not.toHaveClass(/unlocked/);
    await expect(publicPage.locator('#locked-badge')).toBeVisible();
    await expect(publicPage.locator('#share-actions')).toBeVisible();
    await expect(publicPage.locator('#unlocked-actions')).not.toBeVisible();

    // Mock window.open so the test doesn't actually open a new tab and block
    await publicPage.evaluate(() => {
        window.open = function() { return null; };
    });

    // 7. Click share to unlock
    const shareBtn = publicPage.locator('#share-x-btn');
    await expect(shareBtn).toBeVisible();
    await shareBtn.click();

    // 8. Verify it unlocks
    await expect(codeBox).toHaveClass(/unlocked/);
    await expect(publicPage.locator('#share-actions')).not.toBeVisible();
    await expect(publicPage.locator('#unlocked-actions')).toBeVisible();
    await expect(publicPage.locator('#copy-code-btn')).toBeVisible();

    await publicPage.close();
  });
});