package org.dataartifex.sdt;

/* metamodel_version: 1.11.0 */
/* version: 0.1.0-dev */
import java.net.URI;
import java.time.LocalDate;
import java.time.LocalTime;
import java.time.ZonedDateTime;
import java.util.List;
import lombok.*;

/**
  A high-level data type associated with a distinct concept, validation/generation rules, and metadata.
**/
@Data
@EqualsAndHashCode(callSuper=false)
public class SemanticDataType  {

  private String id;
  private String name;
  private String description;
  private String version;
  private List<ConceptReference> concepts;
  private List<StorageType> storageTypes;
  private ClassificationSystem classification;
  private List<ValidationRule> validationRules;
  private List<GenerationRule> generationRules;
  private AgentInstruction agentInstructions;
  private List<AgentSkill> agentSkills;
  private ScopeContext scope;
  private DisplayFormat displayFormat;
  private List<Example> examples;
  private List<Resource> resources;


}
