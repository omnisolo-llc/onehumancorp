import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('mission_track smoke', async ({ page, request }) => { await currentAppSmoke(page, request, 'mission_track'); });
