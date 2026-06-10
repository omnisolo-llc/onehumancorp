import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('metrics_observation smoke', async ({ page, request }) => { await currentAppSmoke(page, request, 'metrics_observation'); });
