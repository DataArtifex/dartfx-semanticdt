---
search:
  boost: 5.0
---

# Slot: vocabulary


_Name of the vocabulary or ontology (e.g., SKOS, DDI-CDI, Wikidata, LCSH)._



<div data-search-exclude markdown="1">



URI: [sdt:vocabulary](https://w3id.org/dartfx/semanticdt/vocabulary)
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
| Owner | [ConceptReference](ConceptReference.md) |












## Identifier and Mapping Information





### Schema Source


* from schema: https://w3id.org/dartfx/semanticdt




## Mappings

| Mapping Type | Mapped Value |
| ---  | ---  |
| self | sdt:vocabulary |
| native | sdt:vocabulary |




## LinkML Source

<details>
```yaml
name: vocabulary
description: Name of the vocabulary or ontology (e.g., SKOS, DDI-CDI, Wikidata, LCSH).
from_schema: https://w3id.org/dartfx/semanticdt
rank: 1000
owner: ConceptReference
domain_of:
- ConceptReference
range: string

```
</details></div>
