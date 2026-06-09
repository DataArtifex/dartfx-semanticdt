---
search:
  boost: 5.0
---

# Slot: target_environment


_Target platform, framework, database, or language. Prefer using values from the TargetEnvironmentEnum controlled vocabulary where applicable. Custom values not in the enum are permitted since this slot is of type string._



<div data-search-exclude markdown="1">



URI: [sdt:target_environment](https://w3id.org/dartfx/semanticdt/target_environment)
<!-- no inheritance hierarchy -->





## Applicable Classes

| Name | Description | Modifies Slot |
| --- | --- | --- |
| [StorageType](StorageType.md) | Physical or logical storage data type binding |  no  |
| [CodeSnippet](CodeSnippet.md) | A concrete piece of code or expression implementing logic (e |  no  |






## Properties

### Type and Range

| Property | Value |
| --- | --- |
| Range | [String](String.md) |
| Domain Of | [StorageType](StorageType.md), [CodeSnippet](CodeSnippet.md) |

### Cardinality and Requirements

| Property | Value |
| --- | --- |
| Required | Yes |










## Identifier and Mapping Information





### Schema Source


* from schema: https://w3id.org/dartfx/semanticdt




## Mappings

| Mapping Type | Mapped Value |
| ---  | ---  |
| self | sdt:target_environment |
| native | sdt:target_environment |




## LinkML Source

<details>
```yaml
name: target_environment
description: Target platform, framework, database, or language. Prefer using values
  from the TargetEnvironmentEnum controlled vocabulary where applicable. Custom values
  not in the enum are permitted since this slot is of type string.
from_schema: https://w3id.org/dartfx/semanticdt
rank: 1000
domain_of:
- StorageType
- CodeSnippet
range: string
required: true

```
</details></div>
