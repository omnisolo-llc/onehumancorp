import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('smoke test success_milestones', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'success_milestones');
});
