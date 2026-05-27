# Open Donate Load Testing with k6

This directory contains stress testing scripts using [Grafana k6](https://k6.io/).

## Prerequisites

You need `k6` installed on your machine.
- macOS (Homebrew): `brew install k6`
- Linux (Debian/Ubuntu): `sudo apt install k6`
- Docker: `docker pull grafana/k6`

## Running the Stress Test

Before running the test, make sure your application is running (e.g. `cargo leptos run --release`).

To run the basic stress test script:

```bash
# Run locally
k6 run stress.js

# Or if you are running the app on a different port/URL
k6 run -e BASE_URL=http://localhost:8080 stress.js

# Or using Docker
docker run --rm -i --network host grafana/k6 run - < stress.js
```

## Customizing the Test

You can modify `stress.js` to change the:
- **Stages**: Adjust `target` (number of Virtual Users) and `duration` to simulate different load patterns (e.g., spikes, steady load, soak test).
- **Thresholds**: Update the p(95) duration requirements or acceptable error rates.
- **Scenarios**: Add more realistic user flows like logging in, viewing overlays, or sending donations.
