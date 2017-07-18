class Scrabble {
    companion object {
        fun scoreWord(word: String): Int {
            return word.fold(0, { total, next -> total + score(next) })
        }

        private fun score(letter: Char): Int {
            return when (letter.toUpperCase()) {
                'A', 'E', 'I', 'O', 'U', 'L', 'N', 'R', 'S', 'T' -> 1
                'D', 'G' -> 2
                'B', 'C', 'M', 'P' -> 3
                'F', 'H', 'V', 'W', 'Y' -> 4
                'K' -> 5
                'J', 'X' -> 8
                'Q', 'Z' -> 10
                else -> 0
            }
        }
    }
}