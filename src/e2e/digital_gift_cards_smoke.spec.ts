import { test, expect } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('digital_gift_cards');
