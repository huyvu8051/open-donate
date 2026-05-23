# Streamer Onboarding Flow (Plan)

## Goal
Onboard a streamer from “new account” → “receiving donations” → “overlay live on stream”, with clear status, minimal friction, and safe payout setup.

## Roles
- **Streamer**: the creator being onboarded.
- **Viewer/Donor**: donates on `/streamer/:username`.
- **Admin/Support** (optional): manual review/KYC escalation.

## Primary States
1. `UNCLAIMED` (reserved username exists, no owner)
2. `CLAIMED` (streamer account created + owns profile)
3. `STREAM_ACCOUNT_CONNECTED` (Twitch/YouTube/Kick OAuth linked)
4. `PAYOUT_PENDING` (payout method started but not complete)
5. `PAYOUT_READY` (payout method verified)
6. `OVERLAY_READY` (overlay URL generated + secret set)
7. `LIVE_READY` (all required checks complete)

## MVP Onboarding Steps (UI)
1. **Create/Sign in**
   - Email/OAuth login.
2. **Claim handle**
   - Choose `username` (or import from streaming platform).
   - Create public profile page at `/streamer/:username`.
3. **Profile setup**
   - Display name, avatar, bio, default donation presets.
4. **Connect streaming platform**
   - OAuth connect (Twitch/YouTube/Kick).
   - Store provider + provider_user_id + verified badge.
5. **Payout setup**
   - Choose payout rail: USDC wallet (address) or bank payout.
   - (Optional) KYC flow if required by provider/region.
6. **Overlay setup**
   - Provide OBS browser-source URL, plus a secret token.
   - Offer “Test alert” button.
7. **Go live checklist**
   - Quick checklist + “All set” confirmation.

## What the Dashboard Should Show
- Onboarding checklist with step-by-step status and CTA buttons.
- Donation page URL + QR download.
- OBS overlay URL + copy-to-clipboard + “Test alert”.
- Payout status + estimated next payout date + withdrawals.
- Recent donations table + filters/export.

## Backend/Data Model Notes (MVP)
- `streamers` table:
  - `id`, `username`, `display_name`, `avatar_url`, `bio`, `is_live`
  - `owner_user_id` (nullable until claimed)
  - `onboarding_state` (enum string)
  - `stream_provider`, `stream_provider_user_id`, `verified_at`
  - `overlay_secret`, `overlay_enabled`
  - payout fields (one-of): `wallet_address` OR `payout_account_id`
- `transactions` table:
  - keep existing fields; optionally add `status`, `payment_ref`, `completed_at`

## APIs Needed (next implementation phase)
- `POST /api/streamers/claim` (claim username)
- `PATCH /api/streamers/:id` (update profile)
- `POST /api/streamers/:id/connect/:provider` (start OAuth)
- `POST /api/streamers/:id/payout` (start/confirm payout)
- `POST /api/streamers/:id/overlay/rotate-secret`
- `POST /api/streamers/:id/test-alert`

## Edge Cases
- Username conflicts + reserved names.
- Reconnect streaming provider (account changed).
- Payout changes require re-verification.
- Overlay secret rotation invalidates old URL.

