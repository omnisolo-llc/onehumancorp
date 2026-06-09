import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('smoke test lens_audit', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'lens_audit');
});
