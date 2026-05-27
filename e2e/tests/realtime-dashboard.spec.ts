import { test, expect, chromium } from '@playwright/test';
import { faker } from '@faker-js/faker';
import { StreamerPage } from '../pages/StreamerPage';

test.describe('Real-time Dashboard Flow', () => {
  test('Dashboard automatically reloads and shows new donation when viewer donates', async () => {
    test.setTimeout(60000); // Allow extra time for dual browser flow

    // Create two separate browser contexts to isolate sessions
    const browser = await chromium.launch();
    const streamerContext = await browser.newContext();
    const viewerContext = await browser.newContext();

    const streamerPage = await streamerContext.newPage();
    const viewerPage = await viewerContext.newPage();

    // ==========================================
    // 1. Streamer Logs In & Opens Dashboard
    // ==========================================
    console.log('Streamer: Navigating to login...');
    await streamerPage.goto('http://localhost:3000/api/login');
    await streamerPage.waitForURL(/.*localhost:8080.*/);

    const registerButton = streamerPage.locator('text="Register"').first();
    await expect(registerButton).toBeVisible({ timeout: 5000 });
    await registerButton.click();
    await streamerPage.waitForURL(/.*\/login-actions\/registration.*/);

    const fakeEmail = faker.internet.email();
    console.log(`Streamer: Registering new user: ${fakeEmail}`);
    await streamerPage.fill('input[name="email"], input[type="email"]', fakeEmail);

    const firstNameInput = streamerPage.locator('input[name="firstName"]');
    if (await firstNameInput.count() > 0) await firstNameInput.fill(faker.person.firstName());
    
    const lastNameInput = streamerPage.locator('input[name="lastName"]');
    if (await lastNameInput.count() > 0) await lastNameInput.fill(faker.person.lastName());

    const password = '@Aa123456';
    await streamerPage.fill('input[type="password"], input[name="password"]', password);
    const passwordConfirm = streamerPage.locator('input[name="password-confirm"]');
    if (await passwordConfirm.count() > 0) await passwordConfirm.fill(password);

    await streamerPage.click('button[type="submit"], input[type="submit"]');

    console.log('Streamer: Waiting for redirect to dashboard...');
    // In our new flow, auth callback redirects to /dashboard directly, but we accept / as fallback
    await streamerPage.waitForURL(/.*localhost:3000.*/, { timeout: 15000 });
    if (streamerPage.url() !== 'http://localhost:3000/dashboard') {
        await streamerPage.goto('http://localhost:3000/dashboard');
    }

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
