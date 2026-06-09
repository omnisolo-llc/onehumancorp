import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('smoke test telemetry_visualizer', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'telemetry_visualizer');
});
