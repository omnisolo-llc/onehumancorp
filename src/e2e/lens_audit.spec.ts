import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('Lens Audit: Smoke test and UI integrity', async ({ page, request }) => {
    test.setTimeout(120000);
    await currentAppSmoke(page, request, 'lens_audit');
});
