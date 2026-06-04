import { test, expect } from './fixtures';

test.describe('Leo CUJ', () => {
  test('Leo adds a service and creates a link-in-bio', async ({ page }) => {
    // 1. Navigate to the App and wait for dashboard to load (we must simulate the login from root)
    await page.goto('/login');

    // Simulate login for Leo
    const usernameInput = page.getByPlaceholder('Email or Username');
    if (await usernameInput.isVisible()) {
        await usernameInput.fill('leo@example.com');
        await page.locator('input[type="password"]').fill('password123');
        await page.locator('button:has-text("Login")').click();
    }

    // Fallback to direct navigation if the previous block doesn't naturally navigate due to fixture overrides
    await page.goto('/');

    // Wait for the dashboard heading to appear, indicating the app has fully loaded.
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();

    // 2. Add a new service using the dashboard modal (as seen in dashboard/page.tsx)
    const addItemBtn = page.locator('button:has-text("+ Add Item")').first();
    await addItemBtn.waitFor({ state: 'visible', timeout: 10000 });
    await addItemBtn.click();

    // Select 'Service' type
    const serviceTypeBtn = page.locator('button', { hasText: 'Service' });
    await expect(serviceTypeBtn).toBeVisible();
    await serviceTypeBtn.click();

    // Fill in the service details
    const itemNameInput = page.locator('input[placeholder="e.g. Custom Cake"]');
    await itemNameInput.fill('Online Guitar Lesson');

    const priceInput = page.locator('input[placeholder="0.00"]');
    await priceInput.fill('45');

    const durationInput = page.locator('input[placeholder="60"]');
    await durationInput.fill('60');

    // Save the service
    const saveButton = page.locator('button', { hasText: 'Save Service' });
    await saveButton.click();

    // Verify the modal closes by waiting for the 'Save Service' button to disappear
    await expect(saveButton).not.toBeVisible();

    // 3. Navigate to Link In Bio generator using the nav links (found in dashboard/page.tsx)
    const linkInBioNavLink = page.locator('a[href="/link-in-bio-generator"]');
    await expect(linkInBioNavLink).toBeVisible();
    await linkInBioNavLink.click();

    // Fill out the Bio generator fields (as seen in link-in-bio-generator/page.tsx)
    const businessNameInput = page.locator('label', { hasText: 'Business Name' }).locator('..').locator('input');
    await businessNameInput.fill('Leo Guitar Lessons');

    // The preview should update
    await expect(page.locator('h1', { hasText: 'Leo Guitar Lessons' })).toBeVisible();

    // Generate/copy link
    const copyButton = page.locator('button', { hasText: 'Copy Link-in-Bio URL' });
    await copyButton.click();

    // Check if the button text changes indicating success
    await expect(page.locator('button', { hasText: 'Copied Link!' })).toBeVisible();
  });
});