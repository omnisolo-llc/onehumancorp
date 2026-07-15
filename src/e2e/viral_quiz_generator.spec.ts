import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Viral Quiz Generator Loop', () => {
    test('End to end quiz creation and engagement', async ({ page, context }) => {
        // 1. Navigate to dashboard using the standard fixture
        await adminPage(page);

        // 2. Click the Growth feature
        await expect(page.locator('#viral-quiz-btn')).toBeVisible();
        await page.locator('#viral-quiz-btn').click();

        // 3. Verify Generator Load
        await expect(page.getByText('Viral Quiz Generator 🧠')).toBeVisible();

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
        await expect(publicPage.getByRole('heading', { name: 'Are you a 10x Developer?' })).toBeVisible();
        await publicPage.getByRole('button', { name: 'Start Quiz' }).click();

        // See question
        await expect(publicPage.getByText('Question 1 of 3')).toBeVisible();

        // Click an option
        await publicPage.locator("button:has-text('Take immediate charge')").click();

        // Result
        await expect(publicPage.getByRole('heading', { name: 'Your Results Are Ready!' })).toBeVisible();

        // See share buttons
        await expect(publicPage.getByRole('button', { name: 'Share on X to Unlock' })).toBeVisible();
        await expect(publicPage.getByRole('button', { name: 'Share on LinkedIn' })).toBeVisible();
    });
});
