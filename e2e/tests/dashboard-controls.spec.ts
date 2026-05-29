import { test, expect } from '@playwright/test';
import { faker } from '@faker-js/faker';
import { DashboardPage } from '../pages/DashboardPage';

test.describe('Dashboard Controls & Interactivity E2E', () => {
  test('Should allow toggle pause, toggle sound, copy overlay link, and mark transactions as viewed', async ({ page }) => {
    test.setTimeout(45000);
    const dashboardPage = new DashboardPage(page);

    // ==========================================
    // 1. Streamer Registers & Logs In directly via App
    // ==========================================
    console.log('Navigating to app register endpoint...');
    await page.goto('/register');

    const fakeEmail = faker.internet.email();
    console.log(`Registering new user: ${fakeEmail}`);

    await page.fill('input[name="email"]', fakeEmail);

    const password = '@Aa123456';
    await page.fill('input[name="password"]', password);
    await page.fill('input[name="password_confirm"]', password);
    await page.click('button[type="submit"]');

    console.log('Waiting for redirect back to app dashboard...');
    await page.waitForURL(/\/dashboard/, { timeout: 15000 });

    await dashboardPage.waitForDashboard();

    // ==========================================
    // 2. Toggle Pause and Sound
    // ==========================================
    console.log('Toggling pause overlay...');
    await expect(dashboardPage.togglePauseBtn).toContainText('Pause Overlay');
    await dashboardPage.togglePause();
    await expect(dashboardPage.togglePauseBtn).toContainText('Overlay Paused');
    await dashboardPage.togglePause();
    await expect(dashboardPage.togglePauseBtn).toContainText('Pause Overlay');

    console.log('Toggling sound...');
    await expect(dashboardPage.toggleSoundBtn).toContainText('Sound ON');
    await dashboardPage.toggleSound();
    await expect(dashboardPage.toggleSoundBtn).toContainText('Sound OFF');
    await dashboardPage.toggleSound();
    await expect(dashboardPage.toggleSoundBtn).toContainText('Sound ON');

    // ==========================================
    // 3. Click Copy Overlay Link
    // ==========================================
    // Mock clipboard API to avoid permission/headless environment issues
    let copiedText = '';
    await page.exposeFunction('mockWriteText', (text: string) => {
        copiedText = text;
    });
    await page.evaluate(() => {
        Object.defineProperty(navigator, 'clipboard', {
            value: {
                writeText: (text: string) => {
                    (window as any).mockWriteText(text);
                    return Promise.resolve();
                }
            },
            configurable: true
        });
    });

    console.log('Clicking copy overlay link...');
    await dashboardPage.copyOverlayLink();
    
    // Verify copied text contains overlay path
    await page.waitForTimeout(500);
    expect(copiedText).toContain('/overlay/');

    // ==========================================
    // 4. Test Overlay Donation & Mark as Viewed
    // ==========================================
    console.log('Triggering test overlay donation...');
    await dashboardPage.testOverlay();

    // Wait and reload to get the new donation in the table
    await page.waitForTimeout(3000);
    await page.reload();

    const donationRow = page.getByTestId('donation-row').first();
    await expect(donationRow).toBeVisible({ timeout: 10000 });

    // Look for the "Mark as Viewed" button/link inside the row
    const markAsViewedBtn = donationRow.locator('button:has-text("Mark as Viewed")');
    await expect(markAsViewedBtn).toBeVisible({ timeout: 5000 });
    
    console.log('Clicking Mark as Viewed...');
    await markAsViewedBtn.click();
    
    // Check that status updates to "Displayed" and "Mark as Viewed" button disappears
    await expect(markAsViewedBtn).toBeHidden({ timeout: 5000 });
    await expect(donationRow).toContainText('Displayed');
    console.log('Marked transaction viewed successfully!');
  });
});
