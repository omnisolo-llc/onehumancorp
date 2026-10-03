import { test, expect } from './fixtures';
import { generationAcceptance, generationGateReason } from './generation-acceptance';

test.describe('GPT Researcher Proposal Generator UI', () => {
  test('should generate a multi-section proposal using the authorized model @provider-acceptance', async ({ page }) => {
    test.skip(!generationAcceptance, generationGateReason);
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
    const drafting = page.waitForResponse(response => new URL(response.url()).pathname === '/api/v1/proposals/draft' && response.request().method() === 'POST');
    await page.click('text=Generate Proposal');
    const response = await drafting;
    expect(response.status()).toBe(200);
    const returned = await response.json();

    // 6. Wait for the result
    await expect(page.locator('text=Generated Draft')).toBeVisible({ timeout: 15000 });

    const proposalText = await page.locator('.whitespace-pre-wrap').textContent();

    // 7. Validate that the GPT Researcher mechanic successfully generated sections
    expect(proposalText).toBe(returned.proposal);
    expect(proposalText).toContain('Executive Summary');
    expect(proposalText).toContain('Scope');
    expect(proposalText).toContain('Milestones');
    expect(proposalText).toContain('Investment');
    expect(proposalText).toContain('Next Steps');
    expect(proposalText).not.toContain('Generated detail for the requested section');
  });
  test('an unavailable local proposal model retains the brief without inventing sections', async ({ page }) => {
    test.skip(generationAcceptance, 'This contract requires an unconfigured local proposal model.');
    await page.goto('/dashboard');
    await page.getByText('Proposal Draft', { exact: true }).click();
    await expect(page.getByRole('main').getByRole('heading', { name: 'AI Proposal Generator', level: 1 })).toBeVisible();
    const brief = 'Website redesign for local bakery';
    await page.getByLabel('Project Brief / Topic').fill(brief);
    const drafting = page.waitForResponse(response => new URL(response.url()).pathname === '/api/v1/proposals/draft' && response.request().method() === 'POST');
    await page.getByRole('button', { name: 'Generate Proposal' }).click();
    const response = await drafting;
    expect(response.status()).toBe(502);
    expect(await response.text()).toBe('');
    await expect(page.getByRole('alert').filter({ hasText: 'Failed to draft proposal' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Generated Draft' })).toHaveCount(0);
    await expect(page.getByLabel('Project Brief / Topic')).toHaveValue(brief);
    await expect(page.getByRole('button', { name: 'Generate Proposal' })).toBeEnabled();
  });

});
