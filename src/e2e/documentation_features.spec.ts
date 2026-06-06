import { test, expect } from './fixtures';

test.describe('Help Center Page', () => {
  test('should load help center and navigate to article', async ({ page }) => {
    await page.goto('/help');

    await expect(page.getByRole('heading', { name: 'Help Center' })).toBeVisible();

    await expect(page.locator('h2:has-text("Getting Started")')).toBeVisible();

    await page.locator('h2:has-text("Getting Started")').click();

    await expect(page.getByRole('heading', { name: 'Getting Started' })).toBeVisible();

    await expect(page.locator('text=Welcome to OneHumanCorp!')).toBeVisible();
  });

  test('should filter articles using search input', async ({ page }) => {
    await page.goto('/help');

    // Type into the search box
    const searchBox = page.getByPlaceholder('Search for help articles and videos...');
    await searchBox.fill('AI');

    // Wait for the UI to update based on search
    await expect(page.locator('h2:has-text("Your AI Helpers")')).toBeVisible();

    // Verify a non-matching article is hidden
    await expect(page.locator('h2:has-text("Getting Started")')).not.toBeVisible();
  });
});

test.describe('API Documentation', () => {
  test('should load Swagger UI', async ({ page }) => {
    await page.goto('/api-docs');

    await page.waitForLoadState('domcontentloaded');

    await expect(page.locator('.swagger-ui')).toBeVisible();

    await expect(page.locator('text=OHC Advanced API Reference').first()).toBeVisible();

    await expect(page.locator('text=This section is for developers directly integrating with our APIs')).toBeVisible();
  });
});

test.describe('Release Notes and Changelog', () => {
  test('should load changelog page', async ({ page }) => {
    await page.goto('/changelog');

    await expect(page.getByRole('heading', { name: 'Release Notes & Changelog' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Version 1.0 (Latest)' })).toBeVisible();
    await expect(page.locator('text=Interactive AI Store Builder:')).toBeVisible();
  });
});

test.describe('Help Chat Widget', () => {
  test('should verify widget functionality', async ({ page }) => {
    const response = await page.request.post('/api/chat', {
        data: { message: "How do I add a product?" }
    });

    expect(response.status()).toBe(200);
    const result = await response.json();
    expect(result.reply).toBeDefined();
    expect(result.link).toBeDefined();
  });
});

test.describe('Video Tutorials in Help Widget', () => {
  test('should verify video endpoint', async ({ page }) => {
    const response = await page.request.get('/api/videos');
    expect(response.status()).toBe(200);

    const result = await response.json();
    expect(result.length).toBeGreaterThan(0);
    expect(result[0].title).toBeDefined();
    expect(result[0].duration).toBeDefined();
  });
});

test.describe('Tooltips', () => {
  test('should show tooltip on hover', async ({ page }) => {
    await page.goto('/dashboard');

    // the label uses withTooltip which renders a div block, find it by text and wait for tooltips to load
    await page.waitForResponse(response => response.url().includes('/api/tooltips') && response.status() === 200);

    const tooltipTarget = page.locator('div', { hasText: 'Total Sales' }).last();
    await expect(tooltipTarget).toBeVisible();

    // hover target
    await tooltipTarget.hover();

    const tooltip = page.locator('text=Total revenue generated from your sales today.');
    await expect(tooltip).toBeVisible();
  });
});

test.describe('Walkthrough', () => {
  test('should open walkthrough and show steps', async ({ page }) => {
    // Navigate with a specific test parameter if your component listens for it
    await page.goto('/dashboard');

    // Make sure we have the parameter stored in local storage before navigating again
    await page.evaluate(() => window.localStorage.setItem('TEST_WALKTHROUGH', 'true'));

    await page.goto('/dashboard');

    // open help widget
    await page.locator('button[aria-label="Help"]').click();

    // click tour button
    await page.locator('text=Tour: Activate your AI Support Agent').click();

    // assert walkthrough step is visible
    await expect(page.locator('text=Activate your AI agent.')).toBeVisible();

    // click close button inside the dialogue
    await page.locator('div[role="dialog"] button').click();

    // assert walkthrough is closed
    await expect(page.locator('text=Activate your AI agent.')).not.toBeVisible();
  });
});
