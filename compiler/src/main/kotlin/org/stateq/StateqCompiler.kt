package org.stateq

import org.stateq.compiler.CompiledModule
import org.stateq.compiler.ModuleCompiler
import org.stateq.compiler.language.CQivmCodeGenerator
import org.stateq.exception.CompileErrorException
import org.stateq.util.CompileError
import java.io.IOException
import java.nio.file.Path
import kotlin.io.path.Path
import kotlin.io.path.readText
import kotlin.io.path.writeText

object CompileErrorListener {
    private val compileErrors = mutableListOf<CompileError>()
    fun add(compileError: CompileError) = compileErrors.add(compileError)
    fun get(): List<CompileError> = compileErrors.toList().also { this.compileErrors.clear() }
}

data class CompileResult(val targets: List<String>, val errors: List<CompileError>)

fun compile(filePath: String, config: Map<String, String>): CompileResult {

    val sourcePath = Path(filePath)

    if (!filePath.endsWith(".qc")) {
        return CompileError.error(
            sourcePath,
            "`$filePath` is not invalid, a valid stateq source file name should be end with `.qc`"
        ).toCompileResult()
    }

    // val includePaths = config["includePaths"]?.split(";")?.trim() ?: listOf()
    val targetPaths = config["targets"]?.split(";")?.trim() ?: listOf()

    val targets = try {
        val path = Path(filePath)
        val code = try {
            path.readText()
        } catch (exception: IOException) {
            return CompileError.error(path, "Unable to read file $filePath").toCompileResult()
        }
        compileTargets(code, path, targetPaths.map(::CompileTarget))
    } catch (exception: CompileErrorException) {
        return (CompileErrorListener.get() + exception.errors).toCompileResult()
    }

    return CompileResult(targets, CompileErrorListener.get())
}

private fun CompileError.toCompileResult() =  CompileResult(listOf(), listOf(this))

private fun List<CompileError>.toCompileResult() =  CompileResult(listOf(), this)

fun List<String>.trim() = this.map { it.trim() }.filter { it.isNotBlank() }

private fun codeGenTarget(module: CompiledModule, target: CompileTarget): String {
    val codegen = when (target.language) {
        HostLanguage.C -> CQivmCodeGenerator()
        else -> TODO("not implemented yet")
    }
    return module.dumpCode(codegen)
}

private fun compileTargets(code: String, path: Path, targets: List<CompileTarget>): List<String> {
    val module = ModuleCompiler.compileModule(code, path)
    return targets.mapNotNull { target ->
        try {
            val targetCode = codeGenTarget(module, target)
            target.path.writeText(targetCode)
            target.path.toString()
        } catch (exception: IOException) {
            CompileErrorListener.add(CompileError.error(target.path,
                "Unable to write to file ${target.path}"
            ))
            return@mapNotNull null
        } catch (exception: Exception) {
            CompileErrorListener.add(CompileError.error(target.path,
                "Error occurred when generating code for ${target.path}"
            ))
            return@mapNotNull null
        }
    }
}

data class CompileTarget(val path: Path, val language: HostLanguage) {
    constructor(path: String) : this(Path(path), HostLanguage.fromFileName(path) ?:
        CompileError.error(Path(path), "Unable to determine language for target file $path").raise()
    )
}

enum class HostLanguage(val extensions: List<String>) {
    C("c"),
    Cpp("cpp", "cxx", "cc", "h", "hpp", "hxx", "hh"),
    Python("py"),
    Java("java"),
    Rust("rs");

    constructor(vararg extensions: String): this(extensions.toList())

    companion object {
        fun fromFileName(name: String): HostLanguage? {
            for (language in values()) {
                if (language.extensions.any { name.endsWith(it) }) {
                    return language
                }
            }
            return null
        }
    }
}
