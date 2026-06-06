import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('currentAppSmoke: telemetry_visualizer', async ({ page, request }) => { await currentAppSmoke(page, request, 'telemetry_visualizer'); });
