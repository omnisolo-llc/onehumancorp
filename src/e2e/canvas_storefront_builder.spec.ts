import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('smoke test canvas_storefront_builder', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'canvas_storefront_builder');
});
