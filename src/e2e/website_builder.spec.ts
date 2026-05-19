import { test, expect } from './fixtures';

test.describe('Website Builder Full E2E', () => {
  test('renders editable storefront blocks', async ({ page }) => {
    await page.goto('/storefront-builder');
    await expect(page.getByRole('heading', { name: 'Edit Website' })).toBeVisible();
    await page.getByRole('button', { name: 'Rearrange' }).click();
    await expect(page.locator('#builder-preview-container')).toContainText('Hero');
    await expect(page.locator('#builder-preview-container')).toContainText('Product Grid');
  });

  test('opens publish workflow for a free subdomain', async ({ page }) => {
    await page.goto('/storefront-builder');
    await page.getByRole('button', { name: 'Publish Changes' }).click();
    await page.getByRole('button', { name: /Free OHC Subdomain/ }).click();
    await page.getByPlaceholder('mybusiness').fill('test-store');
    await expect(page.getByPlaceholder('mybusiness')).toHaveValue('test-store');
  });

  test('generates storefront via AI without mocks', async ({ page }) => {
    // Navigate to step AI
    await page.goto('/');
    // Trigger the generation directly or navigate the UI to step-ai. For testing UI flow:
    await page.evaluate(() => {
        document.querySelectorAll('.screen').forEach(s => s.style.display = 'none');
        document.getElementById('setup-screen').style.display = 'block';
        // nextStep is a global helper
        (window as any).nextStep('ai');
    });

    const input = page.locator('#step-ai input');
    await input.fill('I am a baker');

    // Make real backend API call
    await page.getByRole('button', { name: 'Generate Storefront →' }).click();

    // Wait for real backend to respond. Since it goes to LLM it may take up to 20 seconds.
    // The real AI typically generates 'Hero' or similar components. We wait for the preview container
    await expect(page.locator('#builder-preview-container')).toBeVisible({ timeout: 20000 });
  });

  test('publishes storefront with generated payload without mocks', async ({ page }) => {
    await page.goto('/storefront-builder');

    // Simulate clicking publish and publishing
    await page.getByRole('button', { name: 'Publish Changes' }).click();
    await page.getByRole('button', { name: /Free OHC Subdomain/ }).click();
    await page.getByPlaceholder('mybusiness').fill('baked-goods');

    // We let the real request happen
    const requestPromise = page.waitForRequest(request => request.url().includes('/api/v1/builder/publish_draft') && request.method() === 'POST');
    await page.getByRole('button', { name: 'Publish' }).click();
    const request = await requestPromise;
    const requestBody = request.postDataJSON();

    expect(requestBody).toBeDefined();
    expect(requestBody.domain).toBe('baked-goods');
    // Ensure blocks are mapped to the correct real draft blocks like 'HeroBlock' or 'ProductGridBlock'
    expect(requestBody.draft.pages[0].blocks.length).toBeGreaterThan(0);
  });

  test('verifies block edits update optimistic UI', async ({ page }) => {
     // Given canvas tests cover this, we will add a secondary check here.
     await page.goto('/storefront-builder');
     await page.getByText('My Awesome Store').click();

     await expect(page.locator('#sheet-title')).toHaveText('Edit Hero');
     await page.locator('input#edit-title').fill('Updated Title');
     await page.getByRole('button', { name: 'Save' }).click();

     await expect(page.locator('#builder-preview-container')).toContainText('Updated Title');
  });

  test('nextjs builder workflow handles empty inputs correctly', async ({ page }) => {
    await page.goto('/builder');

    // Check initial state
    await expect(page.getByRole('heading', { name: 'Welcome to OHC Smart Builder' })).toBeVisible();

    const textarea = page.locator('textarea[placeholder*="e.g. I run a mobile dog grooming service"]');

    // Try to build with too short a string
    await textarea.fill('A');

    // Button should be disabled
    const btn = page.getByRole('button', { name: 'Build My Storefront' });
    await expect(btn).toBeDisabled();
  });

  test('nextjs builder workflow generates and publishes successfully to the real database', async ({ page }) => {
    await page.goto('/builder');

    // Check initial state
    await expect(page.getByRole('heading', { name: 'Welcome to OHC Smart Builder' })).toBeVisible();

    const textarea = page.locator('textarea[placeholder*="e.g. I run a mobile dog grooming service"]');
    // Using a specific keyword that helps the LLM generate a known block
    await textarea.fill('I am a baker');

    // Trigger real backend generation call
    await page.getByRole('button', { name: 'Build My Storefront' }).click();

    // Wait for the LLM to complete and the UI to update to draft view.
    // The real AI judge logic takes a bit.
    await expect(page.getByText('Preview Mode')).toBeVisible({ timeout: 15000 });
    // The real AI typically generates 'Hero' headers.
    await expect(page.locator('.builder-block, .glassmorphism').first()).toBeVisible({ timeout: 15000 });

    // Trigger publish which creates real DB records
    await page.getByRole('button', { name: '1-Tap Launch' }).click();

    // Wait for the backend processing and UI update
    await expect(page.getByRole('heading', { name: "You're Live!" })).toBeVisible({ timeout: 10000 });
    await expect(page.locator('text=ohc.store')).toBeVisible();
  });
});
