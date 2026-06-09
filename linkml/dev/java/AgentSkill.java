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
  Executable tools, functions, or workflows related to the semantic type.
**/
@Data
@EqualsAndHashCode(callSuper=false)
public class AgentSkill  {

  private String toolName;
  private String description;
  private String apiDefinition;


}
