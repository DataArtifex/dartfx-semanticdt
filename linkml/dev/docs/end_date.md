---
search:
  boost: 5.0
---

# Slot: end_date


_End date of the coverage formatted strictly according to ISO 8601 (YYYY-MM-DD)._



<div data-search-exclude markdown="1">



URI: [sdt:end_date](https://w3id.org/dartfx/semanticdt/end_date)

## Inheritance

* [iso_date](iso_date.md)
    * **end_date**






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
| self | sdt:end_date |
| native | sdt:end_date |




## LinkML Source

<details>
```yaml
name: end_date
description: End date of the coverage formatted strictly according to ISO 8601 (YYYY-MM-DD).
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
