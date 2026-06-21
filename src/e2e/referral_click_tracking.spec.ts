import { test, expect } from './fixtures';

test.describe('Referral Click Tracking via GET', () => {
  test('should successfully redirect and potentially log click', async ({ request }) => {
    // Test the GET API endpoint manually
    const response = await request.get('/api/v1/growth/referrals/click?target=/onboarding&ref=e2e-test-ref', {
        maxRedirects: 0 // Prevent following so we can check the status code
    });

    // Next.js NextResponse.redirect uses 307
    expect([307, 308, 302, 301]).toContain(response.status());
    const redirectUrl = response.headers().location;
    expect(redirectUrl).toContain('/onboarding?ref=e2e-test-ref');
  });

  test('should redirect correctly when accessing through browser', async ({ page }) => {
     await page.goto('/api/v1/growth/referrals/click?target=/onboarding&ref=browser-test-ref');

     // Should end up on onboarding
     expect(page.url()).toContain('/onboarding');
     expect(page.url()).toContain('ref=browser-test-ref');
  });
});
