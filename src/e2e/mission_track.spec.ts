import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('smoke test mission_track', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'mission_track');
});
