import { expect, test } from '@playwright/test';
import * as fs from 'fs';

test.describe('Unified Agent Feed Viewport Constraint', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should render properly and not exceed 375px', async ({ page }) => {
    test.setTimeout(180000);

    await page.goto('/dashboard');
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    const feedContainer = page.locator('.glassmorphism').first();
    await expect(feedContainer).toBeVisible({ timeout: 15000 });

    const buttons = page.locator('div.glassmorphism button');
    const buttonCount = await buttons.count();
    let failedButtons = [];
    for (let i = 0; i < buttonCount; i++) {
        const box = await buttons.nth(i).boundingBox();
        if (box && box.height < 44) {
            const html = await buttons.nth(i).evaluate(node => node.outerHTML);
            failedButtons.push({ html, height: box.height });
        }
    }
    fs.writeFileSync('/tmp/failed_buttons.json', JSON.stringify(failedButtons, null, 2));
    expect(failedButtons.length).toBe(0);
  });
});
