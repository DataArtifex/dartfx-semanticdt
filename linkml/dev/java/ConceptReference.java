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
  Reference to a concept in an external ontology or vocabulary.
**/
@Data
@EqualsAndHashCode(callSuper=false)
public class ConceptReference  {

  private String uri;
  private String vocabulary;
  private String prefLabel;
  private String matchType;


}
