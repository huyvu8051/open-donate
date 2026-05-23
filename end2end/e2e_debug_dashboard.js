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
    
    await Promise.all([
        page.waitForNavigation(),
        page.click('button[type="submit"], input[type="submit"]')
    ]);
    
    console.log("URL after submit:", page.url());
    
    // Wait a bit to see if there's another redirect
    await page.waitForTimeout(3000);
    console.log("Final URL:", page.url());
    
    if (page.url().includes('dashboard')) {
        await page.waitForTimeout(3000);
        console.log("--- Dashboard Body ---");
        const body = await page.evaluate(() => document.body.innerHTML);
        console.log(body);
    } else {
        console.log("--- Page Body ---");
        const body = await page.evaluate(() => document.body.innerHTML);
        console.log(body);
    }
    
  } catch (e) {
    console.error(e);
  } finally {
    await browser.close();
  }
})();
