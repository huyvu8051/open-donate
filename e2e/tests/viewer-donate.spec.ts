import { test } from '@playwright/test';
import { faker } from '@faker-js/faker';
import { StreamerPage } from '../pages/StreamerPage';
import { ExplorePage } from '../pages/ExplorePage';

test.describe('Viewer Donation Flow', () => {
  test('Viewer can navigate to streamer page, submit a donation, and see it in Recent Tributes', async ({ page }) => {
    const streamerPage = new StreamerPage(page);

    // 1. Navigate directly to a seeded streamer's public page
    await streamerPage.goto('neonviper');

    // Verify page loaded
    // await streamerPage.verifyStreamerName('HuyVu8051');

    // 2. Fill out the donation form
    const donorName = faker.person.fullName();
    const message = 'Keep up the great work! ' + faker.lorem.sentence();
    const amount = '50';
    const paymentMethod = 'Mock Auto';

    await streamerPage.fillDonationForm(donorName, amount, message, paymentMethod);

    // 3. Submit donation
    await streamerPage.submitDonation();

    // 4. Handle Mock Payment Window
    await streamerPage.verifyMockPaymentFlow();

    // 5. Verify it appears in Recent Tributes
    await streamerPage.verifyDonationInTributes(donorName, amount, message);

    console.log(`Donation by ${donorName} processed and verified successfully!`);
  });

  test('Viewer can explore and donate to a random streamer', async ({ page }) => {
    const explorePage = new ExplorePage(page);
    const streamerPage = new StreamerPage(page);

    // 1. Navigate to the explore page
    await explorePage.goto();
    await explorePage.verifyLoaded();

    // 2. Click a random streamer card
    const { displayName } = await explorePage.clickRandomStreamer();

    // 3. Verify navigated to streamer's page
    await streamerPage.verifyStreamerName(displayName);

    // 4. Fill out the donation form
    const donorName = faker.person.fullName();
    const message = 'Found you on explore page! ' + faker.lorem.sentence();
    const amount = '10';
    const paymentMethod = 'Mock Auto';

    await streamerPage.fillDonationForm(donorName, amount, message, paymentMethod);

    // 5. Submit donation
    await streamerPage.submitDonation();

    // 6. Handle Mock Payment Window
    await streamerPage.verifyMockPaymentFlow();

    // 7. Verify it appears in Recent Tributes
    await streamerPage.verifyDonationInTributes(donorName, amount, message);

    console.log(`Donation by ${donorName} to ${displayName} processed successfully!`);
  });
});
