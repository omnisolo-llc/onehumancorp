import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('smoke test', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'test_e2e_run');
});
