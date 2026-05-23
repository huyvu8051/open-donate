import { expect, type Locator, type Page } from '@playwright/test';

export class StreamerPage {
  readonly page: Page;
  readonly nameInput: Locator;
  readonly messageInput: Locator;
  readonly donateButton: Locator;
  readonly otpInput: Locator;
  readonly paymentSuccessMsg: Locator;
  readonly tributeSection: Locator;

  constructor(page: Page) {
    this.page = page;
    this.nameInput = page.getByTestId('donor-name-input');
    this.messageInput = page.getByTestId('donation-message-input');
    this.donateButton = page.getByTestId('donate-submit-btn');
    this.otpInput = page.getByTestId('mock-otp-input');
    this.paymentSuccessMsg = page.getByTestId('payment-success-msg');
    this.tributeSection = page.getByTestId('recent-tributes-section');
  }

  async goto(username: string) {
    await this.page.goto(`/streamer/${username}`);
  }

  async verifyStreamerName(name: string) {
    await expect(this.page.getByRole('heading', { name })).toBeVisible();
  }

  async selectPresetAmount(amount: string) {
    await this.page.getByRole('button', { name: `$${amount}` }).click();
  }

  async selectPaymentMethod(method: string) {
    await this.page.getByText(method, { exact: true }).click();
  }

  async fillDonationForm(name: string, amount: string, message: string, paymentMethod: string) {
    await this.nameInput.fill(name);
    await this.selectPresetAmount(amount);
    await this.messageInput.fill(message);
    await this.selectPaymentMethod(paymentMethod);
  }

  async submitDonation() {
    await this.donateButton.click();
  }

  async verifyMockPaymentFlow() {
    await expect(this.otpInput).toBeVisible();
    await expect(this.paymentSuccessMsg).toBeVisible({ timeout: 10000 });
  }

  async verifyDonationInTributes(donorName: string, amount: string, message: string) {
    await expect(this.tributeSection.getByText(donorName).first()).toBeVisible({ timeout: 5000 });
    await expect(this.tributeSection.getByText(message).first()).toBeVisible();
    await expect(this.tributeSection.getByText(`$${amount}.00`).first()).toBeVisible();
  }
}
