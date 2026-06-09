---
search:
  boost: 5.0
---

# Slot: codes


_A list of formal codes or URIs (e.g., ISO country/subdivision codes, GeoNames URIs, Wikidata URIs)._



<div data-search-exclude markdown="1">



URI: [sdt:codes](https://w3id.org/dartfx/semanticdt/codes)
<!-- no inheritance hierarchy -->





## Applicable Classes

| Name | Description | Modifies Slot |
| --- | --- | --- |
| [GeospatialCoverage](GeospatialCoverage.md) | Geospatial coverage boundaries represented by description and/or formal codes... |  no  |






## Properties

### Type and Range

| Property | Value |
| --- | --- |
| Range | [String](String.md) |
| Domain Of | [GeospatialCoverage](GeospatialCoverage.md) |

### Cardinality and Requirements

| Property | Value |
| --- | --- |
| Multivalued | Yes |
### Slot Characteristics

| Property | Value |
| --- | --- |
| Owner | [GeospatialCoverage](GeospatialCoverage.md) |












## Identifier and Mapping Information





### Schema Source


* from schema: https://w3id.org/dartfx/semanticdt




## Mappings

| Mapping Type | Mapped Value |
| ---  | ---  |
| self | sdt:codes |
| native | sdt:codes |




## LinkML Source

<details>
```yaml
name: codes
description: A list of formal codes or URIs (e.g., ISO country/subdivision codes,
  GeoNames URIs, Wikidata URIs).
from_schema: https://w3id.org/dartfx/semanticdt
rank: 1000
owner: GeospatialCoverage
domain_of:
- GeospatialCoverage
range: string
multivalued: true

```
</details></div>
