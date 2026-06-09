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
  Physical or logical storage data type binding.
**/
@Data
@EqualsAndHashCode(callSuper=false)
public class StorageType  {

  private String targetEnvironment;
  private String dataType;
  private String formatModifier;


}
