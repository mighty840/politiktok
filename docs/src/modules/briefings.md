# F12 -- Candidate Briefing Generator

Produces concise, structured briefings for candidates covering relevant news, schedule context, talking points, and emerging issues. Each briefing is organized into prioritized sections for quick consumption before events.

## Key Features

- **Structured briefings**: Generate multi-section briefings with prioritized content.
- **Priority tagging**: Each section is tagged as high, medium, or low priority for time-pressed candidates.
- **Context-aware**: Briefings incorporate event type, audience, and current news landscape.
- **Quick consumption format**: Designed for scanning in 5--10 minutes before an event.
- **Briefing history**: Store and retrieve past briefings for reference.
- **Custom section ordering**: Reorder sections based on candidate preferences.

## Server Functions

| Function | Endpoint | Description |
|----------|----------|-------------|
| `generate_briefing` | `briefings/generate` | Generate a candidate briefing |
| `list_briefings` | `briefings/list` | List past briefings |
| `get_briefing` | `briefings/get` | Retrieve a specific briefing |

## Briefing Structure

Each briefing contains:

- **Title**: Descriptive briefing title (e.g., "Town Hall - Healthcare Focus - March 2")
- **Sections**: Ordered list of content sections, each with:
  - **Heading**: Section title
  - **Content**: Detailed information in markdown
  - **Priority**: high, medium, or low

Typical sections include:

1. Key Headlines (high priority)
2. Event Context (high priority)
3. Talking Points (high priority)
4. Background Information (medium priority)
5. Potential Questions (medium priority)
6. Opponent Activity (low priority)

## UI Components

- **Briefing generator** (`/briefings`): Form to specify event context, topics, and audience for briefing generation.
- **Briefing viewer**: Rendered briefing with priority-colored section headers and collapsible content.
- **Priority filter**: Toggle to show only high-priority sections for time-pressed review.
- **Briefing history**: Browse and search past briefings.

## Database Tables

- `briefings` -- briefing documents with title, sections (jsonb), timestamps
