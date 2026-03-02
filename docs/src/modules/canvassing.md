# F06 -- Canvassing Script Generator

Creates dynamic door-to-door canvassing scripts adapted to neighborhood demographics, local issues, and voter concerns.

## Key Features

- **Demographic targeting**: Generate scripts tailored to specific voter segments (suburban families, college students, retirees, etc.).
- **Local issue integration**: Incorporate neighborhood-specific issues into talking points.
- **Structured sections**: Every script includes an opening, issue discussion, objection handling, and closing.
- **Talking point extraction**: Each section includes bullet-point talking points for quick canvasser reference.
- **Script history**: Save and retrieve past scripts for canvasser training and reference.
- **Key asks configuration**: Define the specific actions you want voters to take (vote, volunteer, donate, attend event).

## Server Functions

| Function | Endpoint | Description |
|----------|----------|-------------|
| `generate_script` | `canvassing/generate` | Generate a complete canvassing script |
| `list_scripts` | `canvassing/list` | List saved scripts |
| `get_script` | `canvassing/get` | Retrieve a specific script |

## Script Structure

Each generated script contains four sections:

| Section | Purpose |
|---------|---------|
| **Opening** | Introduce the canvasser and candidate, establish rapport |
| **Issue Discussion** | Address local concerns with candidate's positions |
| **Objection Handling** | Prepared responses to common voter objections |
| **Closing** | Call to action and next steps |

## UI Components

- **Script generator** (`/canvassing`): Form to configure voter segment, local issues, candidate name, and key asks.
- **Script viewer**: Rendered script with collapsible sections and highlighted talking points.
- **Print-friendly mode**: Simplified layout for printing canvasser handouts.

## Database Tables

- `canvassing_scripts` -- saved scripts with voter segment, issues, sections (jsonb)
