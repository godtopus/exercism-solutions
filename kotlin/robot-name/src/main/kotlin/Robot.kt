import java.util.Random
import java.security.SecureRandom

class Robot(private val r: Random = SecureRandom()) {
    var name: String = reset()
        private set

    fun reset(): String {
        name = "".plus((0..1).map { (r.nextInt(26) + 'A'.toInt()).toChar() }.joinToString("")).plus((0..2).map { r.nextInt(10) }.joinToString(""))
        return name
    }
}