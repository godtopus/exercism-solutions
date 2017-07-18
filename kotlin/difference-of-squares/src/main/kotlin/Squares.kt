class Squares(n: Int) {
    var n = n

    fun squareOfSum() = Math.pow((1..n).sum().toDouble(), 2.0).toInt()
    fun sumOfSquares() = (1..n).fold(0.0, { total, next -> total + Math.pow(next.toDouble(), 2.0) }).toInt()
    fun difference() = squareOfSum() - sumOfSquares()
}