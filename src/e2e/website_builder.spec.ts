import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('smoke test website_builder', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'website_builder');
});
