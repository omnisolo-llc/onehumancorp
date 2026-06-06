import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('currentAppSmoke: metrics_observation', async ({ page, request }) => { await currentAppSmoke(page, request, 'metrics_observation'); });
