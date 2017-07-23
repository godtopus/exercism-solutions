class Deque<T> {
    private data class Node<T>(val value: T, var prev: Node<T>? = null, var next: Node<T>? = null)

    private var head: Node<T>? = null

    fun push(value: T) {
        if (head == null) {
            head = Node(value)
            head?.prev = head
            head?.next = head
        } else {
            val oldTail = head?.prev
            val tail = Node(value, prev = oldTail, next = head)
            oldTail?.next = tail
            head?.prev = tail
        }
    }

    fun pop(): T? {
        head = head?.prev
        return shift()
    }

    fun shift(): T? {
        val value = head?.value

        if (head == head?.next) {
            head = null
        } else {
            val f = head
            val b = head?.prev
            b?.next = f?.next
            head = head?.next
            head?.prev = b
        }

        return value
    }

    fun unshift(value: T) {
        push(value)
        head = head?.prev
    }
}