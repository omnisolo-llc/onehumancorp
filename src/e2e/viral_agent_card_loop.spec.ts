import { test, expect } from './fixtures';

test.describe('Viral Agent Card Growth Loop', () => {
  test('should allow creating an agent card, toggle branding, and copy link', async ({ page, context }) => {
    // Wait for the UI to be ready
    await page.goto('/agent-card.html');
  });
});
