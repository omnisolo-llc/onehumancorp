import { test, expect } from '@playwright/test';
import * as path from 'path';
import * as fs from 'fs';

test.describe('Assistant Workspace', () => {
  test('Renders all required layout sections', async ({ page }) => {
    const workspaceRoot = process.env.TEST_WORKSPACE
        ? path.join(process.env.TEST_SRCDIR!, process.env.TEST_WORKSPACE)
        : process.cwd();

    const tauriUiDir = path.join(workspaceRoot, 'src/ui/tauri/src/ui');

    await page.route('/assistant.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'assistant.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });

    await page.goto('/assistant.html');

    // Verify Title
    await expect(page).toHaveTitle('OHC Assistant Workstation');

    // Verify Top Bar Links
    await expect(page.locator('a[aria-label="Remote Control"]')).toBeVisible();
    await expect(page.locator('a[aria-label="Automations"]')).toBeVisible();
    await expect(page.locator('a[aria-label="Memory"]')).toBeVisible();
    await expect(page.locator('a[aria-label="Skills"]')).toBeVisible();
    await expect(page.locator('a[aria-label="Expert Center"]')).toBeVisible();

    // Verify Left Rail
    await expect(page.locator('.rail-title').filter({ hasText: 'Workspace' })).toBeVisible();
    await expect(page.locator('select[aria-label="Workspace Selector"]')).toBeVisible();
    await expect(page.locator('.rail-title').filter({ hasText: 'Active Tasks' })).toBeVisible();

    // Verify Center Conversation
    await expect(page.locator('h2[aria-label="Task Title"]')).toBeVisible();
    await expect(page.locator('div[aria-label="Task Status"]')).toBeVisible();
    await expect(page.locator('div[aria-label="Conversation View"]')).toBeVisible();
    await expect(page.locator('textarea[aria-label="Composer Input"]')).toBeVisible();

    // Verify Right Panel
    await expect(page.locator('div[aria-label="Results Panel"]')).toBeVisible();
    await expect(page.locator('div[aria-label="Tab: Artifacts"]')).toBeVisible();
    await expect(page.locator('div[aria-label="Tab: All Files"]')).toBeVisible();
    await expect(page.locator('div[aria-label="Tab: Changes"]')).toBeVisible();
    await expect(page.locator('div[aria-label="Tab: Preview"]')).toBeVisible();
  });
});
