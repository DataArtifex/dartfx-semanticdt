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
  Controlled vocabulary or coding scheme (making this a categorical variable).
**/
@Data
@EqualsAndHashCode(callSuper=false)
public class ClassificationSystem  {

  private String name;
  private String uri;
  private String version;


}
