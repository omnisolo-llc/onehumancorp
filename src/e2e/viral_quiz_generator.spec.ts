import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test.describe('Viral Quiz Generator Loop', () => {
  test('should allow owner to create a quiz and user to enter it', async ({ page, context, loginAs, adminUser }) => {
    await loginAs(page, adminUser);

    // 1. Navigate to dashboard using the standard fixture
    await page.goto('/dashboard');
    let content = await page.content();
    if (!content.includes('OneHumanCorp')) {
        await page.goto('/tauri_out/dashboard.html');
        content = await page.content();
    }
    if (!content.includes('OneHumanCorp')) {
        await page.goto('/ui/dashboard.html');
    }

    // Wait to ensure client-side hydration doesn't interrupt filling
    await page.waitForTimeout(500);

    // 2. Click the Growth feature
    await expect(page.locator('#viral-quiz-btn')).toBeVisible();
    await page.locator('#viral-quiz-btn').click();

    // 3. Verify Generator Load
    await expect(page.locator('h1').filter({ hasText: /Viral Quiz Generator/i }).first()).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Quiz Settings' })).toBeVisible();

    // 4. Update the settings
    const titleInput = page.getByPlaceholder('e.g. What type of founder are you?');
    await titleInput.fill('Are you a 10x Developer?');

    // Ensure preview updates
    await expect(page.getByRole('heading', { name: 'Are you a 10x Developer?' })).toBeVisible();

    // 5. Navigate to the generated link to simulate a user taking the quiz
    const generatedLinkLocator = page.getByTestId('generated-link');
    const generatedLinkUrl = await generatedLinkLocator.innerText();

    const publicPage = await context.newPage();
    await publicPage.goto(generatedLinkUrl.trim());

    // 6. Test the Public Quiz Loop
    await expect(publicPage.getByRole('heading', { name: 'Are you a 10x Developer?' })).toBeVisible({ timeout: 15000 });
    await publicPage.getByRole('button', { name: 'Start Quiz' }).click();

    // See question
    await expect(publicPage.getByText('Question 1 of 3')).toBeVisible();

    // Click an option
    await publicPage.locator("button:has-text('Take immediate charge')").click();

    // Result
    await expect(publicPage.getByRole('heading', { name: 'Your Results Are Ready!' })).toBeVisible({ timeout: 15000 });

    // See share buttons
    await expect(publicPage.getByRole('button', { name: 'Share on X to Unlock' })).toBeVisible();
    await expect(publicPage.getByRole('button', { name: 'Share on LinkedIn' })).toBeVisible();

    await publicPage.close();
  });
});
