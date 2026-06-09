---
search:
  boost: 5.0
---

# Slot: match_type


_Relationship to the concept (e.g., exactMatch, closeMatch, broadMatch, narrowMatch)._



<div data-search-exclude markdown="1">



URI: [sdt:match_type](https://w3id.org/dartfx/semanticdt/match_type)
<!-- no inheritance hierarchy -->





## Applicable Classes

| Name | Description | Modifies Slot |
| --- | --- | --- |
| [ConceptReference](ConceptReference.md) | Reference to a concept in an external ontology or vocabulary |  no  |






## Properties

### Type and Range

| Property | Value |
| --- | --- |
| Range | [String](String.md) |
| Domain Of | [ConceptReference](ConceptReference.md) |

### Cardinality and Requirements

| Property | Value |
| --- | --- |
### Slot Characteristics

| Property | Value |
| --- | --- |
| If Absent | `string(exactMatch)` |
| Owner | [ConceptReference](ConceptReference.md) |












## Identifier and Mapping Information





### Schema Source


* from schema: https://w3id.org/dartfx/semanticdt




## Mappings

| Mapping Type | Mapped Value |
| ---  | ---  |
| self | sdt:match_type |
| native | sdt:match_type |




## LinkML Source

<details>
```yaml
name: match_type
description: Relationship to the concept (e.g., exactMatch, closeMatch, broadMatch,
  narrowMatch).
from_schema: https://w3id.org/dartfx/semanticdt
rank: 1000
ifabsent: string(exactMatch)
owner: ConceptReference
domain_of:
- ConceptReference
range: string

```
</details></div>
