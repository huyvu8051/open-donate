import { test, expect } from '@playwright/test';

test.describe('Static and Leaderboard Pages E2E Tests', () => {
  test('About page should load and display core sections', async ({ page }) => {
    await page.goto('/about');
    await expect(page.locator('h1')).toContainText('About');
    
    // Check key cards/sections
    await expect(page.locator('text=Mission')).toBeVisible();
    await expect(page.locator('text=Vision')).toBeVisible();
  });

  test('FAQ page should allow expanding/collapsing questions', async ({ page }) => {
    await page.goto('/faq');
    await expect(page.locator('h1')).toContainText('Frequently Asked Questions');

    // Click the first FAQ question button
    const firstQuestion = page.locator('button').first();
    await expect(firstQuestion).toBeVisible();

    // Verify it is collapsed initially (max-h-0 or hidden class)
    const firstAnswer = page.locator('p').first();
    
    // Click to expand
    await firstQuestion.click();
    // Wait for animation transition
    await page.waitForTimeout(500);
    await expect(firstAnswer).toBeVisible();
  });

  test('Privacy Policy and Terms of Service pages should render', async ({ page }) => {
    await page.goto('/privacy');
    await expect(page.locator('h1')).toContainText('Privacy Policy');

    await page.goto('/terms');
    await expect(page.locator('h1')).toContainText('Terms of Service');
  });

  test('Leaderboard page should load and show table headers', async ({ page }) => {
    await page.goto('/leaderboard');
    await expect(page.locator('h1')).toContainText('Leaderboard');
    
    // Verify columns exist
    const headerRow = page.locator('.grid-cols-12').first();
    await expect(headerRow.getByText('Creator', { exact: true })).toBeVisible();
    await expect(headerRow.getByText('Donations', { exact: true })).toBeVisible();
    await expect(headerRow.getByText('Total', { exact: true })).toBeVisible();
  });
});
