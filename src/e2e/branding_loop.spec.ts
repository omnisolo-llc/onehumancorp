import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('branding_loop smoke', async ({ page, request }) => { await currentAppSmoke(page, request, 'branding_loop'); });
