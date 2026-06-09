---
search:
  boost: 5.0
---

# Slot: code


_The actual code snippet or expression (e.g., Python function/lambda, SQL expression)._



<div data-search-exclude markdown="1">



URI: [sdt:code](https://w3id.org/dartfx/semanticdt/code)
<!-- no inheritance hierarchy -->





## Applicable Classes

| Name | Description | Modifies Slot |
| --- | --- | --- |
| [CodeSnippet](CodeSnippet.md) | A concrete piece of code or expression implementing logic (e |  no  |






## Properties

### Type and Range

| Property | Value |
| --- | --- |
| Range | [String](String.md) |
| Domain Of | [CodeSnippet](CodeSnippet.md) |

### Cardinality and Requirements

| Property | Value |
| --- | --- |
| Required | Yes |
### Slot Characteristics

| Property | Value |
| --- | --- |
| Owner | [CodeSnippet](CodeSnippet.md) |












## Identifier and Mapping Information





### Schema Source


* from schema: https://w3id.org/dartfx/semanticdt




## Mappings

| Mapping Type | Mapped Value |
| ---  | ---  |
| self | sdt:code |
| native | sdt:code |




## LinkML Source

<details>
```yaml
name: code
description: The actual code snippet or expression (e.g., Python function/lambda,
  SQL expression).
from_schema: https://w3id.org/dartfx/semanticdt
rank: 1000
owner: CodeSnippet
domain_of:
- CodeSnippet
range: string
required: true

```
</details></div>
