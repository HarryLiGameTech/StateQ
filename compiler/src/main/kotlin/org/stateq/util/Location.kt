package org.stateq.util

import java.nio.file.Path

data class Location(
    val source: Path?,
    val line: Int,
    val column: Int,
) {
    companion object {
        val builtin = Location(null, 0, 0)
    }
}

interface Locatable : OptionalLocatable {
    override val location: Location
}

interface OptionalLocatable {
    val location: Location?
}
