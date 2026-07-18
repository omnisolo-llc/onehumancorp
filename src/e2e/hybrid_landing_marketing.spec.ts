import { test, expect } from '@playwright/test';

test.describe('Hybrid Landing Page Growth Test', () => {
  test('should display the core local-first value propositions and CTA', async ({ page }) => {
    // Navigate to the hybrid landing page
    await page.goto('/api/v1/ui/hybrid-landing.html');

    // Verify main heading
    const mainHeading = page.locator('h1', { hasText: 'The Hybrid Agentic OS' });
    await expect(mainHeading).toBeVisible();

    // Verify subtitle mentions local sovereignty
    const subtitle = page.locator('.subtitle');
    await expect(subtitle).toContainText('local sovereignty');

    // Verify "Zero Data Leakage" pillar
    const zeroDataLeakage = page.locator('h3', { hasText: 'Zero Data Leakage' });
    await expect(zeroDataLeakage).toBeVisible();

    // Verify "Air-Gapped Autonomy" pillar
    const airGapped = page.locator('h3', { hasText: 'Air-Gapped Autonomy' });
    await expect(airGapped).toBeVisible();

    // Verify "Seamless Cloud Bridge" pillar
    const cloudBridge = page.locator('h3', { hasText: 'Seamless Cloud Bridge' });
    await expect(cloudBridge).toBeVisible();

    // Verify the primary Call to Action
    const ctaButton = page.locator('a.cta-btn', { hasText: 'Download Standalone Desktop' });
    await expect(ctaButton).toBeVisible();
    await expect(ctaButton).toHaveAttribute('href', '/setup.html');
  });
});
