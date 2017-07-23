class PascalsTriangle {
    companion object {
        fun computeTriangle(n: Int): List<List<Int>> {
            if (n < 0) {
                throw IllegalArgumentException()
            }

            val triangle = mutableListOf<List<Int>>()

            for (level in (0..n - 1)) {
                if (triangle.isEmpty()) {
                    triangle.add(listOf(1))
                } else {
                    triangle.add(listOf(1).plus((0..level - 1).zip((0..level - 1).drop(1)).map { triangle.last()[it.first] + triangle.last()[it.second] }.toList()).plus(1))
                }
            }

            return triangle
        }
    }
}