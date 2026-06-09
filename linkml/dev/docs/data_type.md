---
search:
  boost: 5.0
---

# Slot: data_type


_Data type identifier in the target environment (e.g., string, varchar, integer, str)._



<div data-search-exclude markdown="1">



URI: [sdt:data_type](https://w3id.org/dartfx/semanticdt/data_type)
<!-- no inheritance hierarchy -->





## Applicable Classes

| Name | Description | Modifies Slot |
| --- | --- | --- |
| [StorageType](StorageType.md) | Physical or logical storage data type binding |  no  |






## Properties

### Type and Range

| Property | Value |
| --- | --- |
| Range | [String](String.md) |
| Domain Of | [StorageType](StorageType.md) |

### Cardinality and Requirements

| Property | Value |
| --- | --- |
| Required | Yes |
### Slot Characteristics

| Property | Value |
| --- | --- |
| Owner | [StorageType](StorageType.md) |












## Identifier and Mapping Information





### Schema Source


* from schema: https://w3id.org/dartfx/semanticdt




## Mappings

| Mapping Type | Mapped Value |
| ---  | ---  |
| self | sdt:data_type |
| native | sdt:data_type |




## LinkML Source

<details>
```yaml
name: data_type
description: Data type identifier in the target environment (e.g., string, varchar,
  integer, str).
from_schema: https://w3id.org/dartfx/semanticdt
rank: 1000
owner: StorageType
domain_of:
- StorageType
range: string
required: true

```
</details></div>
