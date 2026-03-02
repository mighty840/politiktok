# F23 -- Compliance Reporting

Automates electoral compliance reporting by tracking expenditures, donations, and campaign activities against regulatory requirements.

## Key Features

- **Automated report generation**: LLM-powered generation of compliance reports structured to regulatory standards.
- **Period-based reporting**: Generate reports for specific time periods (monthly, quarterly, annual).
- **Section-level compliance**: Each report section includes a compliance status (compliant/non-compliant) with explanatory notes.
- **Action checking**: Evaluate specific campaign actions against electoral law requirements.
- **Multi-type reports**: Support for FEC filings, state compliance, expenditure reports, and donation tracking.
- **Audit trail**: Complete history of generated reports with status tracking.

## Server Functions

| Function | Endpoint | Description |
|----------|----------|-------------|
| `generate_report` | `compliance/generate` | Generate a compliance report |
| `check_action` | `compliance/check` | Check an action against electoral law |
| `list_reports` | `compliance/list` | List past compliance reports |
| `get_report` | `compliance/get` | Retrieve a specific report |

## Report Structure

Each compliance report contains:

- **Report type**: FEC filing, state compliance, expenditure report, etc.
- **Period**: The time period covered
- **Sections**: List of compliance sections, each with:
  - **Title**: Section heading
  - **Content**: Detailed compliance information
  - **Compliant**: Boolean compliance status
  - **Notes**: Explanatory notes or flagged concerns
- **Status**: draft, under_review, filed, or rejected

## Action Check

The action check feature evaluates a specific campaign activity:

- Input: Description of the action and relevant regulatory context
- Output: Compliance assessment with applicable rules and recommendations

## UI Components

- **Compliance dashboard** (`/compliance`): Overview of reporting status with compliance indicators.
- **Report generator**: Form to configure report type and period.
- **Report viewer**: Rendered report with section-level compliance badges (green/red).
- **Action checker**: Quick check form for evaluating campaign activities.
- **Report history**: Browse, filter, and download past reports.

## Database Tables

- `compliance_reports` -- report type, period, sections (jsonb), status
- `compliance_checks` -- action descriptions, assessments, applicable rules
