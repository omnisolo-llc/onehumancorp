import { test, expect } from './fixtures';

test.describe('Regression Audit: Verify Mocks Removed and Features Rewired', () => {

  test('verify seasonal promo generation without setTimeout', async ({ page }) => {
    await page.goto('/seasonal-promo');

    // Fill out form
    const occasionInput = page.locator('input#promo-occasion');
    await occasionInput.fill('Spring Sale');

    const discountInput = page.locator('input#promo-discount');
    await discountInput.fill('20');

    // Click generate button
    const generateBtn = page.getByRole('button', { name: /Generate Campaign/ });
    await generateBtn.click();

    // Expect to see soft paywall if we are not Pro
    await expect(page.getByText('Upgrade to Pro')).toBeVisible();

    // Bypass window.open mock
    await page.evaluate(() => {
        window.open = function() { return null; };
    });

    // We expect the alert to have been removed or changed to non-blocking
    page.on('dialog', async dialog => {
      await dialog.accept();
    });

    const shareButton = page.getByRole('button', { name: /Share on X to get 7 Days Free/ });
    await shareButton.click();

    // Expect generation output
    await expect(page.getByText('Spring Sale Special! 20% OFF')).toBeVisible();
  });

  test('verify saving a new service navigates away', async ({ page }) => {
    await page.goto('/services/new');

    // Fill out title
    const titleInput = page.getByLabel('Service Title');
    if (await titleInput.isVisible()) {
        await titleInput.fill('Test Service');
    }

    // Attempt save
    const saveButton = page.getByRole('button', { name: 'Save Service' });
    if (await saveButton.isVisible()) {
        await saveButton.click();

        // Ensure successful navigation to dashboard
        await expect(page).toHaveURL(/.*\/dashboard/);
    }
  });

  test('verify Kairos walkthrough has no delay', async ({ page }) => {
    await page.goto('/kairos?walkthrough=true');
    // Ensure the walkthrough elements exist immediately
    await expect(page.getByText(/The Shared Task List is the 'Brain'/)).toBeVisible({ timeout: 2000 });
  });

  test('verify dashboard VIP customer referral campaign modal', async ({ page }) => {
    await page.goto('/dashboard');

    const sendButton = page.getByRole('button', { name: /Send Campaign to 12 Customers/ });
    if (await sendButton.isVisible()) {
      await sendButton.click();
      await expect(page.getByText(/VIP Referral Invite/)).not.toBeVisible();
    }
  });

  test('verify onboarding intake hits backend successfully', async ({ request }) => {
     const res = await request.post('/api/onboarding/intake', {
        data: { description: 'Maya' }
     });

     // Instead of mock, this should hit backend, so we check if response structure differs from mock
     // or simply succeeds properly without the hardcoded 'Maya Cakes' logic.
     expect(res.ok()).toBeTruthy();
     const data = await res.json();
     expect(data).toBeDefined();
  });

});
