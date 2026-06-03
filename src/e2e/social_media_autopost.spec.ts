import { test, expect } from './fixtures';

test.describe('Autonomous Social Media Manager Agent Workflow', () => {
  test('generates a social post draft in approval inbox when a new product is added', async ({ page }) => {
    test.setTimeout(60000);

    // 1. Navigate to Add Product page directly and trigger the tenant.product.created event
    await page.goto('/products/new');
    await expect(page.getByRole('heading', { name: 'Add Product' })).toBeVisible();

    const productName = `E2E AI Vegan Cake ${Date.now()}`;

    // Simulate auto-generation wait or manually fill the form
    // Since the actual page does not have a real file upload but uses states or an input,
    // We will just directly fill the inputs if they are available, or bypass if it's an AI dream state.
    // If we look at products/new/page.tsx, we can mock the photo upload process.

    // Upload a photo
    const fileChooserPromise = page.waitForEvent('filechooser');
    await page.locator('text="Take a photo or upload"').click();
    const fileChooser = await fileChooserPromise;
    // We can simulate an image by writing a tiny valid transparent pixel image first, but let's just upload anything or use the API
    // Actually, creating a dummy file is easier
    const require = require('node:module').createRequire(import.meta.url);
    const fs = require('node:fs');
    fs.writeFileSync('/tmp/dummy.png', 'iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg==', 'base64');
    await fileChooser.setFiles('/tmp/dummy.png');

    // Wait for the AutoDream to analyze and display the generated input form
    await expect(page.getByDisplayValue(/.*|/)).toBeVisible({ timeout: 15000 });

    // Now fill in the generated inputs
    await page.locator('input[type="text"]').first().fill(productName);

    // Publish the product
    await page.getByRole('button', { name: 'Publish Product' }).click();

    // Verify it was published successfully
    await expect(page.getByRole('heading', { name: 'Product Published!' })).toBeVisible();

    // 2. Navigate to the team approvals inbox where the AI agent should have drafted a post based on our new product
    await page.goto('/team');
    await expect(page.getByRole('heading', { name: 'The Expert Team' })).toBeVisible();

    // Open The Promoter's inbox (Marketing)
    await page.getByText('The Promoter').click();
    await expect(page.getByRole('heading', { name: 'The Promoter' })).toBeVisible();

    // 3. Verify the dynamically generated social post draft is visible in the inbox
    await expect(page.getByText('Draft Social Post')).toBeVisible({ timeout: 15000 });
    await expect(page.getByText(`New post for ${productName} ready to schedule.`)).toBeVisible();

    // Check that the AI generated caption is present
    await expect(page.getByText('Draft Caption')).toBeVisible();

    // 4. Approve & Schedule the seeded post
    const approveButton = page.getByRole('button', { name: 'Approve & Schedule' }).first();
    await expect(approveButton).toBeVisible();
    await approveButton.click();

    // Verify the approval was processed and removed from the list
    await expect(page.getByText(`New post for ${productName} ready to schedule.`)).not.toBeVisible();
  });
});