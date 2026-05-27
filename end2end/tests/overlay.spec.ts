import { test, expect, chromium } from '@playwright/test';
import { faker } from '@faker-js/faker';
import * as fs from 'fs';
import { DashboardPage } from './pages/DashboardPage';
import { OverlayPage } from './pages/OverlayPage';

test.describe('Overlay Flow E2E', () => {
  test('Streamer can open overlay, see status change, and test donation', async () => {
    // We need to allow multiple contexts to simulate Streamer + OBS Browser Source
    const browser = await chromium.launch();
    const streamerContext = await browser.newContext();
    const overlayContext = await browser.newContext();

    // Setup permission for clipboard copy
    await streamerContext.grantPermissions(['clipboard-read', 'clipboard-write']);

    const streamerPage = await streamerContext.newPage();
    const overlayPageRaw = await overlayContext.newPage();

    const dashboardPage = new DashboardPage(streamerPage);
    const overlayPage = new OverlayPage(overlayPageRaw);

    streamerPage.on('console', msg => console.log('Streamer Page Log:', msg.text()));
    streamerPage.on('pageerror', err => console.log('Streamer Page Error:', err));

    // ==========================================
    // 1. Streamer Registers & Opens Dashboard
    // ==========================================
    console.log('Streamer: Navigating to register...');
    const testEmail = faker.internet.email();
    console.log(`Streamer: Registering with email: ${testEmail}`);
    
    await dashboardPage.register(testEmail);

    console.log('Streamer: Waiting for redirect to dashboard...');
    await dashboardPage.waitForDashboard();

    // ==========================================
    // 2. Streamer checks initial overlay status
    // ==========================================
    console.log('Streamer: Checking initial overlay status...');
    try {
        await dashboardPage.checkInactiveStatus();
    } catch (e) {
        console.log("FAILED TO FIND INACTIVE STATUS. Saving HTML...");
        fs.writeFileSync('debug.html', await streamerPage.content());
        throw e;
    }

    // ==========================================
    // 3. Streamer copies overlay link
    // ==========================================
    console.log('Streamer: Copying overlay link...');
    await dashboardPage.copyOverlayLink();

    // Get clipboard text using playwright evaluation
    const overlayUrl = await streamerPage.evaluate(() => navigator.clipboard.readText());
    expect(overlayUrl).toContain('/overlay/');
    console.log(`Streamer: Copied URL: ${overlayUrl}`);

    // ==========================================
    // 4. OBS opens overlay link (Browser Source)
    // ==========================================
    console.log('OBS: Opening overlay link...');
    await overlayPage.open(overlayUrl);

    // ==========================================
    // 5. Streamer sees status change to Active
    // ==========================================
    console.log('Streamer: Waiting for status to become Active...');
    await dashboardPage.checkActiveStatus();
    console.log('Streamer: Status is Active!');

    // ==========================================
    // 6. Streamer tests overlay
    // ==========================================
    console.log('Streamer: Clicking Test Overlay button...');
    await dashboardPage.testOverlay();

    // ==========================================
    // 7. OBS receives and displays test donation
    // ==========================================
    console.log('OBS: Waiting for test donation animation...');
    await overlayPage.checkMockDonation();

    console.log('Success! Overlay E2E test passed.');

    await browser.close();
  });
});
