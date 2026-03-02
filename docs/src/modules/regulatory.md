# F20 -- Regulatory Monitor

Monitors regulatory changes and translates dense legal and regulatory text into plain-language summaries for campaign staff and voters.

## Key Features

- **Regulatory intake**: Submit regulatory texts for plain-language summarization.
- **Impact assessment**: Evaluate how regulatory changes affect the campaign and constituents.
- **Urgency classification**: Rate regulatory updates by urgency (critical, high, medium, low).
- **Action items**: Identify specific actions required in response to regulatory changes.
- **Brief generation**: Compile multiple updates into a consolidated regulatory brief.
- **Source attribution**: Track the originating agency or body for each update.

## Server Functions

| Function | Endpoint | Description |
|----------|----------|-------------|
| `summarize_regulation` | `regulatory/summarize` | Summarize a regulatory text in plain language |
| `list_updates` | `regulatory/list` | List regulatory updates |
| `get_update` | `regulatory/get` | Retrieve a specific regulatory update |
| `generate_brief` | `regulatory/brief` | Generate a consolidated regulatory brief |

## Update Structure

Each regulatory update includes:

- **Title**: Brief title of the regulation
- **Summary**: Plain-language summary
- **Effective date**: When the regulation takes effect
- **Impact assessment**: How it affects the campaign or constituents
- **Urgency**: critical, high, medium, or low
- **Action required**: Specific steps needed in response
- **Source name**: Originating agency or legislative body

## UI Components

- **Regulatory monitor page** (`/regulatory`): Dashboard of recent regulatory updates with urgency indicators.
- **Update detail**: Full plain-language summary with impact assessment.
- **Brief generator**: Select multiple updates to compile into a consolidated brief.
- **Urgency filter**: Filter updates by urgency level.

## Database Tables

- `regulatory_updates` -- titles, summaries, impact assessments, urgency, effective dates
- `regulatory_briefs` -- compiled briefs covering multiple updates
