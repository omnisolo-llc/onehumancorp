import { Page, expect } from '@playwright/test';
import { BasePage } from './BasePage';

export class LoginPage extends BasePage {
    async login(email = 'test@example.com', password = 'password123') {
        await this.page.goto('/login');
        await this.page.fill('input[placeholder="Email or Username"]', email);
        await this.page.fill('input[placeholder="Password"]', password);
        await this.page.click('button:has-text("Login")');
    }
}
