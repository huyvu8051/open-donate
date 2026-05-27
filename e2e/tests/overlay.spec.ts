import { test, expect, chromium } from '@playwright/test';
import { faker } from '@faker-js/faker';
import * as fs from 'fs';
import { DashboardPage } from './pages/DashboardPage';
import { OverlayPage } from './pages/OverlayPage';

test.describe('Overlay Flow E2E', () => {
  test('Streamer can open overlay, see status change, and test donation', async () => {
    test.setTimeout(90000); // Increase timeout since this test has long waits for animations
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
    
    // Playwright disables autoplay without user gesture in Safari/WebKit sometimes.
    // We must dismiss the prompt to unlock audio and see the mock donation.
    await overlayPage.dismissInteractionPrompt();

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
    
    // Wait for the first donation to finish displaying and cleanup
    await overlayPage.page.waitForTimeout(7000);

    // ==========================================
    // 8. Test Pause Overlay
    // ==========================================
    console.log('Streamer: Toggling Pause Overlay...');
    await dashboardPage.togglePause();
    
    console.log('Streamer: Sending another test donation while paused...');
    await dashboardPage.testOverlay();
    
    console.log('OBS: Verifying no donation appears (Overlay is Paused)...');
    await overlayPage.verifyNoMockDonation();
    
    // ==========================================
    // 9. Test Resume Overlay
    // ==========================================
    console.log('Streamer: Resuming Overlay...');
    await dashboardPage.togglePause();
    
    console.log('OBS: Waiting for queued donation to appear after resuming...');
    await overlayPage.checkMockDonation();

    // Wait for the second donation to finish displaying
    await overlayPage.page.waitForTimeout(7000);

    // ==========================================
    // 10. Test Mute Sound
    // ==========================================
    console.log('Streamer: Toggling Sound OFF...');
    await dashboardPage.toggleSound();
    
    console.log('Streamer: Sending another test donation (Muted)...');
    await dashboardPage.testOverlay();
    
    console.log('OBS: Verifying donation appears (Even if muted)...');
    // We can only visually verify the donation appears, Playwright doesn't easily verify if audio actually played vs didn't play
    // without complex setup, so we just ensure the overlay doesn't crash and still shows the visual.
    await overlayPage.checkMockDonation();

    console.log('Success! Overlay E2E test passed including Pause and Sound toggles.');

    await browser.close();
  });
});
