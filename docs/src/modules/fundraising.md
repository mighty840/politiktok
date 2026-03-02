# F07 -- Fundraising Assistant

Optimizes fundraising strategy by analyzing donor patterns, suggesting outreach timing, and generating personalized solicitation content.

## Key Features

- **Donor management**: Full CRUD for donor profiles including contact information, donation history, and engagement scoring.
- **Donation recording**: Track individual donations with amounts, dates, and campaign attribution.
- **Engagement scoring**: Automatic scoring based on donation frequency, recency, and total amount.
- **Fundraising summaries**: Aggregate statistics including total raised, average donation, and top donor rankings.
- **Solicitation drafting**: LLM-generated personalized fundraising emails based on donor profile and giving history.
- **Outreach timing**: Suggestions for optimal contact timing based on donor engagement patterns.

## Server Functions

| Function | Endpoint | Description |
|----------|----------|-------------|
| `create_donor` | `fundraising/create-donor` | Create a new donor profile |
| `update_donor` | `fundraising/update-donor` | Update donor information |
| `list_donors` | `fundraising/list-donors` | List donors with search and filter |
| `get_donor` | `fundraising/get-donor` | Fetch a single donor profile |
| `record_donation` | `fundraising/record-donation` | Record a new donation |
| `get_fundraising_summary` | `fundraising/summary` | Aggregate fundraising statistics |
| `draft_solicitation` | `fundraising/draft-solicitation` | Generate a personalized solicitation email |

## Fundraising Summary

The summary endpoint returns:

- **Total donors**: Count of all donor records
- **Total raised**: Sum of all recorded donations
- **Average donation**: Mean donation amount
- **Top donors**: Ranked list of highest-value donors with total amounts

## UI Components

- **Fundraising dashboard** (`/fundraising`): Overview with key metrics, recent donations, and top donor table.
- **Donor list**: Searchable table with engagement score indicators.
- **Donor detail**: Full profile with donation timeline and solicitation drafting.
- **Donation form**: Quick-entry form for recording new donations.

## Database Tables

- `donors` -- donor profiles with contact info, engagement score, tags
- `donations` -- individual donation records with amount, date, campaign attribution
