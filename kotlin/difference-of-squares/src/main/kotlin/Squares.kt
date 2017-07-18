class Squares(private val n: Int) {
    fun squareOfSum() = (1..n).sum().let { it.times(it) }
    fun sumOfSquares() = (1..n).sumBy { it.times(it) }
    fun difference() = squareOfSum() - sumOfSquares()
}