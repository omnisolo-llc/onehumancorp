import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('telemetry_visualizer smoke', async ({ page, request }) => { await currentAppSmoke(page, request, 'telemetry_visualizer'); });
