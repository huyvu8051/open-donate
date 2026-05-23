const { chromium } = require('playwright');

(async () => {
  const browser = await chromium.launch({ headless: true });
  const context = await browser.newContext();
  const page = await context.newPage();
  
  // Login flow
  await page.goto('http://127.0.0.1:3000');
  
  try {
    // wait for redirect or dashboard
    await page.waitForTimeout(2000);
    const content = await page.content();
    if (!content.includes('Glint')) {
        console.log("Not logged in, going to dashboard...");
        await page.goto('http://127.0.0.1:3000/dashboard');
        await page.waitForTimeout(3000);
    }
    
    console.log("--- Dashboard Content ---");
    const dashboardHtml = await page.evaluate(() => document.body.innerHTML);
    console.log(dashboardHtml.substring(0, 1000) + "...");
    
    // Check buttons
    const buttons = await page.$$eval('button', btns => btns.map(b => ({
      text: b.innerText,
      disabled: b.disabled,
      className: b.className
    })));
    console.log("Buttons:", buttons);
  } catch (e) {
    console.error(e);
  } finally {
    await browser.close();
  }
})();
