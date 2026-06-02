import { expect, type Locator, type Page } from '@playwright/test';



export class LeaderboardPage {
  readonly page: Page;
  readonly pageHeader: Locator;
  readonly headerRow: Locator;
  readonly emptyState: Locator;

  constructor(page: Page) {
    this.page = page;
    this.pageHeader = page.getByTestId('page-header');
    this.headerRow = page.getByTestId('leaderboard-header');
    this.emptyState = page.getByTestId('empty-state');
  }

  async goto() {
    await this.page.goto('/leaderboard');
    await expect(this.pageHeader).toContainText('Leaderboard');
  }

  async verifyLayout() {
    await expect(this.headerRow).toBeVisible();
    await expect(this.emptyState).toBeVisible();
  }
}
