---
search:
  boost: 5.0
---

# Slot: iso_period


_ISO 8601 time interval or period representation (e.g., '2020-01-01/2020-12-31', 'P1Y')._



<div data-search-exclude markdown="1">



URI: [sdt:iso_period](https://w3id.org/dartfx/semanticdt/iso_period)
<!-- no inheritance hierarchy -->





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












## Identifier and Mapping Information





### Schema Source


* from schema: https://w3id.org/dartfx/semanticdt




## Mappings

| Mapping Type | Mapped Value |
| ---  | ---  |
| self | sdt:iso_period |
| native | sdt:iso_period |




## LinkML Source

<details>
```yaml
name: iso_period
description: ISO 8601 time interval or period representation (e.g., '2020-01-01/2020-12-31',
  'P1Y').
from_schema: https://w3id.org/dartfx/semanticdt
rank: 1000
owner: TemporalCoverage
domain_of:
- TemporalCoverage
range: string

```
</details></div>
