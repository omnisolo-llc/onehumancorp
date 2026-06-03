import { test, expect } from '@playwright/test';
import fs from 'fs/promises';
import path from 'path';

test.describe('Ralph Loop Critical User Journey', () => {
  test('should allow user to initiate a long-running Ralph mission and observe progress', async ({ page }) => {
    // Setup E2E fixture
    const taskId = `e2e-test-${Date.now()}`;
    const fixturePath = path.join(process.cwd(), 'src/e2e/fixtures/ralph-progress.json');
    const targetPath = path.join(process.cwd(), `.ralph_progress_${taskId}.json`);

    await fs.copyFile(fixturePath, targetPath);

    // Navigate to Ralph Mission Control
    await page.goto(`/ralph-loop`);

    // Verify page content
    await expect(page.locator('h1')).toContainText('Ralph Mission Control');

    // Enter mission objective
    const missionObjective = 'Build a full inventory management system with real-time stock alerts.';
    await page.fill('textarea', missionObjective);

    // Start mission
    await page.route('**/api/ralph-loop', async route => {
      if (route.request().method() === 'POST') {
        await route.fulfill({ json: { taskId: taskId } });
      } else {
        await route.continue();
      }
    });

    await page.click('button:has-text("Initiate Ralph Loop")');

    // Verify transition to active mission state
    await expect(page.locator('h3:has-text("Mission Roadmap")')).toBeVisible();
    await expect(page.locator('h3:has-text("Mission Terminal Logs")')).toBeVisible();

    // Verify initial roadmap features exist (from real fixture)
    await expect(page.locator('p:has-text("Step 1: Database Schema Design")')).toBeVisible();
    await expect(page.locator('p:has-text("Step 2: API Endpoint Implementation")')).toBeVisible();

    // Cleanup E2E fixture
    await fs.unlink(targetPath);

    // Verify terminal logs are populated
    await expect(page.locator('div.font-mono p').first()).toContainText('Initialized task and broken down into features.');

    // Verify architectural decisions
    await expect(page.locator('h4:has-text("Key Decisions")')).toBeVisible();
    await expect(page.locator('li:has-text("Decision: Use PostgreSQL row-level security")')).toBeVisible();

    // Verify bug tracker
    await expect(page.locator('h4:has-text("Known Issues")')).toBeVisible();
    await expect(page.locator('li:has-text("Bug: Pagination fails")')).toBeVisible();
  });
});
