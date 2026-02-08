package org.stateq.parser

import org.stateq.parameter.ClassicalVariable
import org.stateq.parameter.QuantumVariable
import org.stateq.util.privateField
import org.stateq.visitor.ClassicalVariableTable
import org.stateq.visitor.StateqVisitor
import org.stateq.visitor.QuantumVariableTable

fun StateqParser.getVisitor(): StateqVisitor {
    return this.privateField("visitor")
}

fun StateqParser.getClassicalVariableTable(): ClassicalVariableTable {
    return this.getVisitor().privateField("classicalVariableTable")
}

fun StateqParser.getQuantumVariableTable(): QuantumVariableTable {
    return this.getVisitor().privateField("quantumVariableTable")
}

fun StateqParser.enterMockScope() {
    this.getClassicalVariableTable().enterScope()
    this.getQuantumVariableTable().enterScope()
}

fun StateqParser.putClassicalVariable(variable: ClassicalVariable) {
    this.getClassicalVariableTable().put(variable.ident, variable)
}

fun StateqParser.putQuantumVariable(variable: QuantumVariable) {
    this.getQuantumVariableTable().put(variable.ident, variable)
}
