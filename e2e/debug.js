const { chromium } = require('playwright');
(async () => {
  const browser = await chromium.launch();
  const page = await browser.newPage();
  await page.goto('http://localhost:3000/api/login');
  await page.waitForTimeout(2000);
  console.log("URL:", page.url());
  const html = await page.content();
  console.log("HTML:", html.substring(0, 1000));
  await browser.close();
})();
