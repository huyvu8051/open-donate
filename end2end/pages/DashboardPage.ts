import { expect, type Locator, type Page } from '@playwright/test';

export class DashboardPage {
  readonly page: Page;
  readonly getStartedHeader: Locator;
  readonly quickLinksContainer: Locator;

  constructor(page: Page) {
    this.page = page;
    this.getStartedHeader = page.locator('text=Streamer Dashboard');
    this.quickLinksContainer = page.locator('.glass-card').first();
  }

  async goto() {
    await this.page.goto('/dashboard');
  }

  async setMockAuthCookie(userId: string, name: string, email: string) {
    // We mock the auth_token by creating a dummy JWT format or simply expecting the backend 
    // to handle testing logic if a secret cookie is passed. 
    // Since our backend uses Zitadel JWT, mocking it in a real E2E requires a valid signed token 
    // unless the backend is in a test mode that accepts fake tokens.
    // For this test plan, we assume the backend has a backdoor in DEV mode to accept mock tokens,
    // or we are just setting a cookie the backend can decode (e.g., using a test JWT secret).
    // In a real project, we would use a library like jsonwebtoken here to sign a fake token
    // with the known ZITADEL_ISSUER and secret if it's asymmetric/symmetric.
    
    // For this example, we'll set a dummy token structure that would pass if the backend allowed it,
    // or we leave a placeholder comment. The E2E plan was to "Mock auth cookie".
    // In Leptos, the cookie name is `auth_token`.
    await this.page.context().addCookies([
      {
        name: 'auth_token',
        value: `mock_jwt_for_${userId}_${email}`, // In real life, sign a proper JWT here
        domain: 'localhost',
        path: '/',
      }
    ]);
  }
}
