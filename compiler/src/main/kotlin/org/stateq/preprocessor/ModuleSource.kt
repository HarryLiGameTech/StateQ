package org.stateq.preprocessor

import org.stateq.HostLanguage

data class ModuleSource(
    val hostLanguage: HostLanguage,
    val code: String,
    val sourcePath: String,
    val line: UInt,
    val column: UInt
)
