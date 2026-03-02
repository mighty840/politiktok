# F22 -- Disinformation Warning

Detects emerging disinformation campaigns targeting candidates or policies, enabling rapid response before false narratives take hold.

## Key Features

- **Disinformation detection**: Analyze content for indicators of coordinated disinformation.
- **Risk level assessment**: Rate content as low, medium, high, or critical risk.
- **Indicator identification**: Detect specific disinformation markers (astroturfing, bot-like patterns, narrative manipulation, source fabrication).
- **Confidence scoring**: Each indicator includes a confidence level.
- **Response recommendations**: Generate strategic responses to counter identified disinformation.
- **Early warning**: Surface emerging disinformation before it goes viral.

## Server Functions

| Function | Endpoint | Description |
|----------|----------|-------------|
| `analyze_disinfo` | `disinfo/analyze` | Analyze content for disinformation indicators |
| `list_analyses` | `disinfo/list` | List past disinformation analyses |
| `get_analysis` | `disinfo/get` | Retrieve a specific analysis |

## Indicator Types

The detection system identifies several categories of disinformation markers:

| Indicator | Description |
|-----------|-------------|
| `astroturfing` | Fake grassroots activity |
| `bot_activity` | Automated account behavior patterns |
| `narrative_manipulation` | Deliberate framing distortions |
| `source_fabrication` | Fake or misattributed sources |
| `emotional_exploitation` | Manufactured outrage or fear |
| `context_stripping` | Facts presented without critical context |

## Analysis Output

Each analysis includes:

- **Content**: The analyzed text or social media content
- **Risk level**: low, medium, high, or critical
- **Indicators**: List of detected disinformation markers with descriptions and confidence scores
- **Recommended response**: Strategic guidance for countering the disinformation

## UI Components

- **Disinfo warning page** (`/disinfo`): Content submission form with risk assessment results.
- **Risk gauge**: Visual indicator of disinformation risk level.
- **Indicator cards**: Detailed breakdown of each detected marker with confidence bars.
- **Response panel**: Actionable counter-messaging recommendations.
- **Alert feed**: Recent high-risk detections across all analyzed content.

## Database Tables

- `disinfo_analyses` -- content, risk level, indicators (jsonb), recommended response
