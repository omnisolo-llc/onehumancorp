import { Page, expect } from '@playwright/test';
import { BasePage } from './BasePage';

export class DashboardPage extends BasePage {
    async navigateTo(menuItem: string) {
        await this.page.click(`text="${menuItem}"`);
    }
}
