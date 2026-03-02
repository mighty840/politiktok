# F10 -- Narrative Contagion Model

Models how political narratives spread through networks, predicting viral potential and identifying key amplification vectors.

## Key Features

- **Narrative extraction**: Identify embedded narratives, framing techniques, and emotional triggers within political text.
- **Virality prediction**: Score each narrative's potential for viral spread (0.0 to 1.0).
- **Audience targeting analysis**: Determine which audience segments a narrative is designed to reach.
- **Spread prediction**: Model how a narrative would propagate through media and social networks.
- **Response recommendations**: Generate strategic responses to counter or amplify identified narratives.
- **Emotional trigger mapping**: Identify the psychological mechanisms driving narrative engagement.

## Server Functions

| Function | Endpoint | Description |
|----------|----------|-------------|
| `analyze_narrative` | `narrative/analyze` | Extract and analyze narratives from text |
| `list_analyses` | `narrative/list` | List past narrative analyses |
| `get_analysis` | `narrative/get` | Retrieve a specific analysis |

## Analysis Output

Each narrative identified in the text includes:

- **Theme**: The core narrative theme
- **Framing**: How the narrative frames the issue
- **Target audience**: Intended demographic segment
- **Virality score**: Predicted viral potential (0.0--1.0)
- **Emotional triggers**: Psychological drivers (fear, hope, anger, pride, etc.)

The overall analysis includes:

- **Spread prediction**: Description of how the narratives would propagate
- **Recommended responses**: Strategic actions the campaign should take

## UI Components

- **Narrative analyzer** (`/narrative`): Text submission form with analysis results.
- **Narrative cards**: Visual display of each identified narrative with virality gauges and emotional trigger tags.
- **Spread visualization**: Network-style diagram showing predicted propagation paths.
- **Response panel**: Actionable response recommendations.

## Database Tables

- `narrative_analyses` -- input text, extracted narratives (jsonb), spread predictions, recommendations
