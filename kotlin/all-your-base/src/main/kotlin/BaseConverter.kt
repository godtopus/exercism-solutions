class BaseConverter(val base: Int, val number: IntArray) {
    init {
        if (base < 2) {
            throw IllegalArgumentException("Bases must be at least 2.")
        }

        if (number.isEmpty()) {
            throw IllegalArgumentException("You must supply at least one digit.")
        }

        if (number.size > 1 && number[0] == 0) {
            throw IllegalArgumentException("Digits may not contain leading zeros.")
        }

        if (number.any { it < 0 }) {
            throw IllegalArgumentException("Digits may not be negative.")
        }

        if (number.any { it >= base }) {
            throw IllegalArgumentException("All digits must be strictly less than the base.")
        }
    }

    fun convertToBase(baseConverted: Int) : IntArray {
        if (baseConverted < 2) {
            throw IllegalArgumentException("Bases must be at least 2.")
        }

        var sum = number.fold(0, { total, next -> total * base + next })
        if (sum == 0) {
            return intArrayOf(0)
        }

        var result = mutableListOf<Int>()
        while (sum > 0) {
            result.add(sum % baseConverted)
            sum /= baseConverted
        }

        return result.reversed().toIntArray()
    }
}