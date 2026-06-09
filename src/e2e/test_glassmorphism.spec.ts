import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('Verify glassmorphism and core app smoke test', async ({ page, request }) => {
    test.setTimeout(120000);
    await currentAppSmoke(page, request, 'test_glassmorphism');
});
