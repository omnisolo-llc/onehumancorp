import { test, expect } from './fixtures';

test.describe('Onboarding Wizard', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('Maya (The Home Baker) onboarding flow', async ({ page }) => {
    // 1. Acquisition & Onboarding start
    await page.goto('/website-builder');

    // Simulate clicking "Build your bakery in 3 minutes"
    await expect(page.locator('#setup-screen')).toBeVisible();
    await page.getByRole('button', { name: /Start My Business Next/ }).click();

    // 2. Simplified Mobile First Onboarding inputs
    // The design doc specifies we just ask for Name and Category to defer friction

    await expect(page.getByRole('heading', { name: 'What kind of business are you building?' })).toBeVisible();
    // Simulate picking "Bakery" -> We'll pick Local Business for now since Bakery isn't in this specific builder.
    await page.getByRole('button', { name: /Local Business/ }).click();

    await expect(page.getByRole('heading', { name: 'Give your business a name' })).toBeVisible();
    await page.getByPlaceholder('What is your business called?').fill("Maya's Cakes");
    await page.getByRole('button', { name: /Next/ }).click();

    // Defer Stripe connection to Activation (We should skip this if possible but for testing we will follow the setup logic or what is simulated)
    await expect(page.getByRole('heading', { name: 'What do you sell?' })).toBeVisible();
    await page.getByLabel(/Physical Products/).check();
    await page.getByRole('button', { name: /Next/ }).click();

    await page.getByPlaceholder('What is the name of this product?').fill('Custom Cake Deposit');
    await page.getByPlaceholder('0.00').fill('50.00');
    await page.getByRole('button', { name: /Next/ }).click();

    // Approves & connects Stripe (simulated in UI)
    await expect(page.getByRole('heading', { name: 'How do you want to receive payments?' })).toBeVisible();
    await page.getByRole('button', { name: 'Online', exact: true }).click();

    // Account details
    await page.getByPlaceholder('e.g. Maya Smith').fill('Maya');
    await page.getByPlaceholder('you@email.com').fill('maya@cakes.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: /Next/ }).click();

    // Templates
    await page.getByRole('button', { name: 'Modern' }).click();
    await page.getByRole('button', { name: /Next/ }).click();

    // Publish
    await page.getByRole('button', { name: /Free OHC Domain/ }).click();
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: /Publish my business/ }).click();

    // 3. Activation
    await expect(page.getByRole('heading', { name: 'CONFETTI SUCCESS' })).toBeVisible({ timeout: 15000 });
  });
});
