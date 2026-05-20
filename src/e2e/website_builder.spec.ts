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

  test('generates storefront via AI', async ({ page }) => {
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

    // We no longer intercept API. Instead, we let the real MinimaxClient generate the storefront
    // using the provided prompt. We use waitForResponse to ensure the backend actually answered.
    const responsePromise = page.waitForResponse(res => res.url().includes('/api/v1/builder/generate') && res.status() === 200, { timeout: 30000 });
    await page.getByRole('button', { name: 'Generate Storefront →' }).click();

    const response = await responsePromise;
    const body = await response.json();

    // We can't guarantee exact text like "Fresh Cakes", but we know it should have pages and blocks
    expect(body.pages).toBeDefined();
    expect(body.pages.length).toBeGreaterThan(0);
    expect(body.pages[0].blocks).toBeDefined();
    expect(body.pages[0].blocks.length).toBeGreaterThan(0);

    // We should be redirected to the builder preview container eventually
    await expect(page.locator('#builder-preview-container')).toBeVisible({ timeout: 15000 });
  });

  test('publishes storefront with generated payload', async ({ page }) => {
    await page.goto('/storefront-builder');

    // Need some draft payload to be generated first to simulate a real flow.
    // However, since we are directly visiting /storefront-builder, it relies on initial state.
    // Let's ensure the payload publish button exists and we can click it.
    await page.getByRole('button', { name: 'Publish Changes' }).click();
    await page.getByRole('button', { name: /Free OHC Subdomain/ }).click();
    await page.getByPlaceholder('mybusiness').fill('real-baked-goods');

    // We no longer intercept. It hits the real db and enqueue jobs.
    const requestPromise = page.waitForRequest('/api/v1/builder/publish_draft');
    const responsePromise = page.waitForResponse(res => res.url().includes('/api/v1/builder/publish_draft') && res.status() === 200);

    await page.getByRole('button', { name: 'Publish' }).click();

    const request = await requestPromise;
    const response = await responsePromise;

    const requestBody = request.postDataJSON();
    expect(requestBody).toBeDefined();
    expect(requestBody.domain).toBe('real-baked-goods');

    const responseBody = await response.json();
    expect(responseBody.domain).toBe('real-baked-goods');
    expect(responseBody.id).toBeDefined();
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
});
