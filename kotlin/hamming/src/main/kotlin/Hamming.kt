class Hamming {
    companion object {
        fun compute(sequence1: String, sequence2: String): Int {
            if (sequence1.length != sequence2.length) {
                throw IllegalArgumentException("leftStrand and rightStrand must be of equal length.")
            }

            return sequence1.zip(sequence2).count { it.first != it.second }
        }
    }
}