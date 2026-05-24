import { test, expect } from '@playwright/test';
import { faker } from '@faker-js/faker';

test('Keycloak Registration and Auto-Onboarding Flow', async ({ page }) => {
  // 1. Trigger login redirect from the app
  console.log('Navigating to app login endpoint...');
  await page.goto('http://localhost:3000/api/login');

  // Wait for redirect to Keycloak
  await page.waitForURL(/.*localhost:8080.*/);
  console.log('Redirected to Keycloak.');

  // 2. On Keycloak login page, find and click "Register"
  const registerButton = page.locator('text="Register"').first();
  await expect(registerButton).toBeVisible({ timeout: 5000 });
  await registerButton.click();

  // Wait for registration form
  await page.waitForURL(/.*\/login-actions\/registration.*/);

  // 3. Fill in registration details
  // Keycloak registration form with email as username and disabled First/Last name
  const fakeEmail = faker.internet.email();

  console.log(`Registering new user: ${fakeEmail}`);

  await page.fill('input[name="email"], input[type="email"]', fakeEmail);

  // If First/Last name fields happen to exist (fallback), fill them
  const firstNameInput = page.locator('input[name="firstName"]');
  if (await firstNameInput.count() > 0) {
    await firstNameInput.fill(faker.person.firstName());
  }
  const lastNameInput = page.locator('input[name="lastName"]');
  if (await lastNameInput.count() > 0) {
    await lastNameInput.fill(faker.person.lastName());
  }

  // 4. Set Password
  const password = '@Aa123456';
  const passwordInput = page.locator('input[type="password"], input[name="password"]').first();
  await expect(passwordInput).toBeVisible({ timeout: 5000 });
  await passwordInput.fill(password);
  
  // Find password confirmation
  const passwordConfirm = page.locator('input[name="password-confirm"]');
  if (await passwordConfirm.count() > 0) {
    await passwordConfirm.fill(password);
  }

  // Click Register/Submit
  await page.click('button[type="submit"], input[type="submit"]');

  // 5. Verification
  // The user should be immediately authenticated (no email verify) and redirected back.
  console.log('Waiting for redirect back to app dashboard...');
  
  await page.waitForURL(/.*localhost:3000.*/, { timeout: 15000 });

  // Navigate to dashboard
  await page.goto('http://localhost:3000/dashboard');

  // 6. Verify Dashboard and Auto-Onboarding
  const expectedPrefix = fakeEmail.split('@')[0].toLowerCase();
  console.log(`Expected username prefix: ${expectedPrefix}`);

  // The dashboard should load
  await expect(page.getByTestId('streamer-dashboard-header')).toBeVisible({ timeout: 10000 });

  console.log('Test completed successfully! User was auto-onboarded via Keycloak.');
});
