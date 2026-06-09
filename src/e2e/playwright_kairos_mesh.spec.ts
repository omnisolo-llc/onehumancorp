import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('smoke test playwright_kairos_mesh', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'playwright_kairos_mesh');
});
