import { test, expect } from '../../../../e2e/onboarding_fixtures';
import { completeReactZeroClickReview } from '../../../../e2e/react_zero_click_review';

test.describe('Zero Click Builder Mobile Onboarding', () => {
  test.use({ viewport: { width: 375, height: 812 } });
  test('User can generate a store with a single prompt', async ({ page, onboardingOwner }, testInfo) => {
    // Preserve the existing entry URL while using the shared owned setup flow.
    await page.goto('/zero-click-builder');
    await expect(page.getByRole('heading',{name:'Tell us about your business',level:1})).toBeVisible();
    const prompt=page.getByPlaceholder('e.g. I am a home baker in Austin selling custom vegan cakes.');await expect(prompt).toBeVisible();await prompt.fill('I am a home baker in Austin selling custom vegan cakes and cupcakes.');
    const send=page.getByRole('button',{name:'Send message',exact:true});await expect(send).toBeEnabled();
    const pending=page.waitForResponse(response=>new URL(response.url()).pathname==='/api/v1/onboarding/chat'&&response.request().method()==='POST').then(async response=>({status:response.status(),body:await response.json()}));
    const[result]=await Promise.all([pending,send.click()]);const mode=await completeReactZeroClickReview(page,onboardingOwner,result);testInfo.annotations.push({type:'setup-mode',description:mode});
    // The saved setup survives reopening this alias. It does not claim a public
    // storefront deployment or substitute a placeholder iframe for one.
    await page.goto('/zero-click-builder');await expect(page.getByText('Setup complete',{exact:true})).toBeVisible();await expect(page.getByText('Your business is live!')).toHaveCount(0);
    const preview=page.locator('iframe[title="Storefront Preview"]');await expect(preview).toBeVisible();const src=await preview.getAttribute('src');expect(src).toBeTruthy();const target=new URL(src!,page.url());expect(target.origin).toBe(new URL(page.url()).origin);expect(target.pathname).toBe('/builder');expect(target.searchParams.get('tenant')).toBe(onboardingOwner.tenantId);expect(target.searchParams.get('preview')).toBe('true');
    const dashboard=page.getByRole('button',{name:/Go to dashboard/i});await expect(dashboard).toBeVisible();await dashboard.click();await expect(page).toHaveURL(/\/dashboard$/);
  });
});
