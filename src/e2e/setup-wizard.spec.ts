import { test, expect } from './fixtures';
import * as path from 'path';
import * as fs from 'fs';

test.describe('Setup Wizard 375px Flow', () => {
    test.use({ viewport: { width: 375, height: 812 } });

    test('should render properly and allow selection', async ({ browser }) => {
        const workspaceRoot = process.env.TEST_WORKSPACE
            ? path.join(process.env.TEST_SRCDIR || process.cwd(), process.env.TEST_WORKSPACE)
            : process.cwd();

        const tauriUiDir = path.join('/app', 'src/ui/tauri/src/ui');

        const page = await browser.newPage();

        await page.route('http://mock/setup.html', async route => {
            const content = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
            await route.fulfill({ contentType: 'text/html', body: content });
        });

        // Go to setup wizard
        await page.goto('http://mock/setup.html');
        await expect(page).toHaveTitle(/OHC Setup/);

        // Wait for page to be ready
        await page.waitForLoadState('domcontentloaded');

        // Check if step-initial is active
        const initialStep = page.locator('#step-initial');
        await expect(initialStep).toBeVisible();

        // Click Step-by-Step Setup
        await page.getByTestId('next-step-btn').first().click();

        // Step Context
        const stepContext = page.locator('#step-context');
        await expect(stepContext).toBeVisible();
        await expect(stepContext).not.toHaveCSS('overflow-x', 'scroll'); // No horizontal scroll

        const personaRow = page.locator('.persona-row');
        await expect(personaRow).toHaveCSS('flex-direction', 'column');

        const workContextCards = page.locator('#context-group');
        await expect(workContextCards).toHaveCSS('flex-direction', 'column');

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
        await page.getByTestId('team-operations').click();
        await page.locator('#assistant-tone').selectOption('Professional');

        // Check toggles
        const draftToggle = page.getByTestId('cap-draft');
        await expect(draftToggle).toBeChecked();

        const scheduleToggle = page.getByTestId('cap-schedule');
        await expect(scheduleToggle).toBeChecked();
        await scheduleToggle.evaluate((node) => { (node as HTMLInputElement).checked = false; node.dispatchEvent(new Event('change')); });
        await expect(scheduleToggle).not.toBeChecked();

        await page.locator('#step-assistant .next-step-btn').click();

        // Verify we reached Admin setup step
        await expect(page.locator('#step-admin')).toBeVisible();

        await page.close();
    });
});
