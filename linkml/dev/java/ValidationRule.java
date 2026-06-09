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
  A rule defining syntactical or logical validation of values.
**/
@Data
@EqualsAndHashCode(callSuper=false)
public class ValidationRule  {

  private String ruleType;
  private String pattern;
  private String algorithm;
  private String message;
  private List<CodeSnippet> codeSnippets;


}
