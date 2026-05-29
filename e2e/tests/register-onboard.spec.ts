import { test, expect } from '@playwright/test';
import { faker } from '@faker-js/faker';

test('Firebase Auth Validation, Registration and Logout Flow', async ({ page }) => {
  // 1. Trigger registration from the app
  console.log('Navigating to app register endpoint...');
  await page.goto('/register');

  const fakeEmail = faker.internet.email();
  console.log(`Registering new user: ${fakeEmail}`);

  await page.fill('input[name="email"]', fakeEmail);

  // 2. Test Validation (Mismatching passwords)
  const password = '@Aa123456';
  await page.fill('input[name="password"]', password);
  await page.fill('input[name="password_confirm"]', 'differentpassword');
  await page.click('button[type="submit"]');

  // Verify error message
  await expect(page.locator('.bg-error-container')).toBeVisible({ timeout: 5000 });

  // 3. Fix passwords and Register
  await page.fill('input[name="password_confirm"]', password);
  await page.click('button[type="submit"]');

  // 4. Verification of login
  console.log('Waiting for redirect back to app dashboard...');
  await page.waitForURL(/\/dashboard/, { timeout: 15000 });

  // The dashboard should load and display the main header
  await expect(page.getByTestId('streamer-dashboard-header')).toBeVisible({ timeout: 10000 });
  console.log('Test completed successfully! User was authenticated via Firebase REST API.');

  // 5. Test Logout
  console.log('Testing logout flow...');
  // Click logout button (since it's in the header, might be hidden in mobile menu, but playwright uses desktop safari which is usually wide enough, let's just click the button with text "Logout")
  await page.getByTestId('logout-button').click();

  // Wait for redirect to home
  await page.waitForURL('/', { timeout: 10000 });

  // Verify user is redirected to landing page and logout button is gone
  await expect(page).toHaveURL('/');
  await expect(page.getByTestId('logout-button')).not.toBeVisible();
  console.log('Logout successful!');
});
