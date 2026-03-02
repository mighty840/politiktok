# F21 -- Media Monitor

Tracks media coverage across outlets, detecting bias patterns and coverage gaps to inform media strategy and enable rapid response.

## Key Features

- **Article analysis**: LLM-powered analysis of individual media articles for bias, claims, and tone.
- **Bias assessment**: Detect political lean and framing bias in coverage.
- **Claim extraction**: Identify key claims made in articles for fact-checking.
- **Fact-check notes**: Generate preliminary fact-check observations.
- **Coverage comparison**: Compare how different outlets cover the same topic.
- **Coverage tone tracking**: Monitor whether coverage is positive, negative, or neutral.

## Server Functions

| Function | Endpoint | Description |
|----------|----------|-------------|
| `analyze_article` | `media-monitor/analyze` | Analyze a media article for bias and claims |
| `compare_coverage` | `media-monitor/compare` | Compare coverage across outlets on a topic |
| `list_analyses` | `media-monitor/list` | List past media analyses |
| `get_analysis` | `media-monitor/get` | Retrieve a specific media analysis |

## Analysis Output

Each media analysis includes:

- **Source**: The media outlet
- **Bias assessment**: Detected bias direction and magnitude
- **Key claims**: Factual claims made in the article
- **Fact-check notes**: Preliminary observations on claim accuracy
- **Coverage tone**: Overall tone classification

## Coverage Comparison

When comparing coverage of a topic across outlets:

- **Topic**: The subject being compared
- **Source analyses**: Per-outlet analysis showing how each covers the story
- **Coverage gaps**: Issues or perspectives missing from certain outlets
- **Overall narrative**: How the combined coverage shapes public perception

## UI Components

- **Media monitor page** (`/media-monitor`): Article submission form with analysis results and outlet comparison view.
- **Bias indicators**: Visual bias spectrum for each analyzed article.
- **Claim tracker**: Extracted claims with fact-check notes and source links.
- **Outlet comparison grid**: Side-by-side comparison of how outlets cover the same topic.

## Database Tables

- `media_analyses` -- article text, source, bias assessment, claims, tone, fact-check notes
- `coverage_comparisons` -- topic, per-source analyses (jsonb)
