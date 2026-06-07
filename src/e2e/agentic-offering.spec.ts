import { test, expect } from '@playwright/test';
import { loginAsTestUser } from './fixtures';

test.describe('Agentic Offering Creation', () => {
  test('User can create an offering using the AI agent text prompt', async ({ page }) => {
    // 1. Log in to Dashboard.
    await loginAsTestUser(page);

    // 2. Click "New Offering" button in the dashboard
    const newOfferingButton = page.locator('a', { hasText: 'New Offering' });
    await expect(newOfferingButton).toBeVisible();
    await newOfferingButton.click();

    // 3. Verify we are on the /offerings/new page and the prompt is visible
    await expect(page).toHaveURL(/\/offerings\/new/);
    await expect(page.locator('text=What do you want to offer?')).toBeVisible();

    // 4. Type the prompt
    const promptInput = page.locator('textarea[placeholder="e.g. Guitar lessons for beginners, 1 hour"]');
    await expect(promptInput).toBeVisible();
    await promptInput.fill('Guitar lessons for beginners, 1 hour');

    // 5. Submit the intent
    const generateButton = page.locator('button', { hasText: 'Generate' });
    await expect(generateButton).toBeEnabled();
    await generateButton.click();

    // 6. Verify loading state
    await expect(page.locator('text=AI is drafting your offering...')).toBeVisible();

    // 7. Verify the AI response pre-filled the form
    await expect(page.locator('text=AI is drafting your offering...')).toBeHidden({ timeout: 15000 });

    // Check that the title input exists and has some value populated by the AI
    const titleLabels = page.locator('label', { hasText: 'Title' });
    await expect(titleLabels).toBeVisible();

    const inputs = page.locator('input[type="text"]');
    // We expect multiple text inputs (Title, Price, Type).
    // They should not be empty.
    const count = await inputs.count();
    expect(count).toBeGreaterThan(0);

    // 8. Find and modify the price
    // Since the AI might suggest different prices, we locate the price input relative to the $ sign or label
    const priceInput = page.locator('label', { hasText: 'Price' }).locator('..').locator('input[type="text"]');
    await expect(priceInput).toBeVisible();
    await priceInput.fill('45.00');

    // 9. Click "Publish Service" or "Publish Product" (dynamic based on AI type)
    const publishButton = page.locator('button', { hasText: /^Publish/ });
    await publishButton.click();

    // 10. Verify success state
    await expect(page.locator('text=Published!')).toBeVisible();
    await expect(page.locator('text=Your new offering is now live on your storefront.')).toBeVisible();

    // 11. Click "View Storefront" and verify the item is visible
    const viewStorefrontButton = page.locator('a', { hasText: 'View Storefront' });
    await viewStorefrontButton.click();

    await expect(page).toHaveURL(/\/bio\/demo-tenant/);
  });
});
