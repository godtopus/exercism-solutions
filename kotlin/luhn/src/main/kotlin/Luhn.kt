class Luhn {
    companion object {
        fun isValid(sequence: String): Boolean {
            val trimmed = sequence.filterNot(Char::isWhitespace)
            if (trimmed.length <= 1 || trimmed.any { !it.isDigit() }) {
                return false
            }

            return trimmed.reversed().mapIndexed { index, c ->
                if (index.rem(2) == 1) {
                    val doubled = c.minus('0').times(2)
                    if (doubled > 9) doubled - 9 else doubled
                } else {
                    c.minus('0')
                }
            }.sum().rem(10) == 0
        }
    }
}