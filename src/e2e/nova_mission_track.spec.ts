import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('nova_mission_track smoke', async ({ page, request }) => { await currentAppSmoke(page, request, 'nova_mission_track'); });
