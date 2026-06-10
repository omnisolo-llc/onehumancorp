import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('portfolio_generator smoke', async ({ page, request }) => { await currentAppSmoke(page, request, 'portfolio_generator'); });
