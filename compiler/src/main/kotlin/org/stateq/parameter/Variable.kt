package org.stateq.parameter

import org.stateq.type.Type
import org.stateq.util.OptionalLocatable

interface Variable : OptionalLocatable {
    val ident: String
    val type: Type
}
