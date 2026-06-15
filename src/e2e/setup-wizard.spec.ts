import { test, expect } from './fixtures';

test.describe('Setup Wizard 375px Flow', () => {
    test.use({ viewport: { width: 375, height: 812 } });

    test('should render properly and allow selection', async ({ page }) => {
        // Go to setup wizard
        await page.goto('/ui/setup.html');
        await expect(page).toHaveTitle(/OHC Setup/);

        // Wait for page to be ready
        await page.waitForLoadState('domcontentloaded');

        // Check if step-initial is active
        const initialStep = page.locator('#step-initial');
        await expect(initialStep).toBeVisible();

        // Click Start My Business
        await page.getByTestId('next-step-btn').first().click();

        // Step Context
        const stepContext = page.locator('#step-context');
        await expect(stepContext).toBeVisible();
        await expect(stepContext).not.toHaveCSS('overflow-x', 'scroll'); // No horizontal scroll

        // Click Storefront context card
        const storefrontCard = page.getByTestId('context-storefront');
        await expect(storefrontCard).toBeVisible();
        await storefrontCard.click();

        // Assert the card has selected class
        await expect(storefrontCard).toHaveClass(/selected/);

        // Click next
        await page.locator('#step-context .next-step-btn').click();

        // Categories
        await page.locator('#business-categories').selectOption('Bakery');
        await page.locator('#step-categories .next-step-btn').click();

        // Business Name
        await page.locator('#business-name').fill('My Cool Bakery');
        await page.locator('#step-name .next-step-btn').click();

        // Assistant Setup
        await page.locator('#assistant-name').fill('Jarvis');
        await page.locator('#assistant-tone').selectOption('Professional');

        // Check toggles
        const draftToggle = page.getByTestId('cap-draft');
        await expect(draftToggle).toBeChecked();

        const scheduleToggle = page.getByTestId('cap-schedule');
        await expect(scheduleToggle).toBeChecked();
        await scheduleToggle.uncheck();
        await expect(scheduleToggle).not.toBeChecked();

        await page.locator('#step-assistant .next-step-btn').click();

        // Verify we reached Admin setup step
        await expect(page.locator('#step-admin')).toBeVisible();

        // Admin Setup
        await page.locator('#admin-email').fill('admin@mycoolbakery.com');
        await page.locator('#admin-password').fill('securepassword123');
        await page.locator('#step-admin .next-step-btn').click();

        // Offer Setup
        await page.locator('#first-offer').fill('Custom Wedding Cake');
        await page.locator('#step-offer .next-step-btn').click();

        // Template Setup
        await page.locator('#template-selection').selectOption('Modern');
        await page.locator('#finish-btn').click();

        // We expect to get redirected to success page
        await expect(page).toHaveURL(/\/ui\/success\.html/);

        // And check if Go to Assistant button is there
        await expect(page.locator('#dashboard-btn')).toBeVisible();
    });
});
