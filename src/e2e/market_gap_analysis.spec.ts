import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('market_gap_analysis', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'market_gap_analysis');
});
