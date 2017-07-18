import java.time.LocalDate
import java.time.LocalDateTime

class Gigasecond(date: LocalDateTime) {
    var date = date.plusSeconds(1000000000)

    constructor(localDate: LocalDate) : this(localDate.atStartOfDay())
}