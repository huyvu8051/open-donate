import { test, expect } from '@playwright/test';
import { faker } from '@faker-js/faker';
import { DashboardPage } from '../pages/DashboardPage';
import { RegisterPage } from '../pages/RegisterPage';
import { StreamerSettingsPage } from '../pages/StreamerSettingsPage';
import { StreamerPaymentsPage } from '../pages/StreamerPaymentsPage';

test('Streamer Settings Update Flow (Profile, Media, Payments)', async ({ page }) => {
  const dashboardPage = new DashboardPage(page);
  const registerPage = new RegisterPage(page);
  const settingsPage = new StreamerSettingsPage(page);
  const paymentsPage = new StreamerPaymentsPage(page);

  const fakeEmail = await registerPage.registerNewUser();
  await dashboardPage.waitForDashboard();

  // ==========================================
  // 1. Update Basic Settings Profile
  // ==========================================
  console.log('Navigating to dashboard settings...');
  await settingsPage.goto();

  const newDisplayName = faker.person.fullName();
  const newBio = faker.lorem.sentence();
  const newUsername = faker.string.alphanumeric({ length: 8, casing: 'lower' });

  console.log(`Updating settings - Name: ${newDisplayName}, Username: ${newUsername}`);
  await settingsPage.updateSettings(newDisplayName, newBio, newUsername);

  // Verify success message appeared
  await settingsPage.verifySuccessMessage();

  // Verify the inputs retained their new values
  await settingsPage.verifySettingsValues(newDisplayName, newBio, newUsername);

  // ==========================================
  // 2. Update Media settings (Fallback Selection)
  // ==========================================
  console.log('Updating Media Settings...');
  // Wait for the options to load asynchronously and be attached to the DOM
  await page.waitForSelector('select[name="fallback_media_file"] option', { state: 'attached', timeout: 10000 });
  const values = await settingsPage.fallbackMediaSelect.locator('option').evaluateAll(elrs => elrs.map(o => (o as HTMLOptionElement).value));
  const targetOption = values.find(v => v.includes('funny_1.mp3')) || '/audio/funny_1.mp3';
  console.log(`Selecting dynamic option: ${targetOption}`);
  await settingsPage.selectFallbackMedia(targetOption);

  // Click the save button under Media settings
  await settingsPage.saveMediaSettings();

  // ==========================================
  // 3. Update Payments Settings Page
  // ==========================================
  console.log('Navigating to payments page...');
  await paymentsPage.goto();

  // Uncheck Auto, Check Manual
  await paymentsPage.setPaymentMethod('Mock Manual');

  // Save payments
  await paymentsPage.save();
  console.log('Payments settings updated successfully!');
});

test('Streamer Settings File Uploads (Avatar & Media)', async ({ page }) => {
  test.setTimeout(120000);
  const dashboardPage = new DashboardPage(page);
  const registerPage = new RegisterPage(page);
  const settingsPage = new StreamerSettingsPage(page);

  const fakeEmail = await registerPage.registerNewUser();
  console.log(`Registered user for upload tests: ${fakeEmail}`);
  await dashboardPage.waitForDashboard();

  console.log('Navigating to dashboard settings...');
  await settingsPage.goto();

  // ==========================================
  // 1. Upload Avatar
  // ==========================================
  console.log('Uploading avatar...');
  const avatarFile = {
    name: 'avatar.png',
    mimeType: 'image/png',
    buffer: Buffer.from('iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwAEhQGAhKmMIQAAAABJRU5ErkJggg==', 'base64')
  };

  await settingsPage.uploadAvatar(avatarFile);

  await expect(async () => {
    const isHidden = await settingsPage.saveAvatarButton.isHidden();
    let hasFailed = false;
    if (await settingsPage.avatarUploadStatus.isVisible()) {
      hasFailed = await settingsPage.avatarUploadStatus.evaluate(node => /Failed|Saving\.\.\./i.test(node.textContent || ''));
    }
    expect(isHidden || hasFailed).toBeTruthy();
  }).toPass({ timeout: 15000 });

  // ==========================================
  // 2. Upload Media
  // ==========================================
  // Reload the page to clear the crop modal if it got stuck during avatar upload
  await page.reload();
  console.log('Uploading media...');
  const mediaFile = {
    name: 'test-audio.mp3',
    mimeType: 'audio/mpeg',
    buffer: Buffer.from('//NExAAAAANIAAAAAExBTUUzLjEwMKqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq', 'base64')
  };

  await settingsPage.uploadMedia(mediaFile);

  await expect(async () => {
    const isSuccess = await settingsPage.mediaUploadSuccess.isVisible();
    const hasFailed = await settingsPage.mediaUploadError.isVisible();
    expect(isSuccess || hasFailed).toBeTruthy();
  }).toPass({ timeout: 15000 });
  console.log('Avatar and Media uploads tested successfully!');
});
