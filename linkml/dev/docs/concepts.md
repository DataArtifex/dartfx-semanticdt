---
search:
  boost: 5.0
---

# Slot: concepts


_A list of conceptual ontology alignments or mapping references representing this data type in external systems (e.g., SKOS concepts, Wikidata items, or vocabularies like DDI-CDI and schema.org). This should list multiple representations of the same underlying semantic concept, not distinct concepts._



<div data-search-exclude markdown="1">



URI: [sdt:concepts](https://w3id.org/dartfx/semanticdt/concepts)
<!-- no inheritance hierarchy -->





## Applicable Classes

| Name | Description | Modifies Slot |
| --- | --- | --- |
| [SemanticDataType](SemanticDataType.md) | A high-level data type associated with a distinct concept, validation/generat... |  no  |






## Properties

### Type and Range

| Property | Value |
| --- | --- |
| Range | [ConceptReference](ConceptReference.md) |
| Domain Of | [SemanticDataType](SemanticDataType.md) |

### Cardinality and Requirements

| Property | Value |
| --- | --- |
| Multivalued | Yes |
### Slot Characteristics

| Property | Value |
| --- | --- |
| Owner | [SemanticDataType](SemanticDataType.md) |












## Identifier and Mapping Information





### Schema Source


* from schema: https://w3id.org/dartfx/semanticdt




## Mappings

| Mapping Type | Mapped Value |
| ---  | ---  |
| self | sdt:concepts |
| native | sdt:concepts |




## LinkML Source

<details>
```yaml
name: concepts
description: A list of conceptual ontology alignments or mapping references representing
  this data type in external systems (e.g., SKOS concepts, Wikidata items, or vocabularies
  like DDI-CDI and schema.org). This should list multiple representations of the same
  underlying semantic concept, not distinct concepts.
from_schema: https://w3id.org/dartfx/semanticdt
rank: 1000
owner: SemanticDataType
domain_of:
- SemanticDataType
range: ConceptReference
multivalued: true

```
</details></div>
