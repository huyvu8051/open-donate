import { expect, type Page, type Locator } from '@playwright/test';

export class OverlayPage {
    readonly page: Page;
    readonly donorName: Locator;
    readonly amount: Locator;
    readonly message: Locator;

    constructor(page: Page) {
        this.page = page;
        this.donorName = page.getByTestId('mock-donor-name');
        this.amount = page.getByTestId('mock-amount');
        this.message = page.getByTestId('mock-message');
    }

    async open(url: string) {
        await this.page.goto(url);
    }

    async checkMockDonation() {
        await expect(this.donorName).toBeVisible({ timeout: 15000 });
        await expect(this.donorName).toHaveText('System Test');
        await expect(this.amount).toBeVisible();
        await expect(this.amount).toHaveText('$5.00');
        await expect(this.message).toBeVisible();
        await expect(this.message).toHaveText(/"This is a test donation for your overlay!"/);
    }
}
