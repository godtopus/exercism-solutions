fun hello(name: String = ""): String {
   val optionalName = if (name.isNullOrBlank()) "World" else name
   return "Hello, $optionalName!"
}
