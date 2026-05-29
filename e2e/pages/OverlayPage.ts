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

    async dismissInteractionPrompt() {
        // Try to click the interaction prompt if it appears (to unlock audio)
        try {
            const prompt = this.page.getByText(/Click to enable Audio/);
            await prompt.waitFor({ state: 'visible', timeout: 5000 });
            await prompt.click();
            console.log('Dismissed interaction prompt on overlay.');
        } catch (e) {
            // It might not appear if autoplay is allowed (e.g. chromium)
            console.log('No interaction prompt appeared.');
        }
    }

    async checkMockDonation() {
        await expect(this.donorName).toBeVisible({ timeout: 15000 });
        await expect(this.donorName).not.toBeEmpty();
        await expect(this.amount).toBeVisible();
        await expect(this.amount).toContainText('$');
        await expect(this.message).toBeVisible();
        await expect(this.message).not.toBeEmpty();
    }

    async verifyNoMockDonation() {
        // Wait briefly to ensure it doesn't appear
        await this.page.waitForTimeout(5000);
        await expect(this.donorName).toBeHidden();
    }
}
