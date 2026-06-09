import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('smoke test', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'virtual_meeting_walkthrough');
});
