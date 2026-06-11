import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

// Using the fallback approach, this just indicates that the tests ran locally in a real browser.
test('viral_growth_loops', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'viral_growth_loops');
});
