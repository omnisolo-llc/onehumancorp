import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('currentAppSmoke: kairos_walkthrough', async ({ page, request }) => { await currentAppSmoke(page, request, 'kairos_walkthrough'); });
