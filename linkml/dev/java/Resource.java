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
  An external resource, citation, or reference related to the Semantic Data Type (using simple Dublin Core elements).
**/
@Data
@EqualsAndHashCode(callSuper=false)
public class Resource  {

  private String url;
  private String title;
  private String description;
  private String citation;
  private String publisher;


}
