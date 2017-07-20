class Isogram {
    companion object {
        fun isIsogram(sequence: String): Boolean {
            var letters = sequence.toLowerCase().filter(Char::isLetter);
            return letters.asSequence().distinct().count() == letters.count()
        }
    }
}