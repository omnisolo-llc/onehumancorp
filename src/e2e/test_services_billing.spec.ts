import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('smoke test test_services_billing', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'test_services_billing');
});
