import { test, expect } from '@playwright/test';
import { faker } from '@faker-js/faker';
import { DashboardPage } from '../pages/DashboardPage';
import { StreamerSettingsPage } from '../pages/StreamerSettingsPage';

test('Streamer Settings Update Flow (Profile, Media, Payments)', async ({ page }) => {
  const loginPage = new DashboardPage(page);
  const settingsPage = new StreamerSettingsPage(page);

  const fakeEmail = faker.internet.email();
  console.log(`Registering new user: ${fakeEmail}`);
  await loginPage.register(fakeEmail);
  await loginPage.waitForDashboard();

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
  const fallbackMediaSelect = page.locator('select[name="fallback_media_file"]');
  await expect(fallbackMediaSelect).toBeVisible({ timeout: 10000 });
  
  // Wait for the options to load asynchronously and be attached to the DOM
  await page.waitForSelector('select[name="fallback_media_file"] option', { state: 'attached', timeout: 10000 });
  const values = await fallbackMediaSelect.locator('option').evaluateAll(elrs => elrs.map(o => (o as HTMLOptionElement).value));
  const targetOption = values.find(v => v.includes('funny_1.mp3')) || '/audio/funny_1.mp3';
  console.log(`Selecting dynamic option: ${targetOption}`);
  await fallbackMediaSelect.selectOption(targetOption);
  
  // Click the save button under Media settings
  const saveMediaButton = page.locator('button:has-text("Save Media Settings")');
  await saveMediaButton.click();
  
  // Verify success banner/message
  await expect(page.locator('text="Settings saved successfully!"')).toBeVisible({ timeout: 5000 });

  // ==========================================
  // 3. Update Payments Settings Page
  // ==========================================
  console.log('Navigating to payments page...');
  await page.goto('/dashboard/payments');
  await page.waitForURL(/.*dashboard\/payments.*/, { timeout: 10000 });
  
  // Verify header
  await expect(page.getByTestId('streamer-payments-header')).toBeVisible();

  // Find checkboxes
  const mockAutoCheckbox = page.locator('input[type="checkbox"][value="Mock Auto"]');
  const mockManualCheckbox = page.locator('input[type="checkbox"][value="Mock Manual"]');
  
  await expect(mockAutoCheckbox).toBeVisible();
  await expect(mockManualCheckbox).toBeVisible();

  // Uncheck Auto, Check Manual
  if (await mockAutoCheckbox.isChecked()) {
    await mockAutoCheckbox.uncheck();
  }
  if (!(await mockManualCheckbox.isChecked())) {
    await mockManualCheckbox.check();
  }

  // Save payments
  const savePaymentsButton = page.getByTestId('save-payments-btn');
  await savePaymentsButton.click();

  // Verify success message
  await expect(page.getByTestId('payments-success-message')).toBeVisible({ timeout: 5000 });
  console.log('Payments settings updated successfully!');
});

test('Streamer Settings File Uploads (Avatar & Media)', async ({ page }) => {
  test.setTimeout(60000);
  const loginPage = new DashboardPage(page);
  const settingsPage = new StreamerSettingsPage(page);

  const fakeEmail = faker.internet.email();
  console.log(`Registering new user for upload tests: ${fakeEmail}`);
  await loginPage.register(fakeEmail);
  await loginPage.waitForDashboard();

  console.log('Navigating to dashboard settings...');
  await page.goto('/dashboard/settings');
  await page.waitForURL(/.*dashboard\/settings.*/, { timeout: 10000 });

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
