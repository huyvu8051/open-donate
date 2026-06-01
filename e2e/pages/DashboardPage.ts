import { expect, type Page, type Locator } from '@playwright/test';

export class DashboardPage {
    readonly page: Page;
    readonly header: Locator;
    readonly activeStatus: Locator;
    readonly inactiveStatus: Locator;
    readonly testOverlayBtn: Locator;
    readonly copyOverlayLinkBtn: Locator;
    readonly togglePauseBtn: Locator;
    readonly toggleSoundBtn: Locator;

    constructor(page: Page) {
        this.page = page;
        this.header = page.getByTestId('streamer-dashboard-header');
        this.activeStatus = page.getByTestId('overlay-status-active');
        this.inactiveStatus = page.getByTestId('overlay-status-inactive');
        this.testOverlayBtn = page.getByTestId('test-overlay-btn');
        this.copyOverlayLinkBtn = page.getByTestId('copy-overlay-link-btn');
        this.togglePauseBtn = page.getByTestId('toggle-pause-btn');
        this.toggleSoundBtn = page.getByTestId('toggle-sound-btn');
    }

    async waitForDashboard() {
        await this.page.waitForURL(/\/dashboard/, { timeout: 120000 });
        await expect(this.header).toBeVisible({ timeout: 15000 });
    }

    async checkInactiveStatus() {
        await expect(this.inactiveStatus).toBeVisible({ timeout: 10000 });
    }

    async checkActiveStatus() {
        await expect(this.activeStatus).toBeVisible({ timeout: 15000 });
    }

    async copyOverlayLink() {
        await this.copyOverlayLinkBtn.click();
    }

    async testOverlay() {
        await this.testOverlayBtn.click();
    }

    async togglePause() {
        await this.togglePauseBtn.click();
    }

    async toggleSound() {
        await this.toggleSoundBtn.click();
    }
}
