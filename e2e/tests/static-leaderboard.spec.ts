import { test, expect } from '@playwright/test';
import { LeaderboardPage } from '../pages/StaticPages';

test.describe('Leaderboard Page E2E Tests', () => {


  test('Leaderboard page should load and show table headers', async ({ page }) => {
    const leaderboardPage = new LeaderboardPage(page);
    await leaderboardPage.goto();
    
    // Verify either columns exist or empty state
    await expect(async () => {
      const isEmpty = await leaderboardPage.emptyState.isVisible();
      const hasHeader = await leaderboardPage.headerRow.getByText('Creator', { exact: true }).isVisible();
      expect(isEmpty || hasHeader).toBeTruthy();
    }).toPass();
  });
});
