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

    // Setup API interception
    await page.route('/api/v1/builder/generate', async route => {
        const json = {
            pages: [{
                blocks: [
                    { block_type: 'HeroBlock', content: { headline: 'Fresh Cakes', subtitle: 'Yum' } }
                ]
            }]
        };
        await route.fulfill({ json });
    });

    await page.getByRole('button', { name: 'Generate Storefront →' }).click();

    // Wait for mock backend parsing
    await expect(page.locator('#builder-preview-container')).toContainText('Fresh Cakes', { timeout: 10000 });
  });

  test('publishes storefront with generated payload', async ({ page }) => {
    await page.goto('/storefront-builder');

    // Simulate clicking publish and publishing
    await page.getByRole('button', { name: 'Publish Changes' }).click();
    await page.getByRole('button', { name: /Free OHC Subdomain/ }).click();
    await page.getByPlaceholder('mybusiness').fill('baked-goods');

    await page.route('/api/v1/builder/publish_draft', async route => {
        await route.fulfill({ json: { domain: 'baked-goods.ohc.app' } });
    });

    const requestPromise = page.waitForRequest('/api/v1/builder/publish_draft');
    await page.getByRole('button', { name: 'Publish' }).click();
    const requestBody = (await requestPromise).postDataJSON();

    expect(requestBody).toBeDefined();
    expect(requestBody.domain).toBe('baked-goods');
    expect(requestBody.draft.pages[0].blocks[0].block_type).toBe('HeroBlock');
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
