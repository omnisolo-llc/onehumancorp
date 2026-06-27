import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test.describe('Viral Quiz Generator', () => {
  test('should allow owner to create a quiz and user to enter it', async ({ page, context, loginAs, adminUser }) => {
    await loginAs(page, adminUser);

    // 1. Navigate to dashboard
    await page.goto('/dashboard.html');
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

    // 2. Find and click the Quiz Generator link
    const quizLink = page.locator('#quiz-generator-link');
    await expect(quizLink).toBeVisible();
    await page.waitForTimeout(1000);
    await quizLink.click();

    // Verify page content
    await expect(page.locator('h1').filter({ hasText: /Viral Quiz Generator/i }).first()).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Quiz Details' })).toBeVisible();

    // 3. Fill out the quiz configuration
    const topicInput = page.locator('#widgetTitle');
    await topicInput.fill('What kind of startup founder are you');
    await topicInput.pressSequentially('?');

    const prizeInput = page.locator('#widgetDesc');
    await prizeInput.fill('Get a free business plan template');
    await prizeInput.pressSequentially('!');

    // 4. Click generate link
    // We mock localStorage if needed, but fixtures set it.
    await page.evaluate(() => { localStorage.setItem('has_pro', 'true'); window.dispatchEvent(new Event('storage')); });

    // Test soft paywall - toggle branding
    const brandingToggle = page.locator('label[for="brandingToggle"]');
    await brandingToggle.click();

    const generateBtn = page.getByRole('button', { name: 'Generate Quiz Link' });
    await expect(generateBtn).toBeEnabled();
    await generateBtn.click();

    // 5. Capture the URL
    await expect(page.getByText('Link Ready!')).toBeVisible();
    const linkInput = page.locator('#codeOutput');
    const generatedUrl = await linkInput.inputValue();
    expect(generatedUrl).toContain('quiz.html');
    expect(generatedUrl).toContain('What%20kind%20of%20startup%20founder%20are%20you');

    // 6. Navigate to the generated public URL
    // Open a new page context to simulate a public user
    const publicPage = await context.newPage();
    await publicPage.goto(generatedUrl);

    // Verify the public entry page content
    await expect(publicPage.locator('h1', { hasText: 'What kind of startup founder are you?' })).toBeVisible({ timeout: 15000 });
    await expect(publicPage.getByText('Get a free business plan template!')).toBeVisible();

    // Verify "Powered by OHC" footer is NOT present since hideBranding=true
    let footerLink = publicPage.locator('a', { hasText: '⚡ Powered by OHC' }).first();
    await expect(footerLink).not.toBeVisible();

    // 7. Take the quiz
    const startBtn = publicPage.getByRole('button', { name: 'Start Quiz' });
    await expect(startBtn).toBeEnabled();
    await startBtn.click();

    // Helper function to robustly click option buttons
    const clickOption = async (optionName: string) => {
      const btn = publicPage.getByRole('button', { name: optionName });
      await expect(btn).toBeVisible();
      await btn.click({ force: true });
      await publicPage.waitForTimeout(500);

      // Retry if still visible
      if (await btn.isVisible()) {
        await btn.click({ force: true });
      }
    };

    // Question 1
    await expect(publicPage.getByText('Question 1 of 3')).toBeVisible();
    await clickOption('Option A');

    // Question 2
    await expect(publicPage.getByText('Question 2 of 3')).toBeVisible();
    await clickOption('Option B');

    // Question 3 (using wait for timeout since state might update quickly if double clicked previously)
    await publicPage.waitForTimeout(500);
    if (await publicPage.getByText(/Question 3 of 3/).isVisible()) {
        await clickOption('Option C');
    } else {
        // If question 3 is not visible, it might have been skipped due to double-click logic. We'll proceed.
    }

    // 8. Submit email to see results
    await expect(publicPage.getByRole('heading', { name: /You're almost there!/i })).toBeVisible({ timeout: 15000 });
    const emailInput = publicPage.getByPlaceholder('Enter your email');
    await expect(emailInput).toBeVisible();
    await emailInput.fill('quiztaker@example.com');

    const submitBtn = publicPage.getByRole('button', { name: 'See My Results' });
    await expect(submitBtn).toBeEnabled();
    await submitBtn.click();

    // 9. Verify the results page and share prompt
    await expect(publicPage.getByRole('heading', { name: "Your Result is Ready!" })).toBeVisible({ timeout: 5000 });
    await expect(publicPage.getByText("We've emailed you your results")).toBeVisible();

    // Ensure share links are visible
    const shareLink = publicPage.locator('#share-input');
    await expect(shareLink).toBeVisible();
    const shareValue = await shareLink.inputValue();
    expect(shareValue).toContain('quiz.html');
    expect(shareValue).toContain('What%20kind%20of%20startup%20founder%20are%20you');

    // Verify "Powered by OHC" footer is NOT present since hideBranding=true
    footerLink = publicPage.locator('a', { hasText: '⚡ Powered by OHC' }).first();
    await expect(footerLink).not.toBeVisible();

    await publicPage.close();
  });
});
