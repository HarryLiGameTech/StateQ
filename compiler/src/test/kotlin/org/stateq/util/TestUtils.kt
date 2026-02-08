package org.stateq.util

import com.google.common.io.Resources
import org.junit.jupiter.api.fail
import org.stateq.exception.CompileErrorException
import org.stateq.exception.InternalCompileException

@Suppress("UnstableApiUsage")
fun readResourceFile(path: String): String {
    return String(Resources.getResource(path).openStream().readAllBytes())
}

inline fun <reified R: Any> Any.privateField(name: String): R {
    return this.javaClass.getDeclaredField(name).let {
        it.isAccessible = true
        assert(it.type == R::class.java)
        it.get(this) as R
    }
}

fun printCompileError(code: String, err: CompileError) {
    code.split('\n').mapIndexed { index, line ->
        if (index + 1 == err.line) {
            " >>" + "${index + 1} | ".padStart(6) + line
        } else {
            "${index + 1} | ".padStart(9) + line
        }
    }.let {
        val lineFrom = if (err.line - 3 >= 0) err.line - 3 else 0
        val lineTo = if (err.line + 2 < it.size) err.line + 2 else it.size
        it.subList(lineFrom, lineTo)
    }.fold("") { acc, msg -> "$acc\n$msg" }.also {
        println(err.toString() + it)
    }
}

fun printCodeWithLineNumber(code: String) {
    code.trimIndent().split('\n').mapIndexed { index, line ->
        println("${index + 1} |  ".padStart(7) + line)
    }
}

fun assertNoCompileErrors(code: String, func: (String) -> Unit) {
    try {
        func.invoke(code)
    } catch (exception: CompileErrorException) {
        println("\nCompile Errors: ")
        exception.errors.forEach {
            printCompileError(code.trimIndent(), it)
        }
        fail(exception)
    } catch (exception: InternalCompileException) {
        println("\nCompile Errors: ")
        printCompileError(
            code.trimIndent(),
            CompileError.error(exception.location!!, exception.message ?: exception.toString())
        )
        fail(exception)
    }
}

inline fun <reified R> Any.assertAs(): R {
    assert(this is R)
    return this as R
}
