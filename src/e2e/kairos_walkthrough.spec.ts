import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('smoke test kairos_walkthrough', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'kairos_walkthrough');
});
