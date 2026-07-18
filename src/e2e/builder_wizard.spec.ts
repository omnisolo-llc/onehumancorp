import { test, expect } from './fixtures';

test.describe('Builder Wizard E2E Flow', () => {
  test('Persona: Business Owner completes the Store Builder Setup', async ({ page, loginAs, adminUser }) => {
    // 1. Owner starts wizard directly from the dashboard or /builder route.
    await loginAs(page, adminUser);
    await page.goto('/builder');

    // Expect the initial "What are you building today?" screen
    await expect(page.getByRole('heading', { name: 'What are you building today?' })).toBeVisible();

    // 2. Advance to Step 1
    await page.getByRole('button', { name: 'Selling Products' }).click();
    await expect(page.getByRole('heading', { name: "Let's build your store" })).toBeVisible();

    // 3. Fill in Business Name and Category (Step 1)
    await page.getByPlaceholder("e.g. Acme Corp").fill('Maya Cakes');
    await page.getByPlaceholder("e.g. Retail, Consulting, Tech").fill('Bakery');

    // Verify Validation Indicators
    await expect(page.getByText('✓ Looks good')).toBeVisible();
    await expect(page.getByText('✓ Sounds great')).toBeVisible();

    // Click Next
    await page.getByRole('button', { name: 'Next: Choose Vibe' }).click();

    // 4. Select Vibe (Step 2)
    await expect(page.getByRole('heading', { name: 'Select Your Vibe' })).toBeVisible();
    await page.getByRole('button', { name: /Friendly/ }).click(); // The icon + label will match this
    await page.getByRole('button', { name: 'Next: Details' }).click();

    // 5. Fill Details (Step 3)
    await expect(page.getByRole('heading', { name: 'Final Details' })).toBeVisible();
    await page.getByPlaceholder(/e\.g\. I run a mobile dog grooming service/i).fill('I bake amazing custom cakes.');

    // Ensure 'Progress saved' is visible
    await expect(page.getByText('Progress saved')).toBeVisible();

    // The 'Build Store' button should be active and we click it
    await page.getByRole('button', { name: 'Build Store' }).click();

    // 6. Verify transition to generating state and then selection state
    await expect(page.getByText('AI Architect')).toBeVisible();
    await expect(page.getByText('Designing your custom storefront...')).toBeVisible();

    // Wait for the "Pick your draft" screen to appear
    await expect(page.getByRole('heading', { name: 'Pick your draft' })).toBeVisible({ timeout: 15000 });

    // Select the second draft
    await page.getByText('Draft 2').click();
    await page.getByRole('button', { name: 'Customize Selected Draft' }).click();

    // 7. Verify we entered the Editor
    await expect(page.getByText('Mobile Editor')).toBeVisible();
  });
});
