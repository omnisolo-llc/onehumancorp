import { test, expect } from './fixtures';

test.describe('Affiliate Hub Growth Loop', () => {
  test('User can navigate to Affiliate Hub and generate an affiliate link', async ({ adminPage }) => {
    const page = adminPage;

    // Check that we are on the dashboard
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();

    // 2. Navigate to Affiliate & Partner Hub via the new card
    // The link is on the "Open Partner Hub" button
    await page.getByRole('link', { name: '🤝 Open Partner Hub' }).click();

    // 3. Verify we are on the Affiliates page
    await expect(page.locator('h1').filter({ hasText: 'Affiliate & Partner Hub' })).toBeVisible();
    await expect(page.locator('h2').filter({ hasText: 'Scale with Partners' })).toBeVisible();

    // 4. Input the affiliate email and generate a link
    await page.fill('input[placeholder="e.g. partner@example.com"]', 'testpartner@ohc.app');

    // Change commission and discount values to make sure forms work
    const commissionInput = page.locator('input[type="number"]').first();
    await commissionInput.fill('20');

    const discountInput = page.locator('input[type="number"]').nth(1);
    await discountInput.fill('15');

    // 5. Click generate button
    await page.getByRole('button', { name: 'Generate Affiliate Link' }).click();

    // 6. Wait for success state
    await expect(page.getByText('Link Generated Successfully!')).toBeVisible({ timeout: 10000 });
  });
});
