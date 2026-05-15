import { Page, expect } from '@playwright/test';
import { BasePage } from './BasePage';

export class WizardPage extends BasePage {
    async startWizard() {
        await this.page.goto('/business-setup');
        await this.page.click('text=🚀 Start My Business');
    }

    async completeBusinessDetails(name: string) {
        await this.page.click('text=🛒 Online Store');
        await this.page.click('text=Next →');
        await this.page.fill('input[placeholder="e.g. Maya\'s Cakes"]', name);
        await this.page.click('text=Next →');
    }
}
