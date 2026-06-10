import { currentAppSmoke } from './current_app_smoke';

import { test } from '@playwright/test';
test('smoke', async ({ page, request }) => {
  await test('smoke', async ({ page, request }) => {
  await currentAppSmoke(page, request, page, request, 'nova_mission_track');
});
});
