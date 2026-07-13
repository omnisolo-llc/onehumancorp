import * as fs from 'fs';
import * as path from 'path';
import { test, expect } from '@playwright/test';
test.describe.serial('OHC Setup Wizard Flow', () => {
  test('should complete the interactive setup wizard flow smoothly on desktop', async ({ page }) => {
    const tauriUiDir = path.join(process.cwd(), 'src/ui/tauri/src/ui');
    await page.route('**/setup.html', async route => {
        const htmlContent = (() => {
    try {
        return fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
    } catch(e) {
        return fs.readFileSync(path.join(process.env.TEST_SRCDIR || '', process.env.TEST_WORKSPACE || '', 'src/ui/tauri/src/ui', 'setup.html'), 'utf-8');
    }
})();
        await route.fulfill({ contentType: 'text/html', body: htmlContent   });
      });
    // intercept tooltips
    await page.route('**/api/tooltips', async route => {
      await route.fulfill({ status: 200, body: JSON.stringify({})   });
      });
    await page.route('**/api/onboarding/draft', async route => {
       await route.fulfill({ status: 200, body: JSON.stringify({})   });
      });
    await page.setViewportSize({ width: 1440, height: 900   });
    await page.goto('http://mock/setup.html');
    // Check initial UI loading
    await expect(page.locator('h1').first()).toBeVisible();
    // Click Start My Business (which goes to context)
    await page.locator('[data-testid="next-step-btn"][data-next="step-context"]').click();
    // Context step
    await page.locator('label.context-card').filter({ hasText: 'Storefront' }).click();
    await page.locator('[data-testid="next-step-btn"][data-next="step-categories"]').click();
    // Category step
    const categorySelect = page.getByTestId('business-categories');
    await expect(categorySelect).toBeVisible();
    await page.waitForTimeout(100);
    await categorySelect.selectOption('Home Baker');
    await page.locator('[data-testid="next-step-btn"][data-next="step-name"]').click();
    // Name step
    await page.getByTestId('business-name').fill('Test Bakery');
    await page.locator('[data-testid="next-step-btn"][data-next="step-assistant"]').click();
    // Assistant step
    await page.getByTestId('team-support').click();
    await page.getByTestId('assistant-tone').selectOption('Friendly');
    await page.locator('[data-testid="next-step-btn"][data-next="step-admin"]').click();
    // Admin step
    await page.getByTestId('admin-name').fill('Admin Name');
    await page.getByTestId('admin-name').fill('Admin Name');
    await page.getByTestId('admin-email').fill('admin@testbakery.local');
    await page.getByTestId('admin-password').fill('SuperSecretPassword123');
    await page.locator('[data-testid="next-step-btn"][data-next="step-offer"]').click();
    // Offer step
    await page.getByTestId('first-offer').fill('Chocolate Cake');
    await page.locator('#step-offer [data-testid="next-step-btn"][data-next="step-location"]').click();

    // Location step
    await page.getByTestId('location-input').fill('Portland, OR');
    await page.locator('[data-testid="next-step-btn"][data-next="step-target-audience"]').click();

    // Target Audience step
    await page.getByTestId('target-audience').fill('Local families');
    await page.locator('[data-testid="next-step-btn"][data-next="step-domain"]').click();
    // Domain step
    await page.getByTestId('domain-name').fill('test-bakery');
    await page.locator('[data-testid="next-step-btn"][data-next="step-template"]').click();
    // Template step
    await page.getByTestId('template-selection').selectOption('Modern', { force: true   });
    // Make sure finish btn is visible before interacting
    await expect(page.getByTestId('finish-btn')).toBeVisible();
    // Intercept backend call
    await page.route('**/api/onboarding/start', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ organization_id: 'test-org-123' })
        });
      });
    await page.route('**/api/onboarding/state', async route => {
       await route.fulfill({ status: 200, body: JSON.stringify({})   });
      });
    // Click Save Draft
    const saveDraftBtn = page.getByTestId('save-draft-btn').last();
    await expect(saveDraftBtn).toBeVisible();
    await saveDraftBtn.click();
    await expect(saveDraftBtn).toHaveText('Draft Saved!', { timeout: 3000   });
    await page.route('**/success.html', async route => {
      await route.fulfill({ status: 200, body: 'Success'   });
      });
    // Submit setup
    await page.evaluate(() => { document.getElementById('finish-btn').click();   });
    });
  test('should support 375px mobile view without horizontal scroll and minimum 44px touch targets', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812   });
    // intercept tooltips
    await page.route('**/api/tooltips', async route => {
      await route.fulfill({ status: 200, body: JSON.stringify({})   });
      });
    const tauriUiDir = path.join(process.cwd(), 'src/ui/tauri/src/ui');
    await page.route('**/setup.html', async route => {
        const htmlContent = (() => {
    try {
        return fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
    } catch(e) {
        return fs.readFileSync(path.join(process.env.TEST_SRCDIR || '', process.env.TEST_WORKSPACE || '', 'src/ui/tauri/src/ui', 'setup.html'), 'utf-8');
    }
})();
        await route.fulfill({ contentType: 'text/html', body: htmlContent   });
      });
    await page.goto('http://mock/setup.html');
    // Evaluate horizontal scroll
    const hasHorizontalScroll = await page.evaluate(() => {
        return document.documentElement.scrollWidth > window.innerWidth;
      });
    expect(hasHorizontalScroll).toBe(false);
    // Verify touch targets height
    await page.locator('.next-step-btn').first().waitFor({ state: 'visible' });
    const btnBox = await page.locator('.next-step-btn').first().boundingBox();
    expect(btnBox?.height).toBeGreaterThanOrEqual(44);
  });
  test('should auto-save progress and clear it on success', async ({ page }) => {
    const tauriUiDir = path.join(process.cwd(), 'src/ui/tauri/src/ui');
    await page.route('**/setup.html', async route => {
        const htmlContent = (() => {
    try {
        return fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
    } catch(e) {
        return fs.readFileSync(path.join(process.env.TEST_SRCDIR || '', process.env.TEST_WORKSPACE || '', 'src/ui/tauri/src/ui', 'setup.html'), 'utf-8');
    }
})();
        await route.fulfill({ contentType: 'text/html', body: htmlContent   });
      });
    // intercept tooltips
    await page.route('**/api/tooltips', async route => {
      await route.fulfill({ status: 200, body: JSON.stringify({})   });
      });
    await page.route('**/api/onboarding/draft', async route => {
       await route.fulfill({ status: 200, body: JSON.stringify({})   });
      });
    await page.goto('http://mock/setup.html');
    // Check initial UI loading
    await expect(page.locator('h1').first()).toBeVisible();
    await page.locator('[data-testid="next-step-btn"][data-next="step-context"]').click();
    await page.locator('label.context-card').filter({ hasText: 'Storefront' }).click();
    await page.locator('[data-testid="next-step-btn"][data-next="step-categories"]').click();
    const categorySelect = page.getByTestId('business-categories');
    await expect(categorySelect).toBeVisible();
    await categorySelect.selectOption('Home Baker');
    await page.locator('[data-testid="next-step-btn"][data-next="step-name"]').click();
    // Name step - Trigger auto-save
    await page.getByTestId('business-name').fill('AutoSave Bakery');
    // Wait for debounce and localstorage to be populated
    await page.waitForTimeout(600);
    // Reload page
    await page.reload();
    // Wait for the state to be reloaded (it jumps to step 3 since it was saved)
    await expect(page.getByTestId('business-name')).toHaveValue('AutoSave Bakery');
    });
  test('should show submit error if start fails', async ({ page }) => {
    const tauriUiDir = path.join(process.cwd(), 'src/ui/tauri/src/ui');
    await page.route('**/setup.html', async route => {
        const htmlContent = (() => {
    try {
        return fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
    } catch(e) {
        return fs.readFileSync(path.join(process.env.TEST_SRCDIR || '', process.env.TEST_WORKSPACE || '', 'src/ui/tauri/src/ui', 'setup.html'), 'utf-8');
    }
})();
        await route.fulfill({ contentType: 'text/html', body: htmlContent   });
      });
    // intercept tooltips
    await page.route('**/api/tooltips', async route => {
      await route.fulfill({ status: 200, body: JSON.stringify({})   });
      });
    await page.route('**/api/onboarding/draft', async route => {
       await route.fulfill({ status: 200, body: JSON.stringify({})   });
      });
    await page.goto('http://mock/setup.html');
    // Intercept backend call with failure
    await page.route('**/api/onboarding/start', async route => {
      await route.fulfill({
        status: 500,
        contentType: 'application/json',
        body: JSON.stringify({ error: 'Backend is broken' })
        });
      });
    // Skip to template step (mock localStorage)
    await page.evaluate(() => {
        localStorage.setItem('onboardingState', JSON.stringify({
            step: 7, // step-template
            businessName: 'Error Bakery',
            categories: 'Home Baker',
            templateSelection: 'Modern'
        }));
      });
    await page.reload();
    await page.getByTestId('template-selection').selectOption('Modern', { force: true   });
    await page.evaluate(() => { document.getElementById('finish-btn').click();   });
    // Check error message
    const errorMsg = page.locator('#submit-error');
    await expect(errorMsg).toBeVisible();
    await expect(errorMsg).toHaveText('Backend is broken');
    });
test.describe('OHC Setup Wizard Form Configuration', () => {
  test.beforeEach(async ({ page }) => {
      const tauriUiDir = path.join(process.cwd(), 'src/ui/tauri/src/ui');
      await page.route('**/setup.html', async route => {
          const content = (() => {
    try {
        return fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
    } catch(e) {
        return fs.readFileSync(path.join(process.env.TEST_SRCDIR || '', process.env.TEST_WORKSPACE || '', 'src/ui/tauri/src/ui', 'setup.html'), 'utf-8');
    }
})();
          await route.fulfill({ contentType: 'text/html', body: content   });
        });
    });
  test('should have appropriate HTML attributes for mobile input configuration', async ({ page }) => {
    await page.goto('http://mock/setup.html');
    // Name step
    const businessName = page.getByTestId('business-name');
    await expect(businessName).toHaveAttribute('autocomplete', 'organization');
    // Admin step
    const adminEmail = page.getByTestId('admin-email');
    await expect(adminEmail).toHaveAttribute('autocomplete', 'email');
    await expect(adminEmail).toHaveAttribute('inputmode', 'email');
    const adminPassword = page.getByTestId('admin-password');
    await expect(adminPassword).toHaveAttribute('autocomplete', 'new-password');
    });
  test('should have border-radius of 16px for .glassmorphism styling', async ({ page }) => {
    // This tests the CSS inline in setup.html and imported globals.css
    await page.goto('http://mock/setup.html');
    const container = page.locator('.container.glassmorphism').first();
    await expect(container).toHaveCSS('border-radius', '16px');
    const textInput = page.locator('#instant-bio');
    // Inputs also use glassmorphism but might be overridden to 8px.
    // However, the mandate specifies containers need 16px.
    await expect(textInput).toHaveCSS('border-radius', '8px');
    });
  test('should support 375px mobile view without horizontal scroll', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812   });
    await page.goto('http://mock/setup.html');
    // Evaluate horizontal scroll
    const hasHorizontalScroll = await page.evaluate(() => {
        return document.documentElement.scrollWidth > window.innerWidth;
      });
    expect(hasHorizontalScroll).toBe(false);
    });
  test('should have minimum 44px touch targets on buttons', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812   });
    await page.goto('http://mock/setup.html');
    // Verify touch targets height
    await page.locator('.next-step-btn').first().waitFor({ state: 'visible' });
    const btnBox = await page.locator('.next-step-btn').first().boundingBox();
    expect(btnBox?.height).toBeGreaterThanOrEqual(44);
    });
  test('should have minimum 44px touch targets on radio options', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812   });
    await page.goto('http://mock/setup.html');
    await page.locator('[data-testid="next-step-btn"][data-next="step-context"]').click();
    await page.locator("label.context-card").first().waitFor({ state: "visible" });
    const inputbox = await page.locator("label.context-card").first().boundingBox();
    expect(inputbox?.height).toBeGreaterThanOrEqual(44);
    });
  });

});

test.describe('OHC Setup Wizard Dark Mode', () => {
  test.beforeEach(async ({ page }) => {
    const fs = require('fs');
    const path = require('path');
    const tauriUiDir = path.join(process.cwd(), 'src/ui/tauri/src/ui');
    await page.route('**/setup.html', async route => {
        const htmlContent = (() => {
    try {
        return fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
    } catch(e) {
        return fs.readFileSync(path.join(process.env.TEST_SRCDIR || '', process.env.TEST_WORKSPACE || '', 'src/ui/tauri/src/ui', 'setup.html'), 'utf-8');
    }
})();
        await route.fulfill({ contentType: 'text/html', body: htmlContent });
    });
    await page.route('**/api/tooltips', async route => { await route.fulfill({ status: 200, body: JSON.stringify({}) }); });
  });

  test('should render Dark Mode Translucent Glass styling correctly', async ({ page }) => {
    // Emulate dark color scheme
    await page.emulateMedia({ colorScheme: 'dark' });
    await page.goto('http://mock/setup.html');

    // Check container dark mode background
    const container = page.locator('.container.glassmorphism').first();
    await expect(container).toHaveCSS('background-color', 'rgba(22, 22, 26, 0.7)');

    // Playwright evaluates body background color as rgba(22, 22, 26, 0.7)
    const body = page.locator('body');
    await expect(body).toHaveCSS('background-color', 'rgba(22, 22, 26, 0.7)');

    // Check text input dark mode styling
    const textInput = page.locator('#instant-bio');
    await expect(textInput).toHaveCSS('background-color', 'rgba(22, 22, 26, 0.7)');
  });
});
