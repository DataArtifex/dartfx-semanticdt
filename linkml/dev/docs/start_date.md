---
search:
  boost: 5.0
---

# Slot: start_date


_Start date of the coverage formatted strictly according to ISO 8601 (YYYY-MM-DD)._



<div data-search-exclude markdown="1">



URI: [sdt:start_date](https://w3id.org/dartfx/semanticdt/start_date)

## Inheritance

* [iso_date](iso_date.md)
    * **start_date**






## Applicable Classes

| Name | Description | Modifies Slot |
| --- | --- | --- |
| [TemporalCoverage](TemporalCoverage.md) | Temporal coverage boundaries represented by description, start/end dates, or ... |  no  |






## Properties

### Type and Range

| Property | Value |
| --- | --- |
| Range | [String](String.md) |
| Domain Of | [TemporalCoverage](TemporalCoverage.md) |

### Cardinality and Requirements

| Property | Value |
| --- | --- |
### Slot Characteristics

| Property | Value |
| --- | --- |
| Owner | [TemporalCoverage](TemporalCoverage.md) |


### Value Constraints

| Property | Value |
| --- | --- |
| Regex Pattern | `^\d{4}-\d{2}-\d{2}$` |












## Identifier and Mapping Information





### Schema Source


* from schema: https://w3id.org/dartfx/semanticdt




## Mappings

| Mapping Type | Mapped Value |
| ---  | ---  |
| self | sdt:start_date |
| native | sdt:start_date |




## LinkML Source

<details>
```yaml
name: start_date
description: Start date of the coverage formatted strictly according to ISO 8601 (YYYY-MM-DD).
from_schema: https://w3id.org/dartfx/semanticdt
rank: 1000
is_a: iso_date
owner: TemporalCoverage
domain_of:
- TemporalCoverage
range: string
pattern: ^\d{4}-\d{2}-\d{2}$

```
</details></div>
