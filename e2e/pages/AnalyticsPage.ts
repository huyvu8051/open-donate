import { expect, type Locator, type Page } from '@playwright/test';

export class AnalyticsPage {
  readonly page: Page;
  readonly totalRevenue: Locator;
  readonly totalDonations: Locator;
  readonly avgDonation: Locator;
  readonly largestSingle: Locator;

  constructor(page: Page) {
    this.page = page;
    this.totalRevenue = page.locator('text=Total Revenue');
    this.totalDonations = page.locator('text=Total Donations');
    this.avgDonation = page.locator('text=Avg. Donation');
    this.largestSingle = page.locator('text=Largest Single');
  }

  async goto() {
    await this.page.goto('/dashboard/analytics');
    await this.page.waitForURL(/.*dashboard\/analytics.*/, { timeout: 30000 });
  }

  async verifyKpis() {
    await expect(this.totalRevenue).toBeVisible({ timeout: 10000 });
    await expect(this.totalDonations).toBeVisible();
    await expect(this.avgDonation).toBeVisible();
    await expect(this.largestSingle).toBeVisible();
  }

  getChartContainer(chartId: string) {
    return this.page.locator(`#${chartId}`);
  }

  async verifyChartsRendered(chartIds: string[]) {
    for (const chartId of chartIds) {
      const chartEl = this.getChartContainer(chartId);
      await expect(chartEl).toBeVisible({ timeout: 10000 });
      const childCount = await chartEl.locator('> *').count();
      expect(childCount).toBeGreaterThan(0);
    }
  }

  async setTimeRange(label: string) {
    const button = this.page.locator(`button:has-text("${label}")`);
    await expect(button).toBeVisible();
    await button.click();
    await this.page.waitForTimeout(500);
    await expect(button).toHaveClass(/.*bg-primary.*/);
  }
}
