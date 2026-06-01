import { expect, type Page, type Locator } from '@playwright/test';

export class LoginPage {
    readonly page: Page;
    readonly emailInput: Locator;
    readonly passwordInput: Locator;
    readonly submitBtn: Locator;

    constructor(page: Page) {
        this.page = page;
        this.emailInput = page.getByTestId('email-input');
        this.passwordInput = page.getByTestId('password-input');
        this.submitBtn = page.getByTestId('auth-submit-btn');
    }

    async login(email: string) {
        await expect(async () => {
            await this.page.goto('/login');
            await this.page.waitForTimeout(1000);
            await this.emailInput.fill(email);
            await this.passwordInput.fill('StrongPassword123!');
            await this.submitBtn.click();
            await expect(this.page).toHaveURL(/\/dashboard/, { timeout: 10000 });
        }).toPass({ timeout: 60000 });
    }
}
