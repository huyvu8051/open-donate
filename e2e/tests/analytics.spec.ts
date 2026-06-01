import { test, expect } from '@playwright/test';
import { faker } from '@faker-js/faker';
import { DashboardPage } from '../pages/DashboardPage';
import { RegisterPage } from '../pages/RegisterPage';

test.describe('Streamer Analytics Page E2E', () => {
  test('New registered streamer can access analytics, see KPI cards and ECharts components', async ({ page }) => {
    const dashboardPage = new DashboardPage(page);
    const registerPage = new RegisterPage(page);

    // 1. Register a new user to access the dashboard
    const fakeEmail = faker.internet.email();
    console.log(`Registering user for analytics test: ${fakeEmail}`);

    await registerPage.register(fakeEmail);
    await dashboardPage.waitForDashboard();

    // 2. Navigate to the Analytics Page
    console.log('Navigating to analytics page...');
    await page.goto('/dashboard/analytics');
    await page.waitForURL(/.*dashboard\/analytics.*/, { timeout: 30000 });

    // 3. Verify KPI cards are present with correct initial values
    console.log('Verifying KPI cards...');
    await expect(page.locator('text=Total Revenue')).toBeVisible({ timeout: 10000 });
    await expect(page.locator('text=Total Donations')).toBeVisible();
    await expect(page.locator('text=Avg. Donation')).toBeVisible();
    await expect(page.locator('text=Largest Single')).toBeVisible();

    // 4. Verify all ECharts chart containers exist and are rendered
    console.log('Checking chart containers...');
    const charts = [
      'chart-revenue-time',
      'chart-cumulative',
      'chart-top-donors',
      'chart-payment-method',
      'chart-amount-dist',
    ];

    for (const chartId of charts) {
      const chartEl = page.locator(`#${chartId}`);
      await expect(chartEl).toBeVisible({ timeout: 10000 });
      // Ensure ECharts has attached the canvas or generated svg/content inside the container
      const childCount = await chartEl.locator('> *').count();
      expect(childCount).toBeGreaterThan(0);
    }

    // 5. Test Time Range Filters (24h, 7d, 30d)
    console.log('Testing time range selectors...');
    const timeRanges = ['24h', '7d', '30d'];
    for (const label of timeRanges) {
      const button = page.locator(`button:has-text("${label}")`);
      await expect(button).toBeVisible();
      await button.click();
      
      // Allow brief moment for API request to fulfill and charts to re-render
      await page.waitForTimeout(500);
      
      // Active button should have bg-primary class
      await expect(button).toHaveClass(/.*bg-primary.*/);
    }

    console.log('Analytics Page E2E tests passed successfully!');
  });
});
