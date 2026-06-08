import { test, expect } from '@playwright/test';

/**
 * Fatima's Persona: Food Cart Operator
 * Goal: 100% of business operations on a 375px mobile screen.
 * CUJ: Capture demand (Agent Proposes Action) -> Review (Expand Card) -> Execute (Approve).
 */

test.use({ viewport: { width: 375, height: 667 } });

test('Fatima approves agent-proposed actions in the Unified Feed', async ({ page }) => {
  // 1. Login/Dashboard
  await page.goto('/dashboard');

  // Wait for feed to load
  await expect(page.getByLabel('Unified Agent Feed')).toBeVisible();

  // 2. Trigger Simulation (using our new simulation buttons in FAB if available, or just mock data if E2E usually uses real backend)
  // Assuming the feed might be empty in a fresh test environment, we use the FAB to trigger simulation
  await page.getByRole('button', { name: /open/i }).first().click(); // Open FAB

  // Trigger Operations Simulation
  await page.getByRole('button', { name: /Simulate Operations/i }).click();
  await expect(page.getByText(/3 new orders to fulfill/i)).toBeVisible();

  // Trigger Advisory Simulation
  await page.getByRole('button', { name: /Simulate Advisory/i }).click();
  await expect(page.getByText(/30 days since your last promo/i)).toBeVisible();

  // 3. Interact with Operations Card (P0 Requirement: Massive Approval Button)
  const opsCard = page.locator('div.glassmorphism').filter({ hasText: '3 new orders to fulfill' });
  const fulfillButton = opsCard.getByTestId('approve-fulfillment');
  await expect(fulfillButton).toBeVisible();

  // Verify touch target (min-h-56 is approx 56px)
  const box = await fulfillButton.boundingBox();
  expect(box?.height).toBeGreaterThanOrEqual(56);

  await fulfillButton.click();
  // Card should disappear (optimistic UI or refresh)
  await expect(opsCard).not.toBeVisible();

  // 4. Interact with Advisory Card (CUJ Step 4: Expand to show AI-drafted email)
  const advisoryCard = page.locator('div.glassmorphism').filter({ hasText: '30 days since your last promo' });
  const draftButton = advisoryCard.getByTestId('approve-draft');

  await draftButton.click();

  // Card should expand to show draft
  await expect(page.getByText(/Flash Sale: 20% off/i)).toBeVisible();

  // Massive "Approve & Send" button should appear
  const sendButton = advisoryCard.getByTestId('approve-send-promo');
  await expect(sendButton).toBeVisible();
  await sendButton.click();

  await expect(advisoryCard).not.toBeVisible();

  // 5. Verification in Activity Feed
  await page.getByRole('button', { name: /Activity Feed/i }).click();
  await expect(page.getByText(/Operations Agent/i).first()).toBeVisible();
  await expect(page.getByText(/APPROVED/i).first()).toBeVisible();
});
