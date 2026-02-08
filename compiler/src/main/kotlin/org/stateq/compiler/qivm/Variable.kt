package org.stateq.compiler.qivm

import org.stateq.parameter.Variable
import org.stateq.type.QuantumType
import org.stateq.util.Location

object ProgramContextVariable : Variable {
    override val ident: String = "ctx"
    override val type = QuantumType.ProgramContext
    override val location: Location? = null
    val list: List<Variable> by lazy { listOf(this) }
}
