import { expect, type Locator, type Page } from '@playwright/test';

export class KeycloakLoginPage {
  readonly page: Page;
  readonly registerButton: Locator;
  readonly emailInput: Locator;
  readonly passwordInput: Locator;
  readonly passwordConfirmInput: Locator;
  readonly submitButton: Locator;

  constructor(page: Page) {
    this.page = page;
    this.registerButton = page.locator('text="Register"').first();
    this.emailInput = page.locator('input[name="email"], input[type="email"]');
    this.passwordInput = page.locator('input[type="password"], input[name="password"]').first();
    this.passwordConfirmInput = page.locator('input[name="password-confirm"]');
    this.submitButton = page.locator('button[type="submit"], input[type="submit"]');
  }

  async gotoAppLogin() {
    await this.page.goto('http://localhost:3000/api/login');
    await this.page.waitForURL(/.*localhost:8080.*/);
  }

  async registerNewUser(email: string, password: string = '@Aa123456') {
    await expect(this.registerButton).toBeVisible({ timeout: 5000 });
    await this.registerButton.click();
    await this.page.waitForURL(/.*\/login-actions\/registration.*/);

    await this.emailInput.fill(email);
    await expect(this.passwordInput).toBeVisible({ timeout: 5000 });
    await this.passwordInput.fill(password);
    
    if (await this.passwordConfirmInput.count() > 0) {
      await this.passwordConfirmInput.fill(password);
    }

    await this.submitButton.click();
    await this.page.waitForURL(/.*localhost:3000.*/, { timeout: 15000 });
  }
}
