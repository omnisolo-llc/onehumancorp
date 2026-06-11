import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('run current app smoke', async ({ page, request }) => {
    await currentAppSmoke(page, request, 'test_e2e_run');
});
