class Acronym {
    companion object {
        fun generate(phrase: String) = phrase.split(' ', '-').map { acronymonize(it.filter(Char::isLetter)) }.joinToString("")

        private fun acronymonize(letters: String): String {
            if (letters.isEmpty()) {
                return ""
            }

            var nextWord = letters.dropWhile { it.isUpperCase() }.dropWhile { it.isLowerCase() }

            return letters[0].toUpperCase().plus(acronymonize(nextWord))
        }
    }
}