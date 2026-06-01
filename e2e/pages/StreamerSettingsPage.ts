import { expect, type Locator, type Page } from '@playwright/test';

export class StreamerSettingsPage {
  readonly page: Page;
  readonly displayNameInput: Locator;
  readonly bioInput: Locator;
  readonly usernameInput: Locator;
  readonly saveButton: Locator;
  readonly successMessage: Locator;

  // Upload components
  readonly avatarFileInput: Locator;
  readonly saveAvatarButton: Locator;
  readonly avatarUploadStatus: Locator;
  
  readonly mediaFileInput: Locator;
  readonly uploadMediaButton: Locator;
  readonly mediaUploadSuccess: Locator;
  readonly mediaUploadError: Locator;

  constructor(page: Page) {
    this.page = page;
    this.displayNameInput = page.getByTestId('settings-display-name-input');
    this.bioInput = page.getByTestId('settings-bio-input');
    this.usernameInput = page.getByTestId('settings-username-input');
    this.saveButton = page.getByTestId('settings-save-button');
    this.successMessage = page.getByTestId('settings-success-message');

    this.avatarFileInput = page.getByTestId('avatar-file-input');
    this.saveAvatarButton = page.getByTestId('save-avatar-button');
    this.avatarUploadStatus = page.getByTestId('avatar-upload-status');

    this.mediaFileInput = page.getByTestId('media-file-input');
    this.uploadMediaButton = page.getByTestId('upload-media-button');
    this.mediaUploadSuccess = page.getByTestId('media-upload-success');
    this.mediaUploadError = page.getByTestId('media-upload-error');
  }

  async goto() {
    await this.page.goto('/dashboard/settings');
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

  async uploadAvatar(file: { name: string, mimeType: string, buffer: Buffer }) {
    await this.avatarFileInput.setInputFiles(file);
    await expect(this.saveAvatarButton).toBeVisible({ timeout: 10000 });
    await this.saveAvatarButton.click();
  }

  async uploadMedia(file: { name: string, mimeType: string, buffer: Buffer }) {
    await this.mediaFileInput.setInputFiles(file);
    await this.uploadMediaButton.click();
  }
}
