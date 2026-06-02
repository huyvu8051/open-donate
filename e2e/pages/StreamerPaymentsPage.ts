import { expect, type Locator, type Page } from '@playwright/test';

export class StreamerPaymentsPage {
  readonly page: Page;
  readonly header: Locator;
  readonly mockAutoCheckbox: Locator;
  readonly mockManualCheckbox: Locator;
  readonly savePaymentsButton: Locator;
  readonly successMessage: Locator;

  constructor(page: Page) {
    this.page = page;
    this.header = page.getByTestId('streamer-payments-header');
    this.mockAutoCheckbox = page.locator('input[type="checkbox"][value="Mock Auto"]');
    this.mockManualCheckbox = page.locator('input[type="checkbox"][value="Mock Manual"]');
    this.savePaymentsButton = page.getByTestId('save-payments-btn');
    this.successMessage = page.getByTestId('payments-success-message');
  }

  async goto() {
    await this.page.goto('/dashboard/payments');
    await this.page.waitForURL(/.*dashboard\/payments.*/, { timeout: 30000 });
    await expect(this.header).toBeVisible();
  }

  async setPaymentMethod(method: 'Mock Auto' | 'Mock Manual') {
    if (method === 'Mock Auto') {
      if (!(await this.mockAutoCheckbox.isChecked())) await this.mockAutoCheckbox.check();
      if (await this.mockManualCheckbox.isChecked()) await this.mockManualCheckbox.uncheck();
    } else {
      if (await this.mockAutoCheckbox.isChecked()) await this.mockAutoCheckbox.uncheck();
      if (!(await this.mockManualCheckbox.isChecked())) await this.mockManualCheckbox.check();
    }
  }

  async save() {
    await this.savePaymentsButton.click();
    await expect(this.successMessage).toBeVisible({ timeout: 5000 });
  }
}
