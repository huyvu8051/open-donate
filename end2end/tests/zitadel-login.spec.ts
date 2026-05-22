import { test, expect } from "@playwright/test";
import * as path from "path";

test("Zitadel Console Login Flow", async ({ page }) => {
  // We want to save screenshots in the conversation artifacts directory
  const artifactDir = "/Users/huyvu8051/.gemini/antigravity-ide/brain/4f6352ae-77e6-49aa-aa7d-8dd665dbc833";
  
  console.log("Navigating to http://localhost:8080/ui/console/ ...");
  await page.goto("http://localhost:8080/ui/console/");

  // Wait for the page to load/redirect
  await page.waitForTimeout(3000);
  
  // Capture initial screenshot
  await page.screenshot({ path: path.join(artifactDir, "step1_initial.png") });
  console.log("Captured initial page screenshot.");

  // Check if we are on the login page.
  // Zitadel's login form has a username input. Usually it's an input with type="text" or name="username" or label.
  // Let's print out all inputs to be sure.
  const inputs = await page.$$eval("input", els => els.map(el => ({
    type: el.type,
    name: el.name,
    placeholder: el.placeholder,
    id: el.id,
    value: el.value
  })));
  console.log("Available inputs on page:", inputs);

  // Locate the username input. In Zitadel V1 login, it's typically:
  // name="loginName" or id="username" or type="text"
  const usernameInput = page.locator('input[name="loginName"], input[type="text"], input[autocomplete="username"]');
  if (await usernameInput.count() > 0) {
    console.log("Filling username...");
    await usernameInput.fill("root@zitadel.localhost");
    await page.screenshot({ path: path.join(artifactDir, "step2_username_filled.png") });
    
    // Submit username page (usually click a next/submit button)
    // Let's find button/submit
    const nextButton = page.locator('button[type="submit"], button:has-text("Next"), button:has-text("Login")');
    console.log("Clicking next button...");
    await nextButton.click();
    
    await page.waitForTimeout(3000);
    await page.screenshot({ path: path.join(artifactDir, "step3_password_page.png") });

    // Fill password
    const passwordInput = page.locator('input[name="password"], input[type="password"]');
    if (await passwordInput.count() > 0) {
      console.log("Filling password...");
      await passwordInput.fill("RootPassword1!");
      await page.screenshot({ path: path.join(artifactDir, "step4_password_filled.png") });
      
      const submitButton = page.locator('button[type="submit"], button:has-text("Login"), button:has-text("Sign In")');
      console.log("Clicking submit button...");
      await submitButton.click();
      
      await page.waitForTimeout(5000);
      await page.screenshot({ path: path.join(artifactDir, "step5_after_login.png") });
      
      console.log("Current URL after login:", page.url());
    } else {
      console.log("Password input not found!");
    }
  } else {
    console.log("Username input not found!");
  }
});
