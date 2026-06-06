import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('currentAppSmoke: viral_trial_extension', async ({ page, request }) => { await currentAppSmoke(page, request, 'viral_trial_extension'); });
