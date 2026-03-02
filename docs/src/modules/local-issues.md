# F17 -- Local Issues Mapper

Maps neighborhood-level issues by aggregating local news, public records, and community feedback into actionable geographic intelligence.

## Key Features

- **Area-based analysis**: Generate issue reports for specific neighborhoods, districts, or communities.
- **Issue identification**: Extract and categorize local concerns from provided context.
- **Severity assessment**: Rate each issue's severity and urgency.
- **Demographic impact**: Identify which population segments are most affected by each issue.
- **Talking point generation**: Produce ready-to-use talking points for each local issue.
- **Priority ranking**: Overall priority ordering across all identified issues.

## Server Functions

| Function | Endpoint | Description |
|----------|----------|-------------|
| `analyze_local_issues` | `local-issues/analyze` | Analyze an area for local issues |
| `list_reports` | `local-issues/list` | List past issue reports |
| `get_report` | `local-issues/get` | Retrieve a specific report |

## Issue Structure

Each identified local issue includes:

- **Title**: Short issue name
- **Description**: Detailed explanation of the issue
- **Severity**: Rating (critical, high, medium, low)
- **Affected demographics**: Population segments impacted
- **Suggested talking points**: Campaign messaging angles

## Report Structure

A complete report includes:

- **Area description**: The geographic area analyzed
- **Issues**: List of identified issues with full details
- **Overall priorities**: Ranked list of the most important issues for the area

## UI Components

- **Local issues page** (`/local-issues`): Area input form with generated issue reports.
- **Issue cards**: Expandable cards with severity badges, demographic tags, and talking points.
- **Priority overview**: Summary view ranking all issues by importance.
- **Report history**: Browse past analyses by area.

## Database Tables

- `local_issue_reports` -- area description, issues (jsonb), priorities, timestamps
