const { chromium } = require('playwright');
const { faker } = require('@faker-js/faker');

(async () => {
  const browser = await chromium.launch({ headless: true });
  const context = await browser.newContext();
  const page = await context.newPage();
  
  try {
    console.log("Navigating to login...");
    await page.goto('http://127.0.0.1:3000/api/login');
    await page.waitForURL(/.*localhost:8080.*/);
    
    console.log("Clicking Register...");
    await page.locator('text="Register"').first().click();
    await page.waitForURL(/.*\/login-actions\/registration.*/);
    
    const fakeEmail = faker.internet.email();
    console.log(`Registering: ${fakeEmail}`);
    await page.fill('input[name="email"], input[type="email"]', fakeEmail);
    
    if (await page.locator('input[name="firstName"]').count() > 0) {
      await page.fill('input[name="firstName"]', faker.person.firstName());
    }
    if (await page.locator('input[name="lastName"]').count() > 0) {
      await page.fill('input[name="lastName"]', faker.person.lastName());
    }
    
    const password = '@Aa123456';
    await page.fill('input[type="password"], input[name="password"]', password);
    if (await page.locator('input[name="password-confirm"]').count() > 0) {
      await page.fill('input[name="password-confirm"]', password);
    }
    
    await page.click('button[type="submit"], input[type="submit"]');
    
    console.log("Waiting for dashboard redirect...");
    await page.waitForURL('http://127.0.0.1:3000/dashboard', { timeout: 15000 });
    console.log("On dashboard!");
    
    // Wait for the history table or buttons to load
    await page.waitForSelector('button:has-text("chevron_left")', { timeout: 5000 });
    
    const prevBtn = await page.locator('button:has-text("chevron_left")').first();
    const nextBtn = await page.locator('button:has-text("chevron_right")').first();
    
    const isPrevDisabled = await prevBtn.isDisabled();
    const isNextDisabled = await nextBtn.isDisabled();
    
    console.log("Previous Button Disabled?", isPrevDisabled);
    console.log("Next Button Disabled?", isNextDisabled);
    
  } catch (e) {
    console.error(e);
  } finally {
    await browser.close();
  }
})();
