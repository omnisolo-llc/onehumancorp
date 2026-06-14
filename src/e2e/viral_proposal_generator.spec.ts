import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';


test.describe('Viral Proposal Generator Loop', () => {
  test('should allow creating a proposal and viewing the viral loop', async ({ page, loginAs, adminUser }) => {
    // Navigate to dashboard first to find the link
    await loginAs(page, adminUser);
    await page.goto('/dashboard');

    const proposalLink = page.locator('a[href="/proposal-generator"]');
    await expect(proposalLink).toBeVisible();
    await proposalLink.click();

    // Verify page content
    await expect(page.getByRole('heading', { name: 'Proposal Generator' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Create Professional Proposal' })).toBeVisible();

    // Fill out the form
    await page.fill('input[placeholder="e.g. Acme Corp"]', 'Vandelay Industries');
    await page.fill('textarea[placeholder="e.g. Website Redesign, SEO Optimization, and Content Strategy"]', 'Full stack rewrite in Rust and React');
    await page.fill('input[placeholder="e.g. 2500.00"]', '10000');
    await page.fill('input[placeholder="e.g. 4-6 Weeks"]', '3 Months');

    // Click generate
    const generateBtn = page.getByRole('button', { name: 'Generate Shareable Proposal' });
    await expect(generateBtn).toBeVisible();
    await generateBtn.click();

    // Verify the proposal is ready
    await expect(page.getByRole('heading', { name: 'Your Proposal is Ready!' })).toBeVisible();

    // Click preview proposal
    const previewLink = page.getByRole('link', { name: 'Preview Proposal' });
    await expect(previewLink).toBeVisible();

    // Instead of waiting for a new tab, let's navigate the current page to the href
    const href = await previewLink.getAttribute('href');
    expect(href).toContain('/proposal-generator/view?data=');

    await page.goto(href!);

    // Verify the proposal view
    await expect(page.getByRole('heading', { name: 'PROJECT PROPOSAL' })).toBeVisible();
    await expect(page.getByText('Vandelay Industries')).toBeVisible();
    await expect(page.getByText('Full stack rewrite in Rust and React')).toBeVisible();
    await expect(page.getByText('$10000.00')).toBeVisible();
    await expect(page.getByText('3 Months')).toBeVisible();

    // Verify the viral loop footer
    const poweredByLink = page.getByRole('link', { name: /Powered by OHC/i });
    await expect(poweredByLink).toBeVisible();

    const ctaLink = page.getByRole('link', { name: /Create your own professional proposals/i });
    await expect(ctaLink).toBeVisible();

    const onboardingHref = await ctaLink.getAttribute('href');
    expect(onboardingHref).toContain('/onboarding?ref=');
    expect(onboardingHref).toContain('source=proposal_generator');
  });
});
