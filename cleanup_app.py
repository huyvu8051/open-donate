import os

def delete_lines(filename, ranges):
    with open(filename, 'r') as f:
        lines = f.readlines()
    
    # Sort ranges in reverse order so deleting doesn't affect subsequent indices
    ranges.sort(key=lambda x: x[0], reverse=True)
    
    for start, end in ranges:
        del lines[start-1:end]
        
    with open(filename, 'w') as f:
        f.writelines(lines)

# Ranges to delete (1-indexed, inclusive bounds)
ranges = [
    (13, 22), # Mock structs
    (24, 32), # LeaderboardEntry struct
    (136, 399), # Header, LanguageSwitcher, Footer
    (400, 459), # Hero
    (461, 591), # ExplorePage
    (593, 706), # LeaderboardPage
    (708, 960), # LandingPage
    (962, 1647), # StreamerPage and sub-components
    (1661, 1696), # GetStreamer
    (1972, 2005), # GetAllStreamers
    (2007, 2059), # CreateDonation
    (2061, 2120), # CreateMockPayment
    (2122, 2139), # AcceptMockPayment
    (2141, 2158), # RejectMockPayment
    (2160, 2182), # GetMockPaymentStatus
    (2184, 2217), # GetRecentTransactions
    (2263, 2299), # GetStreamerLeaderboard
]

delete_lines('src/app.rs', ranges)

# We need to add the imports at the top of app.rs
with open('src/app.rs', 'r') as f:
    content = f.read()

imports = """
use crate::pages::landing::LandingPage;
use crate::pages::explore::ExplorePage;
use crate::pages::leaderboard::LeaderboardPage;
use crate::pages::streamer::StreamerPage;
"""
# insert imports after line 12
lines = content.split('\n')
lines.insert(12, imports)

with open('src/app.rs', 'w') as f:
    f.write('\n'.join(lines))

print("app.rs cleaned up")
