import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('smoke test business_setup_2', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'business_setup_2');
});
