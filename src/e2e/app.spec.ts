import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('app smoke', async ({ page, request }) => { await currentAppSmoke(page, request, 'app'); });
