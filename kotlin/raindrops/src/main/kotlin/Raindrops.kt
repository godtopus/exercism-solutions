class Raindrops {
    companion object {
        fun convert(sequence: Int): String {
            var translated = "".plus(if (sequence % 3 == 0) "Pling" else "").plus(if (sequence % 5 == 0) "Plang" else "").plus(if (sequence % 7 == 0) "Plong" else "")
            return if (translated == "") sequence.toString() else translated
        }
    }
}