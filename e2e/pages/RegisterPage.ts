import { expect, type Page, type Locator } from '@playwright/test';
import { faker } from '@faker-js/faker';

export class RegisterPage {
    readonly page: Page;
    readonly emailInput: Locator;
    readonly passwordInput: Locator;
    readonly passwordConfirmInput: Locator;
    readonly submitBtn: Locator;
    readonly errorContainer: Locator;

    constructor(page: Page) {
        this.page = page;
        this.emailInput = page.getByTestId('email-input');
        this.passwordInput = page.getByTestId('password-input');
        this.passwordConfirmInput = page.getByTestId('password-confirm-input');
        this.submitBtn = page.getByTestId('auth-submit-btn');
        this.errorContainer = page.getByTestId('error-container');
    }

    async register(email: string) {
        await expect(async () => {
            await this.page.goto('/register');
            await this.page.waitForTimeout(1000);
            await this.emailInput.fill(email);
            await this.passwordInput.fill('StrongPassword123!');
            await this.passwordConfirmInput.fill('StrongPassword123!');
            await this.submitBtn.click();
            await expect(this.page).toHaveURL(/\/dashboard/, { timeout: 30000 });
        }).toPass({ timeout: 60000 });
    }

    async registerNewUser(): Promise<string> {
        let finalEmail = '';
        await expect(async () => {
            finalEmail = faker.internet.email();
            console.log(`Registering new user: ${finalEmail}`);
            await this.page.goto('/register');
            await this.page.waitForTimeout(1000);
            await this.emailInput.fill(finalEmail);
            await this.passwordInput.fill('StrongPassword123!');
            await this.passwordConfirmInput.fill('StrongPassword123!');
            await this.submitBtn.click();
            await expect(this.page).toHaveURL(/\/dashboard/, { timeout: 30000 });
        }).toPass({ timeout: 90000 });
        return finalEmail;
    }
}
