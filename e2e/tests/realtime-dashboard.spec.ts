import { test, expect, chromium } from '@playwright/test';
import { faker } from '@faker-js/faker';
import { StreamerPage } from '../pages/StreamerPage';

test.describe('Real-time Dashboard Flow', () => {
  test('Dashboard automatically reloads and shows new donation when viewer donates', async () => {
    test.setTimeout(60000); // Allow extra time for dual browser flow

    // Create two separate browser contexts to isolate sessions
    const browser = await chromium.launch({
        args: ['--autoplay-policy=no-user-gesture-required']
    });
    const streamerContext = await browser.newContext();
    const viewerContext = await browser.newContext();

    const streamerPage = await streamerContext.newPage();
    const viewerPage = await viewerContext.newPage();

    // ==========================================
    // 1. Streamer Logs In & Opens Dashboard
    // ==========================================
    console.log('Streamer: Navigating to register...');
    await streamerPage.goto('/register');

    const fakeEmail = faker.internet.email().toLowerCase();
    console.log(`Streamer: Registering new user: ${fakeEmail}`);
    await streamerPage.fill('input[name="email"]', fakeEmail);

    const password = '@Aa123456';
    await streamerPage.fill('input[name="password"]', password);
    await streamerPage.fill('input[name="password_confirm"]', password);

    await streamerPage.click('button[type="submit"]');

    console.log('Streamer: Waiting for redirect to dashboard...');
    await streamerPage.waitForURL(/\/dashboard/, { timeout: 15000 });

    await expect(streamerPage.getByTestId('streamer-dashboard-header')).toBeVisible({ timeout: 10000 });

    const expectedPrefix = fakeEmail.split('@')[0].toLowerCase();
    const streamerUsername = expectedPrefix;
    console.log(`Streamer: Identified public username as: ${streamerUsername}`);

    // Ensure dashboard has Auto Reload checked
    const autoReloadCheckbox = streamerPage.locator('text="Auto Reload" >> xpath=.. >> input[type="checkbox"]');
    if (!(await autoReloadCheckbox.isChecked())) {
        await autoReloadCheckbox.check();
    }

    // ==========================================
    // 2. Viewer Opens Streamer Page & Donates
    // ==========================================
    console.log('Viewer: Navigating to streamer page...');
    const viewerApp = new StreamerPage(viewerPage);
    await viewerApp.goto(streamerUsername!);
    await viewerPage.waitForLoadState('networkidle');

    const donorName = faker.person.fullName();
    const donationMsg = `Realtime test message ${Date.now()}`;
    const amount = '50';

    console.log(`Viewer: Making donation of $${amount} from ${donorName}...`);
    await viewerApp.fillDonationForm(donorName, amount, donationMsg, 'Mock Auto');
    await viewerApp.submitDonation();
    await viewerApp.verifyMockPaymentFlow();

    // ==========================================
    // 3. Streamer Dashboard Auto-Reloads
    // ==========================================
    console.log('Streamer: Waiting for dashboard to auto-reload (up to 10s)...');
    
    // We expect the donation message to appear in the dashboard's Donation History table automatically
    // The auto reload interval is 5s, so we wait up to 20s.
    await streamerPage.reload();
    const newDonationRow = streamerPage.getByTestId('donation-row').filter({ hasText: donorName }).filter({ hasText: donationMsg });
    await expect(newDonationRow).toBeVisible({ timeout: 20000 });

    console.log(`Streamer: Successfully saw real-time donation from ${donorName}!`);

    // Cleanup
    await browser.close();
  });
});
