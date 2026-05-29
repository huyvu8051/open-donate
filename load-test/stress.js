import http from 'k6/http';
import { check, sleep } from 'k6';

export const options = {
    stages: [
        { duration: '5s', target: 10 },  // Ramp up to 10 VUs
        { duration: '15s', target: 30 }, // Ramp up to 30 VUs
        { duration: '5s', target: 0 },   // Ramp down to 0 VUs
    ],
    thresholds: {
        http_req_duration: ['p(95)<500'], // 95% of requests must complete below 500ms
        http_req_failed: ['rate<0.01'],   // Error rate must be less than 1%
    },
};

const BASE_URL = __ENV.BASE_URL || 'http://localhost:3000';

export default function () {
    // 1. Visit the Homepage (SSR / Static Files)
    const resHome = http.get(`${BASE_URL}/`);
    check(resHome, {
        'homepage status is 200': (r) => r.status === 200,
    });
    sleep(1);

    // 2. Fetch list of all streamers
    const resAllStreamers = http.post(
        `${BASE_URL}/api/GetAllStreamers`,
        JSON.stringify([]),
        {
            headers: {
                'Content-Type': 'application/json',
                'Accept': 'application/json'
            }
        }
    );
    check(resAllStreamers, {
        'GetAllStreamers status is 200': (r) => r.status === 200,
    });
    sleep(1);

    // 3. Get specific streamer profile details (e.g., username "huyvu8051")
    const resStreamer = http.post(
        `${BASE_URL}/api/GetStreamer`,
        JSON.stringify({ username: 'huyvu8051' }),
        {
            headers: {
                'Content-Type': 'application/json',
                'Accept': 'application/json'
            }
        }
    );
    check(resStreamer, {
        'GetStreamer status is 200': (r) => r.status === 200,
    });
    sleep(1);

    // 4. Simulate a donation submission (CreateMockPayment)
    const payloadDonation = JSON.stringify({
        streamer_id: 1,
        donor_name: 'k6 Load Tester',
        amount: parseFloat((Math.random() * 95 + 5).toFixed(2)), // Random amount between $5 and $100
        message: 'Load testing message from k6 virtual user!',
        payment_method: 'Mock Auto'
    });
    const resDonation = http.post(
        `${BASE_URL}/api/CreateMockPayment`,
        payloadDonation,
        {
            headers: {
                'Content-Type': 'application/json',
                'Accept': 'application/json'
            }
        }
    );
    check(resDonation, {
        'CreateMockPayment status is 200': (r) => r.status === 200,
    });
    sleep(1);

    // 5. Query Streamer Analytics (Simulating Dashboard visits)
    const resAnalytics = http.post(
        `${BASE_URL}/api/GetStreamerAnalytics`,
        JSON.stringify({ streamer_id: 1, time_range: 'month' }),
        {
            headers: {
                'Content-Type': 'application/json',
                'Accept': 'application/json'
            }
        }
    );
    check(resAnalytics, {
        'GetStreamerAnalytics status is 200': (r) => r.status === 200,
    });
    sleep(1);
}
