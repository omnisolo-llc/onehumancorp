import { test, expect } from '../onboarding_fixtures';
import {captureSetupPost,captureSetupLaunch,completeManualSetup,expectLaunchedSetup,expectPreparedSetup,verifiedSetupOwner} from '../support/legacy_manual_setup';

test.describe('Zero-Click Onboarding Flow', () => {
  test.use({ viewport: { width: 375, height: 667 } }); // strictly mobile viewport

  test('should complete the zero-click onboarding flow on mobile', async ({ page }) => {
    // Navigate to the real local server
    await page.goto('/setup.html');
    await expect(page).toHaveTitle(/OmniSolo|OmniSolo/);

    await page.getByRole('button', { name: 'Generate My Workspace', exact: true }).click();

    // Initial Screen
    await expect(page.locator('h1', { hasText: 'Tell us about your business' })).toBeVisible({ timeout: 15000 });

    // Check if the input loaded
    await expect(page.locator('#instant-bio')).toBeVisible();

    // Type into the input
    await page.locator('#instant-bio').fill('I am a baker in Austin selling custom cakes');

    const owner=await verifiedSetupOwner(page);
    const generated=captureSetupPost(page,'start_zero_click');
    const [reply]=await Promise.all([generated,page.locator('#generate-storefront-btn').click()]);
    if(reply.status===503){
      expect(reply.body).toMatchObject({error:'onboarding_ai_unconfigured'});
      expect(reply.body.success).not.toBe(true);
      await completeManualSetup(page,'instant','I am a baker in Austin selling custom cakes',owner);
    }else{
      // A configured provider must supply its real prepared receipt; it is never mocked here.
      const id=expectPreparedSetup(reply,owner);
      const approval=page.locator('#step-approval');await expect(approval).toBeVisible();
      const launch=await captureSetupLaunch(page);
      try {
        const [launched]=await Promise.all([launch.response,approval.getByRole('button',{name:'Approve & Complete Setup'}).click()]);
        await expectLaunchedSetup(page,launched,id,owner);
      } finally { await launch.dispose(); }
    }

    // The acknowledged local setup leads to the dashboard.
    await expect(page).toHaveURL(/.*(dashboard\.html|dashboard|success\.html).*/, { timeout: 30000 });
  });
});
