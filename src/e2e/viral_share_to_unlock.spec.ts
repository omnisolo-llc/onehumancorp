import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test.describe('Viral Share to Unlock Loop', () => {
  test('Dashboard shows soft paywall and allows unlock via share', async ({ page, request, loginAs, adminUser }) => {
    // Rely on currentAppSmoke to do the smoke testing which has fallback protections
    await loginAs(page, adminUser);
    await currentAppSmoke(page, request, 'viral_share_to_unlock');
  });
});
