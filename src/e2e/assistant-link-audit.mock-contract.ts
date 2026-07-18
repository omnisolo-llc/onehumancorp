import { test, expect } from './fixtures';
import * as path from 'path';
import * as fs from 'fs';

test.describe('Assistant Link Audit on Dashboard', () => {

  test('1. Dashboard renders the WorkBuddy Assistant entry link', async ({ page }) => {
    await page.goto('/dashboard');
    const assistantLink = page.getByRole('link', { name: /Open WorkBuddy Assistant/i });
    await expect(assistantLink).toBeVisible();
    await expect(assistantLink).toHaveAttribute('href', /assistant\.html/);
  });

  test('2. Dashboard Assistant entry link has correct btn-primary styling', async ({ page }) => {
    await page.goto('/dashboard');
    const assistantLink = page.getByRole('link', { name: /Open WorkBuddy Assistant/i });
    await expect(assistantLink).toHaveClass(/btn-primary/);
  });

  test('3. Clicking the Assistant entry link navigates to the Assistant Workspace', async ({ page }) => {
    // Intercept to return the local assistant.html since it's a tauri page
    const workspaceRoot = process.env.TEST_WORKSPACE
        ? path.join(process.env.TEST_SRCDIR!, process.env.TEST_WORKSPACE)
        : process.cwd();
    const tauriUiDir = path.join(workspaceRoot, 'src/ui/tauri/src/ui');

    await page.route('/assistant.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'assistant.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });

    await page.goto('/dashboard');
    const assistantLink = page.getByRole('link', { name: /Open WorkBuddy Assistant/i });
    await assistantLink.click();

    // Verify Title
    await expect(page).toHaveTitle('OHC Assistant Workstation');
    await expect(page.locator('h2[aria-label="Task Title"]')).toBeVisible();
  });

  test('4. Assistant Link is prominent below the dashboard subtitle', async ({ page }) => {
    await page.goto('/dashboard');

    const subtitle = page.locator('.subtitle', { hasText: 'Your private agentic workspace.' });
    await expect(subtitle).toBeVisible();

    const assistantLinkContainer = page.locator('.assistant-entry-container');
    await expect(assistantLinkContainer).toBeVisible();

    // Quick structural check: The container should be a direct sibling/following the subtitle (visually or structurally)
    // We can just assert they both exist and are visible in the viewport flow.
    const subtitleBox = await subtitle.boundingBox();
    const linkBox = await assistantLinkContainer.boundingBox();

    expect(subtitleBox).toBeDefined();
    expect(linkBox).toBeDefined();
    expect(linkBox!.y).toBeGreaterThan(subtitleBox!.y);
  });

  test('5. Verify zero mock data in the Assistant link context', async ({ page }) => {
     await page.goto('/dashboard');
     const assistantLink = page.getByRole('link', { name: /Open WorkBuddy Assistant/i });
     await expect(assistantLink).toBeVisible();

     // Ensure no hardcoded mock badges or strings exist inside the button
     const text = await assistantLink.innerText();
     expect(text.trim()).toBe('Open WorkBuddy Assistant');
  });

});
