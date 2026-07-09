import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('viral_community_goal_generator_smoke', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'viral_community_goal_generator_smoke');
});

test.describe('Viral Community Goal Generator', () => {
    test('generator page renders correctly and updates preview on input', async ({ page, adminUser, loginAs }) => {
        await loginAs(page, adminUser);

        // Wait for dashboard to load then click link
        await page.goto('/dashboard.html');
        const link = page.locator('a[id="viral-community-goal-link"]');
        await expect(link).toBeVisible();
        await link.click();

        // Check the page header
        await expect(page.locator('h1', { hasText: 'Community Goal Generator' })).toBeVisible();

        // Configure the widget
        const targetInput = page.locator('input[id="goal-target"]');
        await targetInput.fill('1000');

        const rewardInput = page.locator('input[id="goal-reward"]');
        await rewardInput.fill('Free cupcakes for everyone!');

        // Check the iframe preview updates.
        // First wait for the iframe to be present
        const iframe = page.locator('iframe').first();
        await expect(iframe).toBeVisible();

        // Wait for iframe src to update
        await expect(iframe).toHaveAttribute('src', /1000/);
        await expect(iframe).toHaveAttribute('src', /Free%20cupcakes/);

        // Generate embed code
        const generateBtn = page.locator('button[id="generate-btn"]');
        await generateBtn.click();

        // Check the generated embed code
        const resultArea = page.locator('#result-area');
        await expect(resultArea).toBeVisible();

        const embedCode = await page.locator('#embed-code').innerText();
        expect(embedCode).toContain('1000');
        expect(embedCode).toContain('Free%20cupcakes');
        expect(embedCode).toContain('e2e-tenant');
    });
});
