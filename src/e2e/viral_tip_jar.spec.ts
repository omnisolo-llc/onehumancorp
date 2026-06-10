import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test.describe('Viral Tip Jar Growth Loop', () => {
  test('should allow creating a tip jar and viewing the viral loop', async ({ page, request }) => {
    await currentAppSmoke(page, request, 'viral_tip_jar');
    // Navigate to dashboard first to find the link
    await page.goto('/dashboard');

    const tipJarLink = page.locator('a[href="/tip-jar"]');
    await expect(tipJarLink).toBeVisible();
    await tipJarLink.click();

    // Verify generator page content
    await expect(page.getByRole('heading', { name: 'Create Your Tip Jar' })).toBeVisible();

    // Fill out the form
    await page.fill('textarea[placeholder="e.g. Buy me a coffee!"]', 'Thanks for supporting my art!');

    // Check branding toggle exists
    const removeBrandingToggle = page.getByLabel(/Remove "Powered by OHC" Badge/i);
    await expect(removeBrandingToggle).toBeVisible();

    // Generate link
    const generateBtn = page.getByRole('button', { name: 'Generate Tip Jar Link' });
    await generateBtn.click();

    // Verify it's ready
    await expect(page.getByRole('heading', { name: 'Your Tip Jar is Ready!' })).toBeVisible();

    // Click preview
    const previewLink = page.getByRole('link', { name: 'Preview Tip Jar' });
    await expect(previewLink).toBeVisible();

    const href = await previewLink.getAttribute('href');
    expect(href).toContain('/tip-jar/view?data=');

    // Navigate to the view page
    await page.goto(href!);

    // Verify the tip jar view
    await expect(page.getByRole('heading', { name: /Support/ })).toBeVisible();
    await expect(page.getByText('Thanks for supporting my art!')).toBeVisible();

    // Check preset buttons
    await expect(page.getByRole('button', { name: '$5' })).toBeVisible();
    await expect(page.getByRole('button', { name: '$10' })).toBeVisible();
    await expect(page.getByRole('button', { name: '$20' })).toBeVisible();

    // Check custom amount input
    const customInput = page.getByPlaceholder('Custom amount');
    await expect(customInput).toBeVisible();

    // Select a preset
    await page.getByRole('button', { name: '$10' }).click();
    await expect(page.getByRole('button', { name: 'Pay $10.00' })).toBeVisible();

    // Verify the viral loop footer
    const poweredByLink = page.getByRole('link', { name: /Powered by OHC/i });
    await expect(poweredByLink).toBeVisible();

    const createOwnLink = page.getByRole('link', { name: /Create your own tip jar for free/i });
    await expect(createOwnLink).toBeVisible();

    const onboardingHref = await createOwnLink.getAttribute('href');
    expect(onboardingHref).toContain('/onboarding?ref=');
    expect(onboardingHref).toContain('source=tip_jar');
  });
});
