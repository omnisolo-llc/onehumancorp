import { test, expect } from '@playwright/test';
import { e2eTest } from './fixtures';

e2eTest('Viral Ambassador Program Generator and Embed', async ({ adminPage: page }) => {
  // Navigate to dashboard
  await page.goto('/dashboard.html');

  // Click the Ambassador Program link
  await page.click('#viral-ambassador-link');

  // Verify the page title
  await expect(page.locator('h1').filter({ hasText: 'Ambassador Program' })).toBeVisible();

  // Fill out the configuration form
  await page.fill('#program-title', 'My Custom Ambassador Program');
  await page.fill('#program-reward', '20% Commission');

  // Generate the embed code
  await page.click('#generate-btn');

  // Verify the generated code
  await expect(page.locator('#result-area')).toBeVisible();

  const embedCode = await page.locator('#embed-code').innerText();
  expect(embedCode).toContain('<iframe');
  expect(embedCode).toContain('/api/v1/growth/ambassador/embed');

  expect(embedCode).toContain('title=My%20Custom%20Ambassador%20Program');
  expect(embedCode).toContain('reward=20%25%20Commission');

  // Now interact with the actual iframe preview
  const iframeLocator = page.locator('#preview-container iframe');
  const frame = iframeLocator.contentFrame();

  // Make sure the title and reward updated in the iframe
  await expect(frame.locator('.title')).toHaveText('My Custom Ambassador Program');
  await expect(frame.locator('.reward')).toHaveText('20% Commission');

  // Fill the form inside the iframe
  await frame.fill('#amb-name', 'Test Ambassador');
  await frame.fill('#amb-email', 'ambassador@example.com');
  await frame.click('#join-btn');

  // It should show the success message
  await expect(frame.locator('#success-msg')).toBeVisible();
});
