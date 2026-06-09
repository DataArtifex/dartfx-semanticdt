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
  Temporal coverage boundaries represented by description, start/end dates, or ISO 8601 periods/durations.
**/
@Data
@EqualsAndHashCode(callSuper=false)
public class TemporalCoverage  {

  private String description;
  private String startDate;
  private String endDate;
  private String isoPeriod;


}
