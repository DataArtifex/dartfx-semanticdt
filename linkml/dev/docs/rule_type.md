---
search:
  boost: 5.0
---

# Slot: rule_type


_Type of validation (e.g., regex, checksum, range)._



<div data-search-exclude markdown="1">



URI: [sdt:rule_type](https://w3id.org/dartfx/semanticdt/rule_type)
<!-- no inheritance hierarchy -->





## Applicable Classes

| Name | Description | Modifies Slot |
| --- | --- | --- |
| [ValidationRule](ValidationRule.md) | A rule defining syntactical or logical validation of values |  no  |






## Properties

### Type and Range

| Property | Value |
| --- | --- |
| Range | [String](String.md) |
| Domain Of | [ValidationRule](ValidationRule.md) |

### Cardinality and Requirements

| Property | Value |
| --- | --- |
| Required | Yes |
### Slot Characteristics

| Property | Value |
| --- | --- |
| Owner | [ValidationRule](ValidationRule.md) |












## Identifier and Mapping Information





### Schema Source


* from schema: https://w3id.org/dartfx/semanticdt




## Mappings

| Mapping Type | Mapped Value |
| ---  | ---  |
| self | sdt:rule_type |
| native | sdt:rule_type |




## LinkML Source

<details>
```yaml
name: rule_type
description: Type of validation (e.g., regex, checksum, range).
from_schema: https://w3id.org/dartfx/semanticdt
rank: 1000
owner: ValidationRule
domain_of:
- ValidationRule
range: string
required: true

```
</details></div>
