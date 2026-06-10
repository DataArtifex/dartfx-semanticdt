---
search:
  boost: 5.0
---

# Slot: description

<div data-search-exclude markdown="1">



URI: [sdt:description](https://w3id.org/dartfx/semanticdt/description)
<!-- no inheritance hierarchy -->





## Applicable Classes

| Name | Description | Modifies Slot |
| --- | --- | --- |
| [SemanticDataType](SemanticDataType.md) | A high-level data type associated with a distinct concept, validation/generat... |  no  |
| [AgentSkill](AgentSkill.md) | Executable tools, functions, or workflows related to the semantic type |  no  |
| [GeospatialCoverage](GeospatialCoverage.md) | Geospatial coverage boundaries represented by description and/or formal codes... |  no  |
| [TemporalCoverage](TemporalCoverage.md) | Temporal coverage boundaries represented by description, start/end dates, or ... |  no  |
| [Example](Example.md) | Concrete instance of values |  no  |
| [Resource](Resource.md) | An external resource, citation, or reference related to the Semantic Data Typ... |  no  |






## Properties

### Type and Range

| Property | Value |
| --- | --- |
| Range | [String](String.md) |
| Domain Of | [SemanticDataType](SemanticDataType.md), [AgentSkill](AgentSkill.md), [GeospatialCoverage](GeospatialCoverage.md), [TemporalCoverage](TemporalCoverage.md), [Example](Example.md), [Resource](Resource.md) |

### Cardinality and Requirements

| Property | Value |
| --- | --- |










## Identifier and Mapping Information






## Mappings

| Mapping Type | Mapped Value |
| ---  | ---  |
| self | sdt:description |
| native | sdt:description |




## LinkML Source

<details>
```yaml
name: description
domain_of:
- SemanticDataType
- AgentSkill
- GeospatialCoverage
- TemporalCoverage
- Example
- Resource
range: string

```
</details></div>
