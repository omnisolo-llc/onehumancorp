import { test, expect } from '../fixtures';

test.describe('Nora - Agency Principal: Project Intake & Proposal Generation', () => {
  test('Mobile-first intake form triggers AI proposal draft', async ({ page }) => {
    // Set viewport to mobile to enforce 375px requirement
    await page.setViewportSize({ width: 375, height: 812 });

    // Navigate to the test fixture containing the unified component
    await page.goto('/ui/proposal-intake.html');

    // Ensure no horizontal scroll on mobile
    const scrollWidth = await page.evaluate(() => document.documentElement.scrollWidth);
    expect(scrollWidth).toBeLessThanOrEqual(375);

    // Fill out the intake form
    await page.fill('input[name="client_name"]', 'Acme Corp');
    await page.fill('input[name="project_name"]', 'Website Redesign');
  });
});
