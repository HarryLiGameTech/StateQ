package org.stateq.config

import org.stateq.exception.unreachable

data class Config @JvmOverloads constructor(
    val targetLanguage: TargetLanguage,
    val buildLibrary: Boolean = false,
)

enum class TargetLanguage {
    C, Cpp, Rust, Java, Python;
    fun of(name: String) = when (name.lowercase()) {
        "c" -> C
        "c++" -> Cpp
        "cpp" -> Cpp
        "cxx" -> Cpp
        "rust" -> Rust
        "rs" -> Rust
        "java" -> Java
        "py" -> Python
        "python" -> Python
        else -> unreachable()
    }
}
