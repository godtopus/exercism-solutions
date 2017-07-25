class Robot(var gridPosition: GridPosition = GridPosition(0, 0), var orientation: Orientation = Orientation.NORTH) {
    fun turnRight() {
        orientation = when (orientation) {
            Orientation.NORTH -> Orientation.EAST
            Orientation.EAST -> Orientation.SOUTH
            Orientation.SOUTH -> Orientation.WEST
            Orientation.WEST -> Orientation.NORTH
        }
    }

    fun turnLeft() {
        orientation = when (orientation) {
            Orientation.NORTH -> Orientation.WEST
            Orientation.WEST -> Orientation.SOUTH
            Orientation.SOUTH -> Orientation.EAST
            Orientation.EAST -> Orientation.NORTH
        }
    }

    fun advance() {
        gridPosition = when (orientation) {
            Orientation.NORTH -> GridPosition(gridPosition.x, gridPosition.y + 1)
            Orientation.EAST -> GridPosition(gridPosition.x + 1, gridPosition.y)
            Orientation.SOUTH -> GridPosition(gridPosition.x, gridPosition.y - 1)
            Orientation.WEST -> GridPosition(gridPosition.x - 1, gridPosition.y)
        }
    }

    fun simulate(actions: String) {
        actions.forEach {
            when (it) {
                'R' -> turnRight()
                'L' -> turnLeft()
                'A' -> advance()
            }
        }
    }
}