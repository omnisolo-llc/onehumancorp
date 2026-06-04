import { test, expect } from './fixtures';

test('Wizard Refinement E2E - keeps the setup flow plain-language and reversible', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    await page.goto('/onboarding');

    // Step 1: Tell us about your business
    await expect(page.getByRole('heading', { name: 'Tell us about your business' })).toBeVisible();
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    await page.getByPlaceholder("e.g. Maya's Custom Cakes").fill("Test Bakery");
    await page.getByRole('button', { name: 'Next' }).click();

    await expect(page.getByRole('heading', { name: 'What do you sell?' })).toBeVisible();
    await page.getByPlaceholder("e.g. I bake custom vegan cakes for weddings and parties...").fill("I sell vegan cakes");
    await page.getByRole('button', { name: 'Next' }).click();

    await expect(page.getByRole('heading', { name: 'Where are you located?' })).toBeVisible();
    await page.getByRole('button', { name: 'Back' }).click();
    await expect(page.getByRole('heading', { name: 'What do you sell?' })).toBeVisible();
});

test('Wizard Refinement E2E - exposes brand tone selection in Style & Team', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');

    // Setup state for step 3 directly using local storage
    await page.addInitScript(() => {
        window.localStorage.setItem('onboarding-storage-v3', JSON.stringify({
            state: { step: 3, chatStep: 3, businessName: 'Test Bakery', businessType: 'Bakery', categories: ['food'], firstProductName: 'Cake', firstProductPrice: '10', brandTone: 'Professional', aiAgents: [], aiAutoRespond: true, domainChoice: 'subdomain' }
        }));
    });

    await page.goto('/onboarding');
    await expect(page.getByRole('heading', { name: 'Style & Team' })).toBeVisible();
    await expect(page.getByText('Brand Tone')).toBeVisible();

    // Interact with brand tone
    const casualTone = page.getByText('Casual', { exact: true });
    await expect(casualTone).toBeVisible();
    await casualTone.click();

    // Verify it was selected (check class for selection)
    // The parent div of 'Casual' should have bg-[#0066FF]/10 text-[#0066FF]
    await expect(casualTone.locator('..')).toHaveClass(/text-\[#0066FF\]/);
});

test('Wizard Refinement E2E - exposes AI helper and prompt tuning areas', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    await page.goto('/dashboard');
    await page.getByRole('button', { name: 'Manage AI Assistants' }).click();
    await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible();
    await expect(page.getByText('Marketing Pro')).toBeVisible();
});

test('Wizard Refinement E2E - settings remain accessible from dashboard quick actions', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    await page.goto('/dashboard');
    await page.getByRole('button', { name: 'Settings', exact: true }).click();
    await expect(page.getByRole('heading', { name: 'Settings' })).toBeVisible();
    await expect(page.getByText('Enable Email Notifications')).toBeVisible();
});
