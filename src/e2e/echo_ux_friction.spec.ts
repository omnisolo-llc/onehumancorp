import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('echo_ux_friction smoke', async ({ page, request }) => { await currentAppSmoke(page, request, 'echo_ux_friction'); });
