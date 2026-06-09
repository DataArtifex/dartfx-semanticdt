---
search:
  boost: 5.0
---

# Slot: generator_type


_Type of generator (e.g., regex_fuzzer, template, faker_provider, custom)._



<div data-search-exclude markdown="1">



URI: [sdt:generator_type](https://w3id.org/dartfx/semanticdt/generator_type)
<!-- no inheritance hierarchy -->





## Applicable Classes

| Name | Description | Modifies Slot |
| --- | --- | --- |
| [GenerationRule](GenerationRule.md) | Directive for synthetic data generation |  no  |






## Properties

### Type and Range

| Property | Value |
| --- | --- |
| Range | [String](String.md) |
| Domain Of | [GenerationRule](GenerationRule.md) |

### Cardinality and Requirements

| Property | Value |
| --- | --- |
| Required | Yes |
### Slot Characteristics

| Property | Value |
| --- | --- |
| Owner | [GenerationRule](GenerationRule.md) |












## Identifier and Mapping Information





### Schema Source


* from schema: https://w3id.org/dartfx/semanticdt




## Mappings

| Mapping Type | Mapped Value |
| ---  | ---  |
| self | sdt:generator_type |
| native | sdt:generator_type |




## LinkML Source

<details>
```yaml
name: generator_type
description: Type of generator (e.g., regex_fuzzer, template, faker_provider, custom).
from_schema: https://w3id.org/dartfx/semanticdt
rank: 1000
owner: GenerationRule
domain_of:
- GenerationRule
range: string
required: true

```
</details></div>
