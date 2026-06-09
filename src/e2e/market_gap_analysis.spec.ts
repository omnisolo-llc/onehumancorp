import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('smoke test market_gap_analysis', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'market_gap_analysis');
});
