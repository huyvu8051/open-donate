import { test, expect } from '@playwright/test';
import { faker } from '@faker-js/faker';
import { DashboardPage } from '../pages/DashboardPage';
import { RegisterPage } from '../pages/RegisterPage';

test('Auth Validation, Registration and Logout Flow', async ({ page }) => {
  const dashboardPage = new DashboardPage(page);
  const registerPage = new RegisterPage(page);

  // 1. Trigger registration from the app
  console.log('Navigating to app register endpoint...');
  await expect(async () => {
    await page.goto('/register');

    const fakeEmail = faker.internet.email();
    console.log(`Registering new user: ${fakeEmail}`);

    await registerPage.emailInput.fill(fakeEmail);

    // 2. Test Validation (Mismatching passwords)
    const password = '@Aa123456';
    await registerPage.passwordInput.fill(password);
    await registerPage.passwordConfirmInput.fill('differentpassword');
    await registerPage.submitBtn.click();

    // Verify error message
    await expect(registerPage.errorContainer).toBeVisible({ timeout: 5000 });

    // 3. Fix passwords and Register using robust RegisterPage method
    console.log('Executing successful registration flow...');
    await registerPage.register(fakeEmail);

    // 4. Verification of login
    console.log('Redirected to app dashboard!');
    await dashboardPage.waitForDashboard();
  }).toPass({ timeout: 90000 });
  console.log('Test completed successfully! User was authenticated via Leptos backend.');

  // 5. Test Logout
  console.log('Testing logout flow...');
  // Click logout button (since it's in the header, might be hidden in mobile menu, but playwright uses desktop safari which is usually wide enough, let's just click the button with text "Logout")
  await dashboardPage.logout();
  console.log('Logout successful!');
});
