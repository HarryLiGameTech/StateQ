package org.stateq.preprocessor

import java.nio.file.Path

abstract class Preprocessor(val sourcePath: Path) {
    abstract fun preprocess(): List<ModuleSource>
}

class CLikePreprocessor(sourcePath: Path) : Preprocessor(sourcePath) {
    override fun preprocess(): List<ModuleSource> {
        TODO("Not yet implemented")
    }
}

class PythonPreprocessor(sourcePath: Path) : Preprocessor(sourcePath) {
    override fun preprocess(): List<ModuleSource> {
        TODO("Not yet implemented")
    }
}
