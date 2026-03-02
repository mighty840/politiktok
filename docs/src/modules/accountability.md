# F08 -- Manifesto Accountability Engine

Tracks elected officials' actions against their campaign promises and manifesto commitments, generating accountability reports.

## Key Features

- **Commitment extraction**: LLM-powered extraction of specific commitments from manifesto text and speeches.
- **Evidence tracking**: Attach evidence (news articles, voting records, policy actions) to commitments.
- **Evidence classification**: Automated classification of evidence as fulfilled, broken, partial, or unrelated using LLM analysis.
- **Confidence scoring**: Each evidence classification includes a confidence score.
- **Status tracking**: Commitments are tracked as pending, fulfilled, broken, or partially fulfilled.
- **Accountability reports**: Generate summary reports of promise fulfillment rates.

## Server Functions

| Function | Endpoint | Description |
|----------|----------|-------------|
| `extract_commitments` | `accountability/extract` | Extract commitments from manifesto text |
| `list_commitments` | `accountability/list` | List all tracked commitments |
| `get_commitment` | `accountability/get` | Fetch a commitment with evidence |
| `add_evidence` | `accountability/add-evidence` | Attach evidence to a commitment |
| `classify_evidence` | `accountability/classify` | LLM-classify evidence against a commitment |
| `generate_report` | `accountability/report` | Generate an accountability summary report |

## Commitment Structure

Each extracted commitment includes:

- **Text**: The specific promise made
- **Topic**: Policy area (healthcare, economy, education, etc.)
- **Strength**: How definitive the promise was (firm, conditional, aspirational)
- **Date**: When the commitment was made
- **Status**: Current fulfillment status

## Evidence Classification

Evidence is classified against commitments:

| Classification | Meaning |
|---------------|---------|
| `fulfilled` | The commitment has been met |
| `broken` | The commitment has been contradicted |
| `partial` | Some progress but incomplete fulfillment |
| `unrelated` | The evidence is not relevant to this commitment |

## UI Components

- **Accountability dashboard** (`/accountability`): Overview of commitment fulfillment rates with status breakdowns.
- **Commitment list**: Filterable table of all tracked commitments with status badges.
- **Commitment detail**: Full view with evidence timeline and classification details.
- **Report generator**: Produce formatted accountability reports.

## Database Tables

- `commitments` -- extracted promises with topic, strength, status, evidence count
- `evidence` -- evidence records with classification, confidence, and commitment linkage
