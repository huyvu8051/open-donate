import { expect, Locator, Page } from '@playwright/test';

export class ExplorePage {
  readonly page: Page;
  readonly streamerCards: Locator;

  constructor(page: Page) {
    this.page = page;
    this.streamerCards = page.getByTestId('streamer-card');
  }

  async goto() {
    await this.page.goto('/explore');
  }

  async verifyLoaded() {
    await expect(this.page.getByRole('heading', { name: 'Explore Creators' })).toBeVisible();
  }

  async clickRandomStreamer(): Promise<{ username: string, displayName: string }> {
    await this.streamerCards.first().waitFor({ state: 'visible' });
    
    const count = await this.streamerCards.count();
    if (count === 0) {
      throw new Error('No streamers found on the explore page!');
    }

    const randomIndex = Math.floor(Math.random() * count);
    const randomCard = this.streamerCards.nth(randomIndex);
    
    const displayName = await randomCard.getByTestId('streamer-display-name').innerText();
    const rawUsername = await randomCard.getByTestId('streamer-username').innerText();
    const username = rawUsername.replace('@', '').trim();

    await randomCard.click();

    return { username, displayName };
  }
}
