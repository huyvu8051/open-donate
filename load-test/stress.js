import http from 'k6/http';
import { check, sleep } from 'k6';

export const options = {
    stages: [
        { duration: '10s', target: 50 }, // Ramp up to 50 users over 10s
        { duration: '30s', target: 50 }, // Stay at 50 users for 30s
        { duration: '10s', target: 0 },  // Ramp down to 0 users over 10s
    ],
    thresholds: {
        http_req_duration: ['p(95)<500'], // 95% of requests must complete below 500ms
        http_req_failed: ['rate<0.01'],   // Error rate must be less than 1%
    },
};

const BASE_URL = __ENV.BASE_URL || 'http://localhost:3000';

export default function () {
    // 1. Visit the Homepage
    const resHome = http.get(`${BASE_URL}/`);
    check(resHome, {
        'homepage status is 200': (r) => r.status === 200,
        'homepage has body': (r) => r.body.length > 0,
    });
    
    // Simulate user reading the page
    sleep(Math.random() * 2 + 1);

    // 2. Fetch all streamers API (often called by the frontend)
    // In Leptos, server functions are POST requests to /api/FunctionName
    // Let's test the GetAllStreamers endpoint directly if needed,
    // Or we can just load a streamer page. We will test SSR performance.
    
    // For now, let's just make another request to a non-existent streamer to test 404/SSR fallback
    // Or we can query the API directly
    const resStreamers = http.post(`${BASE_URL}/api/GetAllStreamers`, null, {
        headers: {
            'Content-Type': 'application/x-www-form-urlencoded',
            'Accept': 'application/json'
        }
    });

    check(resStreamers, {
        'get streamers status is 200': (r) => r.status === 200,
    });

    sleep(1);
}
