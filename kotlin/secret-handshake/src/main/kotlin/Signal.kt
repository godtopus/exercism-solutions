enum class Signal {
  WINK, DOUBLE_BLINK, CLOSE_YOUR_EYES, JUMP
}

class HandshakeCalculator {
  companion object {
      fun calculateHandshake(n: Int): List<Signal> {
          var handshake = listOf(1, 2, 4, 8).filter { (n and it == it) }.map {
              when (it) {
                  1 -> Signal.WINK
                  2 -> Signal.DOUBLE_BLINK
                  4 -> Signal.CLOSE_YOUR_EYES
                  8 -> Signal.JUMP
                  else -> throw RuntimeException("The one who catches the bug fixes the bug")
              }
          }

          return if (n and 16 == 16) handshake.reversed() else handshake
      }
  }
}