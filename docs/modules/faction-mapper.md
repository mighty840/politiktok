# F19 -- Faction Mapper

Maps internal party factions and identifies consensus opportunities by analyzing voting records, public statements, and policy positions.

## Key Features

- **Faction identification**: Extract and characterize distinct factions within a political party or movement.
- **Ideology mapping**: Describe each faction's ideological position and priorities.
- **Key figure identification**: Identify influential individuals within each faction.
- **Alliance detection**: Map alliances and cooperative relationships between factions.
- **Conflict mapping**: Identify specific policy conflicts between factions.
- **Consensus scoring**: Find areas where factions agree, enabling coalition-building.

## Server Functions

| Function | Endpoint | Description |
|----------|----------|-------------|
| `analyze_factions` | `faction-mapper/analyze` | Analyze a political context for factions |
| `list_analyses` | `faction-mapper/list` | List past faction analyses |
| `get_analysis` | `faction-mapper/get` | Retrieve a specific analysis |

## Faction Structure

Each identified faction includes:

- **Name**: Faction name or label
- **Ideology**: Brief ideological characterization
- **Key figures**: Notable members or leaders
- **Positions**: Key policy positions
- **Influence score**: Relative influence within the party (0.0--1.0)

## Analysis Output

A complete faction analysis includes:

- **Context**: The political situation analyzed
- **Factions**: List of identified factions with full profiles
- **Alliances**: Pairs of factions that tend to cooperate
- **Conflicts**: Triples of (faction A, faction B, issue) identifying specific disputes
- **Consensus areas**: Policy areas where most factions align

## UI Components

- **Faction mapper page** (`/faction-mapper`): Context input with faction analysis visualization.
- **Faction cards**: Detailed cards for each faction with key figures and positions.
- **Relationship diagram**: Visual map showing alliances and conflicts between factions.
- **Consensus finder**: Highlighted areas of agreement for coalition-building.

## Database Tables

- `faction_analyses` -- context, factions (jsonb), alliances, conflicts, consensus areas
