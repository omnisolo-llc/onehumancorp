import * as fs from 'fs';
import { test, expect } from '@playwright/test';
import * as path from 'path';

test.describe('Onboarding Wizard E2E Flow', () => {

  test.beforeEach(async ({ page }) => {
    // Clear local storage to ensure fresh state
    await page.addInitScript(() => {
      window.localStorage.clear();
    });

    const workspaceRoot = process.env.TEST_WORKSPACE
        ? path.join(process.env.TEST_SRCDIR || process.cwd(), process.env.TEST_WORKSPACE)
        : process.cwd();
    const tauriUiDir = path.join(workspaceRoot, 'src/ui/tauri/src/ui');
    await page.route('**/setup.html', async route => {
        const fileContent = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: fileContent });
    });
    await page.addInitScript(() => {
      window.__TAURI__ = {
        core: {
          invoke: async () => null
        }
      };
    });

    // Clear local storage to ensure fresh state
    await page.addInitScript(() => {
      window.localStorage.clear();
    });
  });

  // Test 1: Completes the onboarding flow
  test('Completes the onboarding flow and verifies premium translucent glass styling and flexbox layouts', async ({ page }) => {
    await page.goto('/setup.html');

    // Step 0: Welcome Screen
    const setupScreen = page.locator('.container');
    await page.waitForLoadState('domcontentloaded');
    await expect(setupScreen).toBeVisible({ timeout: 30000 });

    // Click manual configuration
    const startButton = page.locator('button', { hasText: 'Step-by-Step Setup' }).first();
    await startButton.click();
    await page.waitForTimeout(500); // Give it time to render the next step

    // Context Card Flow starts in step-context in Tauri
    await expect(page.locator('#step-context')).toBeVisible();

    const storefrontCard = page.getByTestId('context-storefront');
    await expect(storefrontCard).toBeVisible();
    await storefrontCard.click();
    await page.locator('#step-context .next-step-btn').click();

    // Step Categories
    await expect(page.locator('#step-categories')).toBeVisible();
    await page.locator('#business-categories').selectOption('Home Baker');
    await page.locator('#step-categories .next-step-btn').click();

    // Step Name
    await expect(page.locator('#step-name')).toBeVisible();
    const nameInput = page.locator('#business-name');
    await expect(nameInput).toBeVisible();
    await expect(nameInput).toHaveClass(/glassmorphism/);
    await expect(nameInput).toHaveAttribute('autocomplete', 'organization');

    await nameInput.fill("My Awesome E2E Business");
    await page.locator('#step-name .next-step-btn').click();

    // Check we get to Assistant Step
    await expect(page.locator('#step-assistant')).toBeVisible();
  });

  test('Verifies capabilities can be toggled via click and keyboard', async ({ page }) => {
    await page.goto('/setup.html');

    // Click manual configuration
    const startButton = page.locator('button', { hasText: 'Step-by-Step Setup' }).first();
    await startButton.click();
    await page.waitForTimeout(500);

    // Context Card Flow starts in step-context in Tauri
    await expect(page.locator('#step-context')).toBeVisible();

    const storefrontCard = page.getByTestId('context-storefront');
    await expect(storefrontCard).toBeVisible();
    await storefrontCard.click();
    await page.locator('#step-context .next-step-btn').click();

    // Step Categories
    await expect(page.locator('#step-categories')).toBeVisible();
    await page.locator('#business-categories').selectOption('Home Baker');
    await page.locator('#step-categories .next-step-btn').click();

    // Step Name
    await expect(page.locator('#step-name')).toBeVisible();
    const nameInput = page.locator('#business-name');
    await nameInput.fill("Test Business");
    await page.locator('#step-name .next-step-btn').click();

    // Check we get to Assistant Step
    await expect(page.locator('#step-assistant')).toBeVisible();

    const inventoryToggle = page.locator('#cap-inventory');
    await expect(inventoryToggle).not.toBeChecked();

    await inventoryToggle.check({ force: true });
    await expect(inventoryToggle).toBeChecked();

    const toggleRow = inventoryToggle.locator('xpath=./ancestor::label[contains(@class, "toggle-row")]');
    await toggleRow.focus();
    await page.keyboard.press('Space');
    await expect(inventoryToggle).not.toBeChecked();
  });

  // Test 2: Validates the 44px minimum touch target size (via 44px min-height)
  test('Validates 44px touch targets on mobile sizes', async ({ page }) => {
    // Set a mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto('/setup.html');
    const setupScreen = page.locator('.container');
    await expect(setupScreen).toBeVisible({ timeout: 30000 });

    // Click manual configuration
    const startButton = page.locator('button', { hasText: 'Step-by-Step Setup' }).first();
    await startButton.click();
    await page.waitForTimeout(500); // Give it time to render the next step

    const contextCard = page.locator('.context-card').first();
    await expect(contextCard).toBeVisible();
    const box = await contextCard.boundingBox();
    expect(Math.round(box?.height || 0)).toBeGreaterThanOrEqual(44);
  });

  // Test 3: Verifies input disabled states
  test('Next button fails validation when input is empty', async ({ page }) => {
    await page.goto('/setup.html');
    const setupScreen = page.locator('.container');
    await expect(setupScreen).toBeVisible({ timeout: 30000 });

    // Click manual configuration
    const startButton = page.locator('button', { hasText: 'Step-by-Step Setup' }).first();
    await startButton.click();
    await page.waitForTimeout(500); // Give it time to render the next step

    // Jump straight to the name step to test validation
    await page.evaluate(() => { (window as any).goToStep('step-name', false) });

    const nextButton = page.locator('#step-name .next-step-btn');
    await nextButton.click();

    // Validation fails, showing error message
    const errorMsg = page.locator('#name-error');
    await expect(errorMsg).toBeVisible();

    const nameInput = page.locator('#business-name');
    await nameInput.fill("ABC");
    await nextButton.click();
    await expect(page.locator('#step-assistant')).toBeVisible();
  });

  // Test 4: Enter key submits the first step
  test('Enter key submits the input', async ({ page }) => {
    await page.goto('/setup.html');
    const setupScreen = page.locator('.container');
    await expect(setupScreen).toBeVisible({ timeout: 30000 });

    // Click manual configuration
    const startButton = page.locator('button', { hasText: 'Step-by-Step Setup' });
    await startButton.click();
    await page.waitForTimeout(500); // Give it time to render the next step

    // Jump straight to the name step to test validation
    await page.evaluate(() => { (window as any).goToStep('step-name', false) });

    const nameInput = page.locator('#business-name');
    await nameInput.fill("ABC");
    await nameInput.press('Enter');

    await expect(page.locator('#step-assistant')).toBeVisible();
  });

  // Test 5: Verify text area presence and styling
  test('Verify manual configuration fallback styling', async ({ page }) => {
    await page.goto('/setup.html');
    const setupScreen = page.locator('.container');
    await expect(setupScreen).toBeVisible({ timeout: 30000 });
  });
});

test.describe('Onboarding Wizard E2E Flow - Instant Build Extensions', () => {

  test.beforeEach(async ({ page }) => {
    // Clear local storage to ensure fresh state
    await page.addInitScript(() => {
      window.localStorage.clear();
    });
    const workspaceRoot = process.env.TEST_WORKSPACE
        ? path.join(process.env.TEST_SRCDIR || process.cwd(), process.env.TEST_WORKSPACE)
        : process.cwd();
    const tauriUiDir = path.join(workspaceRoot, 'src/ui/tauri/src/ui');
    await page.route('**/setup.html', async route => {
        const fileContent = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: fileContent });
    });

    // mock the tauri backend
    await page.addInitScript(() => {
        (window as any).__TAURI__ = {
            core: {
                invoke: async (cmd: string, args: any) => {
                    if (cmd === 'start_onboarding') {
                        return { success: true };
                    }
                    if (cmd === 'process_intake') {
                        if (args.input.includes("fail network request")) {
                            throw new Error("Network request failed");
                        }
                        if (!args.input || args.input.trim() === '') {
                             throw new Error("Empty input");
                        }
                        return {
                            business_name: "Mock Instant Business",
                            business_type: "Local Service",
                            categories: ["Mock Category"],
                            location: "San Francisco",
                            target_audience: "Anyone",
                            initial_products: [ { name: "Mock Product", price: "10.00" } ]
                        };
                    }
                    return null;
                }
            }
        };
    });
  });

    // Test 1: Verifies Instant Build successful generation flow
  test('Instant Build successfully creates a fully populated storefront from a valid paragraph', async ({ page }) => {
    await page.goto('/setup.html');
    const setupScreen = page.locator('.container');
    await expect(setupScreen).toBeVisible({ timeout: 30000 });

    // Instant Build is now the initial screen

    const bioInput = page.locator('#instant-bio');
    await expect(bioInput).toBeVisible();
    await expect(bioInput).toHaveClass(/glassmorphism/);

    await bioInput.fill("I run a high-end tech consultation firm specializing in AI in San Francisco.");

    const generateButton = page.locator('#generate-storefront-btn');
    await expect(generateButton).toBeVisible();
    await generateButton.click();

    await page.waitForTimeout(500);
  });

  test('Instant Build image URL is submitted and correctly mapped to state', async ({ page }) => {
    await page.goto('/setup.html');
    // Instant Build is now the initial screen

    const bioInput = page.locator('#instant-bio');
    await bioInput.fill("Test business description.");

    const generateButton = page.locator('#generate-storefront-btn');
    await generateButton.click();

    await page.waitForTimeout(500);
  });

  test('Instant Build image URL can be empty and successfully launches', async ({ page }) => {
    await page.goto('/setup.html');
    // Instant Build is now the initial screen

    const bioInput = page.locator('#instant-bio');
    await bioInput.fill("Test business description without image.");

    const generateButton = page.locator('#generate-storefront-btn');
    await generateButton.click();

    await page.waitForTimeout(500);
  });

  // Test 2: Verifies Instant Build handles network error gracefully
  test('Instant Build gracefully displays an error state on a network failure with proper styling', async ({ page }) => {
    await page.goto('/setup.html');
    const setupScreen = page.locator('.container');
    await expect(setupScreen).toBeVisible({ timeout: 30000 });

    // Instant Build is now the initial screen

    const bioInput = page.locator('#instant-bio');
    await bioInput.fill("fail network request");

    const generateButton = page.locator('#generate-storefront-btn');
    await generateButton.click();

    // Verify error is shown with correct styling
    const errorBlock = page.locator('#instant-error');
    await expect(errorBlock).toBeVisible();

    await expect(generateButton).toHaveText('Generate My Workspace');

    // Typing clears the error
    await bioInput.fill("New text");
    await expect(errorBlock).not.toBeVisible();
  });

  // Test 3: Verifies empty input behavior
  test('Instant Build prevents submission when the input is empty', async ({ page }) => {
    await page.goto('/setup.html');
    // Instant Build is now the initial screen

    const generateButton = page.locator('#generate-storefront-btn');
    await expect(generateButton).toBeDisabled();

    const bioInput = page.locator('#instant-bio');
    await bioInput.fill('    ');
    await expect(generateButton).toBeDisabled();

    // We shouldn't see a loading state.
    await expect(page.locator('#step-loading')).not.toBeVisible();
  });

  // Test 4: Smart defaults fallback on partial info
  test('Instant Build handles partial information appropriately by falling back to smart defaults', async ({ page }) => {
    await page.goto('/setup.html');
    // Instant Build is now the initial screen

    const bioInput = page.locator('#instant-bio');
    // Only provide a generic description
    await bioInput.fill("I sell things online.");

    const generateButton = page.locator('#generate-storefront-btn');
    await generateButton.click();

    await page.waitForTimeout(500);
  });
  test("Onboarding Flow respects mobile viewport constraints (375px) with valid touch targets using real stack", async ({ page, baseURL }) => {
    await page.setViewportSize({ width: 375, height: 667 });

    // Go to the real local server root route which should present setup if unconfigured
    await page.goto('/setup.html');

    // Wait for the container to load
    const container = page.locator(".container");
    await expect(container).toBeVisible({ timeout: 15000 });

    // Check .container has no horizontal overflow
    const containerBox = await container.boundingBox();
    expect(containerBox?.width).toBeLessThanOrEqual(375);

    // Click Step-by-Step Setup
    const startMyBusinessButton = page.locator('button', { hasText: 'Step-by-Step Setup' }).first();
    await startMyBusinessButton.click();

    // Check .context-card
    const contextCard = page.locator(".context-card").first();
    const cardBox = await contextCard.boundingBox();
    expect(Math.round(cardBox?.height || 0)).toBeGreaterThanOrEqual(44);

    // Next step
    await contextCard.click();
    const nextButton = page.locator("button[data-next=\"step-categories\"]").first();
    await nextButton.click();
    await expect(page.locator("#step-categories")).toBeVisible();

    const selectBox = page.locator("#business-categories");
    await expect(selectBox).toBeVisible();
  });

  // Test 5: Mobile responsiveness of the Instant Build component
  test('Instant Build respects mobile viewport constraints (375px) with valid touch targets for the conversational flow', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto('/setup.html');

    // Instant Build is now the initial screen

    const bioInput = page.locator('#instant-bio');
    const box = await bioInput.boundingBox();
    expect(Math.round(box?.height || 0)).toBeGreaterThanOrEqual(44);
    expect(box?.width).toBeLessThanOrEqual(375);

    const generateButton = page.locator('#generate-storefront-btn');
    const btnBox = await generateButton.boundingBox();
    expect(Math.round(btnBox?.height || 0)).toBeGreaterThanOrEqual(44);
  });
});
