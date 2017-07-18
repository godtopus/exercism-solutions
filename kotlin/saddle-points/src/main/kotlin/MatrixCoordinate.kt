data class MatrixCoordinate(val row: Int, val col: Int)

class Matrix(matrix: List<List<Int>>) {
    var saddlePoints = setOf<MatrixCoordinate>()

    init {
        this.saddlePoints = matrix.mapIndexed { rowIndex, row -> row.mapIndexed { colIndex, col ->
            if (row.all { col >= it } && matrix.all { it[colIndex] >= col }) {
                MatrixCoordinate(rowIndex, colIndex)
            } else {
                MatrixCoordinate(-1, -1)
            }
        } }.flatten().filter { it.row >= 0 && it.col >= 0 }.toSet()
    }
}