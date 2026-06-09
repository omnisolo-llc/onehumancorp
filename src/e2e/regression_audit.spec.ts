import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('smoke test regression_audit', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'regression_audit');
});
