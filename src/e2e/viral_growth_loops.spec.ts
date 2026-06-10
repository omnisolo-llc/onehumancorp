import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('viral growth loops e2e smoke', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'viral_growth_loops');
});
