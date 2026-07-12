import { test, expect } from '../../../../e2e/fixtures';

test.describe('Link-in-Bio Generator E2E', () => {
  test('User can create and publish link in bio, then view it publicly', async ({ page }) => {
    // 1. Navigate to the link-in-bio generator page
    await page.goto('/link-in-bio-generator');

    // 2. Wait for the page to be ready (ensure "Publish Changes" button is visible)
    const publishButton = page.locator('button', { hasText: 'Publish Changes' });
    await expect(publishButton).toBeVisible();

    // 3. Update the business name and bio
    const businessNameInput = page.getByRole('textbox', { name: 'Business name' });
    await businessNameInput.fill('Playwright Test Bakery');

    const bioInput = page.getByRole('textbox', { name: 'Bio tagline' });
    await bioInput.fill('We bake the best E2E cakes!');

    // 4. Update the first link
    const linkTitleInput = page.locator('input[placeholder="Title (e.g. Visit my Shop)"]').first();
    await linkTitleInput.fill('Our Menu');

    const linkUrlInput = page.locator('input[placeholder="URL (e.g. https://...)"]').first();
    await linkUrlInput.fill('https://example.com/menu');

    // We can't rely on `window.alert` directly without setting up a listener,
    // so we handle the alert. Playwright automatically dismisses dialogs unless specified,
    // but let's be explicit.
    page.on('dialog', dialog => dialog.accept());

    // 5. Publish changes
    await publishButton.click();

    // Wait a brief moment for the save
    await page.waitForTimeout(500);

    // 6. Navigate to the public bio page
    // By default, the generator uses 'my-store' as the default tenant id in localStorage if none is set
    await page.goto('/bio/my-store');

    // 7. Verify the changes are visible on the public page
    // Using a more specific selector, as Next.js layout might have other h1s (like "Bio" in the header)
    await expect(page.locator('h1.font-outfit.text-3xl')).toHaveText('Playwright Test Bakery');
    await expect(page.locator('p.leading-relaxed')).toHaveText('We bake the best E2E cakes!');

    const publishedLink = page.locator('a:has-text("Our Menu")');
    await expect(publishedLink).toBeVisible();
    await expect(publishedLink).toHaveAttribute('href', 'https://example.com/menu');
  });
});
