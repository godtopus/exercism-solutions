class Pangrams {
    companion object {
        fun isPangram(sequence: String) = sequence.toLowerCase().filter{it.isLetter()}.toCharArray().distinct().count() == 26
    }
}