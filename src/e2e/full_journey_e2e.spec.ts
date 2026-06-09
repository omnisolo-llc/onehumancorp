import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('smoke test full_journey_e2e', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'full_journey_e2e');
});
