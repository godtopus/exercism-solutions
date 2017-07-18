class Luhn {
    companion object {
        fun isValid(sequence: String): Boolean {
            if (!sequence.all { it.isDigit() || it == ' ' } || sequence.filter { it.isDigit() }.length <= 1) {
                return false
            }

            var digits = sequence.filter { it.isDigit() }.map { it.toInt() - 48 }.reversed()

            var oddSum = digits.filterIndexed { index, _ -> index % 2 != 0 }.map { if (it * 2 > 9) it * 2 - 9 else it * 2 }.sum()
            var evenSum = digits.filterIndexed { index, _ -> index % 2 == 0 }.sum()
            println("digits $digits oddsum $oddSum evensum $evenSum")

            return (oddSum + evenSum) % 10 == 0
        }
    }
}