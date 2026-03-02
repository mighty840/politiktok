# F11 -- Coalition Tension Detector

Detects emerging tensions within political coalitions by analyzing public statements, voting patterns, and communication shifts between coalition segments.

## Key Features

- **Coalition segment modeling**: Define segments within a coalition with their priorities and positions.
- **Tension detection**: Identify friction points between segments on specific policy issues.
- **Severity assessment**: Rate detected tensions as low, medium, high, or critical.
- **Explanation generation**: Provide context for why tensions exist and how they might escalate.
- **Historical tracking**: Monitor how tensions evolve over time.
- **Consensus identification**: Find areas of agreement that can strengthen coalition unity.

## Server Functions

| Function | Endpoint | Description |
|----------|----------|-------------|
| `analyze_tensions` | `coalition/analyze` | Analyze policy text for coalition tensions |
| `list_analyses` | `coalition/list` | List past tension analyses |
| `get_analysis` | `coalition/get` | Retrieve a specific analysis |

## Coalition Segment Structure

Each segment is defined by:

- **Name**: Faction or group name
- **Description**: Brief characterization
- **Key priorities**: Issues this segment cares most about

## Tension Structure

Detected tensions include:

- **Segment A / Segment B**: The two groups in tension
- **Issue**: The specific policy area causing friction
- **Severity**: low, medium, high, or critical
- **Explanation**: Why this tension exists and potential consequences

## UI Components

- **Coalition analyzer** (`/coalition`): Input panel for policy text and coalition segment definitions.
- **Tension matrix**: Grid view showing tensions between all segment pairs.
- **Severity indicators**: Color-coded severity badges for quick scanning.
- **Consensus panel**: Areas of agreement across segments.

## Database Tables

- `coalition_analyses` -- policy text, segments (jsonb), tensions (jsonb), consensus areas
