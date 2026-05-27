import { expect, type Locator, type Page } from '@playwright/test';

export class StreamerSettingsPage {
  readonly page: Page;
  readonly displayNameInput: Locator;
  readonly bioInput: Locator;
  readonly usernameInput: Locator;
  readonly saveButton: Locator;
  readonly successMessage: Locator;

  constructor(page: Page) {
    this.page = page;
    this.displayNameInput = page.getByTestId('settings-display-name-input');
    this.bioInput = page.getByTestId('settings-bio-input');
    this.usernameInput = page.getByTestId('settings-username-input');
    this.saveButton = page.getByTestId('settings-save-button');
    this.successMessage = page.getByTestId('settings-success-message');
  }

  async goto() {
    await this.page.goto('http://localhost:3000/dashboard/settings');
    await this.waitForLoad();
  }

  async waitForLoad() {
    await expect(this.displayNameInput).toBeVisible({ timeout: 10000 });
  }

  async updateSettings(displayName: string, bio: string, username: string) {
    await this.displayNameInput.fill(displayName);
    await this.bioInput.fill(bio);
    await this.usernameInput.fill(username);
    await this.saveButton.click();
  }

  async verifySuccessMessage() {
    await expect(this.successMessage).toBeVisible({ timeout: 5000 });
  }

  async verifySettingsValues(displayName: string, bio: string, username: string) {
    await expect(this.displayNameInput).toHaveValue(displayName);
    await expect(this.bioInput).toHaveValue(bio);
    await expect(this.usernameInput).toHaveValue(username);
  }
}
