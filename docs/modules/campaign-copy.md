# F04 -- Campaign Copy Generator

Generates campaign messaging, ad copy, and communications materials tailored to specific audiences and platforms. Supports multiple output formats in a single generation pass.

## Key Features

- **Multi-format generation**: Produce emails, social media posts, press releases, speeches, and ad copy from a single brief.
- **Audience targeting**: Tailor tone and messaging for specific demographic and psychographic segments.
- **Tone control**: Select from professional, casual, urgent, inspirational, and other tonal presets.
- **Key message enforcement**: Ensure specific talking points are woven into all generated content.
- **Word limit adherence**: Per-format word limits for platform-appropriate content length.
- **Job history**: Store and retrieve past generation jobs for reference and iteration.

## Server Functions

| Function | Endpoint | Description |
|----------|----------|-------------|
| `generate_copy` | `campaign-copy/generate` | Generate copy across multiple formats |
| `list_copy_jobs` | `campaign-copy/list-jobs` | List past generation jobs |
| `get_copy_job` | `campaign-copy/get-job` | Retrieve a specific job with results |

## Request Structure

A copy generation request includes:

- **Topic**: The subject matter (e.g., "Infrastructure Investment Plan")
- **Key messages**: Talking points that must appear in the output
- **Audience**: Target demographic description
- **Tone**: Desired communication tone
- **Formats**: List of output formats (email, tweet, press_release, speech, ad)
- **Word limits**: Optional per-format word count constraints

## UI Components

- **Copy generator page** (`/campaign-copy`): Form-based interface for configuring generation parameters with real-time preview of results.
- **Format tabs**: Switch between generated formats (email, social, press release, etc.) in the output panel.
- **Copy history**: Browse and re-use previous generation jobs.
- **One-click copy**: Copy any generated text to clipboard.

## Database Tables

- `copy_jobs` -- generation requests and results with format-specific content stored as JSONB
