import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('inbox smoke', async ({ page, request }) => { await currentAppSmoke(page, request, 'inbox'); });
