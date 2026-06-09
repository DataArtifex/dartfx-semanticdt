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
  Concrete instance of values.
**/
@Data
@EqualsAndHashCode(callSuper=false)
public class Example  {

  private String value;
  private boolean isValid;
  private String description;


}
