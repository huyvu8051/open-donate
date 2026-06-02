import os

def slice_file(filename, start, end):
    with open(filename, 'r') as f:
        lines = f.readlines()
    return "".join(lines[start-1:end])

app_rs = 'src/app.rs'

# Extract LandingPage and Hero
landing_content = "use leptos::prelude::*;\nuse crate::components::layout::{Header, Footer};\n\n"
landing_content += slice_file(app_rs, 400, 459)
landing_content += "\n" + slice_file(app_rs, 708, 960)

with open('src/pages/landing.rs', 'w') as f:
    f.write(landing_content)

# Extract ExplorePage
explore_content = "use leptos::prelude::*;\nuse crate::components::layout::{Header, Footer};\nuse crate::db::DbStreamer;\n\n"
explore_content += slice_file(app_rs, 461, 591)
# Add GetAllStreamers server function
explore_content += "\n" + slice_file(app_rs, 1972, 2005)
with open('src/pages/explore.rs', 'w') as f:
    f.write(explore_content)

# Extract LeaderboardPage
leaderboard_content = "use leptos::prelude::*;\nuse crate::components::layout::{Header, Footer};\nuse serde::{Deserialize, Serialize};\n\n"
leaderboard_content += slice_file(app_rs, 24, 32) # LeaderboardEntry struct
leaderboard_content += "\n" + slice_file(app_rs, 593, 706)
leaderboard_content += "\n" + slice_file(app_rs, 2263, 2299) # GetStreamerLeaderboard
with open('src/pages/leaderboard.rs', 'w') as f:
    f.write(leaderboard_content)

# Extract StreamerPage and sub-components and structs and server fns
streamer_content = "use leptos::prelude::*;\nuse crate::components::layout::{Header, Footer};\nuse crate::db::{TransactionStatus, PaymentMethod, DbStreamer, DbTransaction};\nuse serde::{Deserialize, Serialize};\n\n"
streamer_content += slice_file(app_rs, 13, 22) # Mock structs
streamer_content += "\n" + slice_file(app_rs, 34, 45) # Analytics structs (wait, analytics is in dashboard, maybe we just leave it in app.rs for now or move it? Let's leave it in app.rs and move it later. No, let's keep it in app.rs for now, we won't extract it to streamer.rs)
streamer_content += "\n" + slice_file(app_rs, 962, 1647) # StreamerPage and sub-components
# Server functions for streamer page
streamer_content += "\n" + slice_file(app_rs, 1661, 1696) # GetStreamer
streamer_content += "\n" + slice_file(app_rs, 2007, 2059) # CreateDonation
streamer_content += "\n" + slice_file(app_rs, 2061, 2120) # CreateMockPayment
streamer_content += "\n" + slice_file(app_rs, 2122, 2139) # AcceptMockPayment
streamer_content += "\n" + slice_file(app_rs, 2141, 2158) # RejectMockPayment
streamer_content += "\n" + slice_file(app_rs, 2160, 2182) # GetMockPaymentStatus
streamer_content += "\n" + slice_file(app_rs, 2184, 2217) # GetRecentTransactions

with open('src/pages/streamer.rs', 'w') as f:
    f.write(streamer_content)

print("Files extracted successfully.")
