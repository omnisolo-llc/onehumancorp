import { test, expect } from '@playwright/test';

test.describe('Real Business Owner Documentation CUJ', () => {
  /*
    Business Owner Persona: Maya the Home Baker
    Maya wants to learn how to add products to her store and view the latest platform updates.

    CUJ Workflow Flow Chart:
    1. Start at Dashboard (or Home Page after login mock)
    2. Click Help Center (?)
    3. Verify Help Center loads
    4. Click on "My Store" article to read about adding products
    5. Verify the article content displays
    6. Go back to Help Center
    7. Click on "Watch Video Tutorials"
    8. Verify Video Tutorials page loads and shows video cards
    9. Click on Changelog from navigation (or verify it exists)
    10. Verify Changelog page loads and shows updates
  */

  test('Maya learns how to use the platform through documentation', async ({ page }) => {
    // 1. Navigate to home (which redirects to dashboard or onboarding)
    // We will go directly to help center as the starting point for this CUJ
    await page.goto('/help');

    // 3. Verify Help Center loads
    await expect(page.getByRole('heading', { name: 'Help Center' })).toBeVisible();
    await expect(page.getByPlaceholder('Search for help articles...')).toBeVisible();

    // 4. Click on "My Store" article
    // Since it's a link, we can click by text
    await page.getByText('My Store').click();

    // Wait for navigation
    await page.waitForURL('**/help/my-store');

    // 5. Verify the article content displays
    await expect(page.getByRole('heading', { name: 'Managing My Store' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Adding Products' })).toBeVisible();

    // 6. Go back to Help Center
    await page.getByRole('button', { name: '← Back to Help Center' }).click();
    await page.waitForURL('**/help');

    // 7. Click on "Watch Video Tutorials"
    await page.getByText('Watch Video Tutorials').click();

    // Wait for navigation
    await page.waitForURL('**/help/tutorials');

    // 8. Verify Video Tutorials page loads and shows video cards
    await expect(page.getByRole('heading', { name: 'Video Tutorials' })).toBeVisible();
    // Verify a video title is visible. Since data comes from our API, we know "Set up your store" should be there
    await expect(page.getByText('Set up your store')).toBeVisible();

    // 9. Go to Changelog
    await page.goto('/changelog');

    // 10. Verify Changelog page loads and shows updates
    await expect(page.getByRole('heading', { name: 'Release Notes & Changelog' })).toBeVisible();
    await expect(page.getByText('Interactive AI Store Builder:')).toBeVisible();

    // Extra Check: Go to API Docs (Advanced feature)
    await page.goto('/api-docs');
    await expect(page.getByText('This section is for developers directly integrating with our APIs.')).toBeVisible();
  });
});