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
  A concrete piece of code or expression implementing logic (e.g., validation, generation) in a specific environment.
**/
@Data
@EqualsAndHashCode(callSuper=false)
public class CodeSnippet  {

  private String targetEnvironment;
  private String code;


}
