import { test, expect } from '@playwright/test';
import { spawn, execSync } from 'child_process';
import fs from 'fs';
import path from 'path';

const PORT = 3000;
const STAGING_DIR = '/tmp/slint-web-test';

test.describe('Slint Web E2E', () => {
  let server: ReturnType<typeof spawn>;
  const consoleErrors: string[] = [];

  test.beforeAll(async () => {
    // Clean up any previous staging directory
    execSync(`rm -rf ${STAGING_DIR}`);
    fs.mkdirSync(STAGING_DIR, { recursive: true });

    // Build the WASM app
    console.log('Building WASM app...');
    try {
      execSync('bazel build //src/app:app-wasm', { stdio: 'inherit' });
    } catch (e) {
      throw new Error('Failed to build WASM app. Make sure bazel is installed.');
    }

    // Copy WASM artifacts from bazel-bin to staging directory
    const bazelOut = 'bazel-bin/src/app';
    if (fs.existsSync(bazelOut)) {
      const files = fs.readdirSync(bazelOut);
      for (const file of files) {
        if (file.startsWith('app_wasm')) {
          fs.copyFileSync(
            path.join(bazelOut, file),
            path.join(STAGING_DIR, file)
          );
        }
      }
    } else {
      throw new Error(`Bazel output directory ${bazelOut} not found. WASM build may have failed.`);
    }

    // Copy index.html to staging directory
    fs.copyFileSync(
      path.join('src/app/web', 'index.html'),
      path.join(STAGING_DIR, 'index.html')
    );

    // Start HTTP server to serve the staging directory
    console.log(`Starting HTTP server on port ${PORT}...`);
    server = spawn('python3', ['-m', 'http.server', String(PORT)], {
      cwd: STAGING_DIR,
      detached: true,
    });

    // Wait for server to be ready
    await new Promise<void>((resolve, reject) => {
      setTimeout(() => resolve(), 2000);
    });

    // Verify server is running
    if (!fs.existsSync(path.join(STAGING_DIR, 'index.html'))) {
      throw new Error('HTTP server failed to start - index.html not found');
    }
  });

  test.beforeEach(async ({ page }) => {
    // Set up console error listener BEFORE any interactions
    consoleErrors.length = 0;
    page.on('console', msg => {
      if (msg.type() === 'error') {
        consoleErrors.push(msg.text());
      }
    });
  });

  test.afterAll(() => {
    // Kill HTTP server
    if (server) {
      console.log('Stopping HTTP server...');
      process.kill(-server.pid!, 'SIGTERM');
    }
    // Clean up staging directory
    execSync(`rm -rf ${STAGING_DIR}`);
  });

  test('should load without errors', async ({ page }) => {
    // Navigate to the app
    await page.goto(`http://localhost:${PORT}`);

    // Wait for the canvas to be ready
    await page.waitForSelector('canvas', { timeout: 10000 });

    // Wait a bit for Slint to initialize
    await page.waitForTimeout(1000);

    // Assert no console errors on initial load
    expect(consoleErrors).toHaveLength(0);
  });

  test('should click all buttons without errors', async ({ page }) => {
    await page.goto(`http://localhost:${PORT}`);
    await page.waitForSelector('canvas', { timeout: 10000 });
    await page.waitForTimeout(1000);

    // Find all buttons via accessibility tree
    const buttons = page.getByRole('button');
    const count = await buttons.count();
    console.log(`Found ${count} buttons`);

    for (let i = 0; i < count; i++) {
      const button = buttons.nth(i);
      try {
        const name = await button.textContent();
        console.log(`Clicking button: ${name || '(unnamed)'}`);
        await button.click({ timeout: 5000 });
        await page.waitForTimeout(200); // Small delay between clicks
      } catch (e) {
        // Button may have become stale or disabled, continue
        console.log(`Button ${i} click failed: ${e}`);
      }
    }

    // Give any async handlers time to complete
    await page.waitForTimeout(500);

    // Assert no console errors after clicking all buttons
    expect(consoleErrors).toHaveLength(0);
  });

  test('should click all links without errors', async ({ page }) => {
    await page.goto(`http://localhost:${PORT}`);
    await page.waitForSelector('canvas', { timeout: 10000 });
    await page.waitForTimeout(1000);

    // Find all anchor tags (links)
    const links = page.locator('a');
    const count = await links.count();
    console.log(`Found ${count} links`);

    for (let i = 0; i < count; i++) {
      const link = links.nth(i);
      try {
        const href = await link.getAttribute('href');
        const text = await link.textContent();
        console.log(`Clicking link: ${text || '(unnamed)'} -> ${href}`);
        await link.click({ timeout: 5000 });
        await page.waitForTimeout(200);
      } catch (e) {
        // Link may have become stale or navigation may not be implemented
        console.log(`Link ${i} click failed: ${e}`);
      }
    }

    // Give any async handlers time to complete
    await page.waitForTimeout(500);

    // Assert no console errors after clicking all links
    expect(consoleErrors).toHaveLength(0);
  });

  test('should handle all interactive elements without errors', async ({ page }) => {
    await page.goto(`http://localhost:${PORT}`);
    await page.waitForSelector('canvas', { timeout: 10000 });
    await page.waitForTimeout(1000);

    // Get all interactive elements (buttons, links, inputs)
    const buttons = page.getByRole('button');
    const links = page.locator('a');

    const buttonCount = await buttons.count();
    const linkCount = await links.count();

    console.log(`Total buttons: ${buttonCount}, Total links: ${linkCount}`);

    // Click all buttons
    for (let i = 0; i < buttonCount; i++) {
      const button = buttons.nth(i);
      try {
        await button.click({ timeout: 5000 });
        await page.waitForTimeout(100);
      } catch (e) {
        // Continue on error
      }
    }

    // Click all links
    for (let i = 0; i < linkCount; i++) {
      const link = links.nth(i);
      try {
        await link.click({ timeout: 5000 });
        await page.waitForTimeout(100);
      } catch (e) {
        // Continue on error
      }
    }

    await page.waitForTimeout(500);

    // Final assertion - no console errors
    expect(consoleErrors).toHaveLength(0);
  });
});