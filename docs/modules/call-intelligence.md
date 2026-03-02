# F13 -- Call Intelligence

Analyzes constituent call data to extract trends, common concerns, and sentiment, providing actionable intelligence for representatives and campaign staff.

## Key Features

- **Transcript analysis**: LLM-powered analysis of constituent call transcripts.
- **Summary generation**: Automatic concise summaries of call content.
- **Sentiment classification**: Determine caller sentiment (positive, negative, neutral).
- **Issue extraction**: Identify key issues raised during the call.
- **Action item detection**: Extract specific follow-up actions from the conversation.
- **Satisfaction scoring**: Estimate caller satisfaction on a 0.0--1.0 scale.
- **Trend aggregation**: Track common issues and sentiment trends across many calls.

## Server Functions

| Function | Endpoint | Description |
|----------|----------|-------------|
| `analyze_call` | `call-intel/analyze` | Analyze a constituent call transcript |
| `list_analyses` | `call-intel/list` | List past call analyses |
| `get_analysis` | `call-intel/get` | Retrieve a specific call analysis |
| `get_call_trends` | `call-intel/trends` | Aggregate issue and sentiment trends |

## Analysis Output

Each call analysis produces:

- **Summary**: A brief paragraph capturing the key points
- **Sentiment**: positive, negative, or neutral
- **Key issues**: List of specific issues raised
- **Action items**: Tasks that need follow-up
- **Caller satisfaction**: Estimated satisfaction score (0.0--1.0)

## UI Components

- **Call intelligence dashboard** (`/call-intel`): Overview of recent call analyses with sentiment breakdown and trending issues.
- **Call detail view**: Full analysis of a single call with summary, issues, and action items.
- **Trend charts**: Visual representation of issue frequency and sentiment trends over time.
- **Action item tracker**: Filterable list of extracted action items across calls.

## Database Tables

- `call_analyses` -- transcripts with summaries, sentiment, issues, action items, satisfaction scores
