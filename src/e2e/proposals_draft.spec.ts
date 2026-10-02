import { test, expect } from '@playwright/test';

test.describe('GPT Researcher Proposal Generator UI', () => {
  test('should generate a multi-section proposal using planner and execution agent', async ({ page }) => {
    // 1. Authenticate & load the home page (which redirects to dashboard or we can just go to /dashboard)
    await page.goto('/dashboard');

    await expect(page.getByRole('heading', { name: 'Welcome back' })).toBeVisible();

    // 2. Click on the "Proposal Draft" widget/link from the Dashboard
    await page.click('text=Proposal Draft');

    // 3. Verify we reached the generator page
    await expect(page.getByRole('main').getByRole('heading', { name: 'AI Proposal Generator', level: 1 })).toBeVisible();

    // 4. Enter a brief topic
    await page.locator('textarea').fill('Website redesign for local bakery');

    // 5. Click generate
    await page.click('text=Generate Proposal');

    // 6. Wait for the result
    await expect(page.locator('text=Generated Draft')).toBeVisible({ timeout: 15000 });

    const proposalText = await page.locator('.whitespace-pre-wrap').textContent();

    // 7. Validate that the GPT Researcher mechanic successfully generated sections
    expect(proposalText).toContain('Research Report: Website redesign for local bakery');
    expect(proposalText).toContain('Executive Summary');
    expect(proposalText).toContain('Project Scope');
    expect(proposalText).toContain('Budget and Timeline');
    expect(proposalText).toContain('Generated detail for the requested section');
  });
});
