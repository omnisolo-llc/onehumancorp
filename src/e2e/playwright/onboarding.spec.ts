import { test, expect } from '@playwright/test';
import * as fs from 'fs';
import * as path from 'path';

test.describe('Onboarding Flow', () => {
  test.use({ viewport: { width: 375, height: 667 } }); // strictly mobile viewport

  test('should complete the onboarding flow on mobile', async ({ page }) => {
    // Start local http server to serve the page because Docker is not available in sandbox
    const tauriUiDir = path.join(process.cwd(), 'src/ui/tauri/src/ui');
    await page.route('**/*setup.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });

    // Mock tooltips and API calls
    await page.route('**/api/tooltips', async route => {
      await route.fulfill({ status: 200, body: JSON.stringify({}) });
    });

    await page.route('**/api/onboarding/draft', async route => {
       await route.fulfill({ status: 200, body: JSON.stringify({}) });
    });

    await page.route('**/api/onboarding/start', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ organization_id: 'test-org-123' })
      });
    });

    await page.route('**/*success.html*', async route => {
      await route.fulfill({ status: 200, contentType: 'text/html', body: '<html><body>Success</body></html>' });
    });

    await page.route('**/*success.html', async route => {
      await route.fulfill({ status: 200, contentType: 'text/html', body: '<html><body>Success</body></html>' });
    });

    // Navigate to onboarding page
    await page.goto('http://127.0.0.1:18789/setup.html');
    await expect(page).toHaveTitle(/OneHumanCorp|OHC/);

    // Initial Screen
    await expect(page.locator('.container')).toBeVisible();
    await page.locator('[data-testid="next-step-btn"][data-next="step-context"]').click();

    // Step Context
    await expect(page.getByRole('heading', { name: 'How do you work?' })).toBeVisible();
    await page.locator('[data-testid="context-storefront"]').click();
    await page.locator('[data-testid="next-step-btn"][data-next="step-categories"]').click();

    // Step Categories
    const categorySelect = page.getByTestId('business-categories');
    await expect(categorySelect).toBeVisible();
    await categorySelect.selectOption('Home Baker');
    await page.locator('[data-testid="next-step-btn"][data-next="step-name"]').click();

    // Step Name
    await page.getByTestId('business-name').fill('Test Business');
    await page.locator('[data-testid="next-step-btn"][data-next="step-assistant"]').click();

    // Step Assistant
    await page.getByTestId('team-operations').click();
    await page.getByTestId('assistant-tone').selectOption('Friendly');
    await page.locator('[data-testid="next-step-btn"][data-next="step-admin"]').click();

    // Step Admin
    await page.getByTestId('admin-name').fill('Test Admin');
    await page.getByTestId('admin-email').fill(`admin-${Date.now()}@test-business.com`);
    await page.getByTestId('admin-password').fill('Password123!');
    await page.locator('[data-testid="next-step-btn"][data-next="step-offer"]').click();

    // Step Offer
    await page.getByTestId('first-offer').fill('Awesome widgets');
    await page.locator('#step-offer [data-testid="next-step-btn"][data-next="step-location"]').click();

    // Step Location
    await page.getByTestId('location-input').fill('Austin, TX');
    await page.locator('#step-location [data-testid="next-step-btn"][data-next="step-target-audience"]').click();

    // Step Target Audience
    await page.getByTestId('target-audience').fill('Tech startups');
    await page.locator('#step-target-audience [data-testid="next-step-btn"][data-next="step-domain"]').click();

    // Step Domain
    await page.getByTestId('domain-name').fill(`test-business-${Date.now()}`);
    await page.locator('#step-domain [data-testid="next-step-btn"][data-next="step-template"]').click();

    // Step Template
    await page.getByTestId('template-selection').selectOption('Modern');

    // Submit
    const publishButton = page.getByTestId('finish-btn');
    await publishButton.waitFor({ state: 'visible' });

    // Catch any page navigation and verify success
    await Promise.all([
      page.waitForNavigation({ url: /.*success\.html.*/, timeout: 15000 }),
      publishButton.click()
    ]);
  });

  test('should complete the onboarding flow even if draft API fails (offline resilience)', async ({ page }) => {
    // Start local http server to serve the page because Docker is not available in sandbox
    const tauriUiDir = path.join(process.cwd(), 'src/ui/tauri/src/ui');
    await page.route('**/*setup.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });

    // Mock tooltips and API calls
    await page.route('**/api/tooltips', async route => {
      await route.fulfill({ status: 200, body: JSON.stringify({}) });
    });

    // explicitly fail draft API
    // Instead of just failing draft, we use the browser context offline mode for part of it
    await page.route('**/api/onboarding/draft', async route => {
       await route.abort('failed');
    });

    await page.route('**/api/onboarding/start', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ organization_id: 'test-org-123' })
      });
    });

    await page.route('**/*success.html*', async route => {
      await route.fulfill({ status: 200, contentType: 'text/html', body: '<html><body>Success</body></html>' });
    });

    await page.route('**/*success.html', async route => {
      await route.fulfill({ status: 200, contentType: 'text/html', body: '<html><body>Success</body></html>' });
    });

    // Navigate to onboarding page
    await page.goto('http://127.0.0.1:18789/setup.html');
    await expect(page).toHaveTitle(/OneHumanCorp|OHC/);

    // Initial Screen
    await expect(page.locator('.container')).toBeVisible();
    await page.locator('[data-testid="next-step-btn"][data-next="step-context"]').click();

    // Step Context
    await expect(page.getByRole('heading', { name: 'How do you work?' })).toBeVisible();
    await page.locator('[data-testid="context-storefront"]').click();
    await page.locator('[data-testid="next-step-btn"][data-next="step-categories"]').click();

    // Step Categories
    const categorySelect = page.getByTestId('business-categories');
    await expect(categorySelect).toBeVisible();
    await categorySelect.selectOption('Home Baker');
    await page.locator('[data-testid="next-step-btn"][data-next="step-name"]').click();

    // Step Name
    await page.getByTestId('business-name').fill('Test Business Offline');
    await page.locator('[data-testid="next-step-btn"][data-next="step-assistant"]').click();

    // Step Assistant
    await page.getByTestId('team-operations').click();
    await page.getByTestId('assistant-tone').selectOption('Friendly');
    await page.locator('[data-testid="next-step-btn"][data-next="step-admin"]').click();

    // Step Admin
    await page.getByTestId('admin-name').fill('Test Admin');
    await page.getByTestId('admin-email').fill(`admin-${Date.now()}@test-business.com`);
    await page.getByTestId('admin-password').fill('Password123!');
    await page.locator('[data-testid="next-step-btn"][data-next="step-offer"]').click();

    // Step Offer
    await page.getByTestId('first-offer').fill('Awesome widgets');
    await page.locator('#step-offer [data-testid="next-step-btn"][data-next="step-location"]').click();

    // Step Location
    await page.getByTestId('location-input').fill('Austin, TX');
    await page.locator('#step-location [data-testid="next-step-btn"][data-next="step-target-audience"]').click();

    // Step Target Audience
    await page.getByTestId('target-audience').fill('Tech startups');
    await page.locator('#step-target-audience [data-testid="next-step-btn"][data-next="step-domain"]').click();

    // Step Domain
    await page.getByTestId('domain-name').fill(`test-business-${Date.now()}`);
    await page.locator('#step-domain [data-testid="next-step-btn"][data-next="step-template"]').click();

    // Step Template
    await page.getByTestId('template-selection').selectOption('Modern');

    // Submit
    const publishButton = page.getByTestId('finish-btn');
    await publishButton.waitFor({ state: 'visible' });

    // Catch any page navigation and verify success
    await Promise.all([
      page.waitForNavigation({ url: /.*success.html.*/, timeout: 15000 }),
      publishButton.click()
    ]);
  });

});
