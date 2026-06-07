import { test, expect } from '@playwright/test';

test('Advisor agent weekly health report generates an approval card in the feed', async ({ page }) => {
  // 1. Given the e2e-tenant has a seeded pending weekly report from the AdvisorWorker

  // 2. The user navigates to the mobile dashboard (using the typical 375px viewport)
  await page.setViewportSize({ width: 375, height: 812 });

  // Log in as the test user
  await page.goto('/login');
  await page.fill('input[type="email"]', 'admin@e2e.test');
  await page.fill('input[type="password"]', 'password123');
  await page.click('button:has-text("Sign in")');

  // Ensure dashboard loads
  await page.waitForURL('/dashboard');

  // Navigate to the Proposals/Activity Feed if it's a tab
  // In UnifiedAgentFeed, Proposals is active by default
  const proposalsTab = page.locator('button', { hasText: 'Proposals' });
  if (await proposalsTab.isVisible()) {
    await proposalsTab.click();
  }

  // 3. We should see the 'Weekly Business Health Report' card from the 'BUSINESS ADVISORY' department
  const advisorCard = page.locator('div.glassmorphism').filter({ hasText: 'business advisory' });
  await expect(advisorCard).toBeVisible();

  // 4. Verify the contents of the report match the prompt generation requirement
  await expect(advisorCard).toContainText('Weekly Business Health Report');

  // Check that the summary and actionable suggestion are rendered explicitly by our UI fix
  await expect(advisorCard).toContainText('Great job this week!');
  await expect(advisorCard).toContainText('Want me to draft a new promo email for next week?');

  // 5. User approves the actionable suggestion
  const approveButton = advisorCard.locator('button:has-text("Approve")');
  await approveButton.click();

  // Card should either disappear or state it is approved depending on the UI logic
  // (In UnifiedAgentFeed.tsx, handleDecision usually filters the card out on success)
  await expect(advisorCard).not.toBeVisible({ timeout: 5000 });
});
