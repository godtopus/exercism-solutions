class DiamondPrinter {
    fun printToList(c: Char): List<String> {
        var n = ('A'..c).count()

        var top = ('A'..c).mapIndexed { index, c ->
            var outerPadding = (0..n - index - 2).map { " " }.joinToString(separator = "")
            var innerPadding = if (index == 0) "" else (0..(n * 2 - 1) - outerPadding.length * 2 - 2 - 1).map { " " }.joinToString(separator = "")

            outerPadding.plus(c).plus(innerPadding).plus(if (index == 0) "" else c).plus(outerPadding)
        }

        return sequenceOf(top, top.reversed().drop(1)).flatten().toList()
    }
}
