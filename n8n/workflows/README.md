# GoldHunter — n8n Workflows

This directory contains exported n8n workflow JSON files for GoldHunter automation.

## Planned Workflows

| Workflow | Description | Status |
|----------|-------------|--------|
| `poi-sync.json` | Sync POI data from external sources via Scrapling | Pending |
| `brand-onboard.json` | Automate B2B brand onboarding flow | Pending |
| `cpa-tracker.json` | Track and report cost-per-action for each brand redemption | Pending |
| `album-expiry.json` | Notify users when limited edition album slots are running low | Pending |
| `guardian-feedback.json` | Collect and log Guardian AI interaction quality metrics | Pending |

## Setup

Import any `.json` file from this directory into your n8n instance via:
**Workflows → Import from file**

## Environment Variables

Set these in your n8n instance:
- `ANTHROPIC_API_KEY` — Claude API key for Guardian AI calls
- `GOLDHUNTER_API_URL` — Backend API base URL
- `SOLANA_RPC_URL` — Solana RPC endpoint (Devnet or Mainnet)
