import { test } from '@playwright/test';
import { faker } from '@faker-js/faker';
import { KeycloakLoginPage } from './pages/KeycloakLoginPage';
import { StreamerSettingsPage } from './pages/StreamerSettingsPage';

test('Streamer Settings Update Flow (POM & Test IDs)', async ({ page }) => {
  const loginPage = new KeycloakLoginPage(page);
  const settingsPage = new StreamerSettingsPage(page);

  // 1. Authenticate / Auto-onboard via Keycloak
  console.log('Navigating to app login endpoint...');
  await loginPage.gotoAppLogin();

  const fakeEmail = faker.internet.email();
  console.log(`Registering new user: ${fakeEmail}`);
  await loginPage.registerNewUser(fakeEmail);

  // 2. Navigate to Settings Page
  console.log('Navigating to dashboard settings...');
  await settingsPage.goto();

  // 3. Update settings form
  const newDisplayName = faker.person.fullName();
  const newBio = faker.lorem.sentence();
  const newUsername = faker.string.alphanumeric({ length: 8, casing: 'lower' });

  console.log(`Updating settings - Name: ${newDisplayName}, Username: ${newUsername}`);
  await settingsPage.updateSettings(newDisplayName, newBio, newUsername);

  // 4. Verify success message appeared
  await settingsPage.verifySuccessMessage();
  
  // 5. Verify the inputs retained their new values
  await settingsPage.verifySettingsValues(newDisplayName, newBio, newUsername);

  console.log('Settings updated successfully test passed!');
});
