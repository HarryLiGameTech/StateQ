package org.stateq.type

interface ClassicalTrait

interface BoolTrait : ClassicalTrait
interface NumericTrait : ClassicalTrait
interface IntTrait : NumericTrait
interface FloatTrait : NumericTrait
interface ComplexTrait : ClassicalTrait
interface BitsTrait : ClassicalTrait
interface ListTrait : ClassicalTrait

interface QuantumTrait

interface QvarTrait : QuantumTrait
interface QrefTrait : QuantumTrait
