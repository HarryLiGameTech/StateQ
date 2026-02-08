grammar Stateq;

stateqCode
    :   moduleImports module
    ;

moduleImports
    :   ('import' modulePaths+=modulePath ';')*
    ;

modulePath
    :   (ident=CamelIdent '::')* ident=CamelIdent
    ;

module
    :   (
            constants+=constantDef |
            externs+=externFuncDef |
            gates+=gateDef |
            operations+=operationDef |
            programs+=programDef
        )+
    ;

nameAnnotation
    :   '#' 'ident' '(' ident=(PascalIdent | CamelIdent | AsciiIdentifier) ')'
    ;

gateDef
    :   (rename=nameAnnotation)?
        'gate' ident=PascalIdent '[' (params=classicalParamList ('|' init=classicalInitParamList )?)? ']'
        expr=operationExpr
    ;

programDef
    :   (rename=nameAnnotation)?
        'program' ident=PascalIdent '[' (params=classicalParamList ('|' init=classicalInitParamList )?)? ']'
        'shot' '(' shots=intExpr ')' body=statementsBlock
    ;

operationDef
    :   (rename=nameAnnotation)?
        'operation' ident=PascalIdent ('[' classicalParams=classicalParamList? ']')? '(' quantumParams=quantumParamList ')' body=statementsBlock
    ;

externFuncDef
    :   (rename=nameAnnotation)?
        'extern' 'func' ident=CamelIdent '(' ((CamelIdent ':')? paramTypes+=classicalTypeIdent ',')* ((CamelIdent ':')? paramTypes+=classicalTypeIdent ','?)? ')'
         ('=>' returnType=classicalTypeIdent )?';'
    ;

quantumParamList
    :   ((params+=qvarParam ',')* params+=qvarParam)?
    ;

classicalInitParamList
    :   (params+=classicalInitParam ',')* params+=classicalInitParam
    ;

classicalInitParam
    :   ident=CamelIdent (':' type=classicalTypeIdent)? '=' assignment=classicalExpr
    ;

statementsBlock
    :   '{' (statements+=statement)+ '}'
    ;

listComprehension
    :   ident=CamelIdent 'in' expr=classicalExpr
    ;

statement
    :   expr=qvarExpr 'as' qvar=qvarIdent ';'                                                   # qvarOperateAsStatement
    |   ('each' (compre+=listComprehension '&')* compre+=listComprehension '|')?
            expr=qvarExpr ('ctrl' ctrl=qrefExpr)? ';'                                           # qvarCompreOperateStatement
    |   expr=qvarExpr ';'                                                                       # qvarOperateStatement
    |   'measure' (slice=slicing)? expr=qvarExpr ';'                                            # qvarMeasurementStatement
    |   'if' cond=boolExpr ifBranch=statementsBlock ('else' elseBranch=statementsBlock)?        # cifStatement
    |   'qif' ctrl=qrefExpr ifBranch=statementsBlock ('else' elseBranch=statementsBlock)?       # qifStatement
    // |   'while' cond=boolExpr loopBody=statementsBlock                                          # whileStatement
    |   'for' iterator=CamelIdent 'in' iterable=iterableExpr loopBody=statementsBlock           # forStatement
    |   'with' expr=qvarExpr body=statementsBlock                                               # withStatement
    |   'let' ident=CamelIdent (':' type=classicalTypeIdent)? '=' value=classicalExpr ';'       # letStatement
    ;

qvarIdent
    :   '$' ident=CamelIdent;

qrefIdent
    :   '&' ident=CamelIdent;

slicing
    // [start : end by step]
    :   '[' (start=intExpr)? endGuard=':' (end=intExpr)? (stepGuard='by' step=intExpr)? ']'   # slicingExclusive
    // [start .. end by step]
    |   '[' (start=intExpr)? endGuard='..' (end=intExpr)? (stepGuard='by' step=intExpr)? ']'  # slicingInclusive
    ;

qvarInit
    :   '|' literal=IntLiteral ('>' | '⟩')                      # qvarInitBinaryLiteral
    |   '|' value=intExpr '\'' size=intExpr ('>' | '⟩')         # qvarInitClassicalExpr
    ;

atomicOperation
    :   ident=PascalIdent ('[' argList=classicalArgList ']')?
    ;

operationExpr
    :   op=atomicOperation                                          # operationExprElementary
    |   op=unitaryMatrix                                            # operationExprMatrix
    |   '(' operations+=operationExpr+ ')'                          # operationExprMatMul
    |   op=operationExpr '!'                                        # operationExprDagger
    |   op=operationExpr '@' exponent=intExpr                       # operationExprExtended
    |   coefficient=complexExpr '*' op=operationExpr                # operationExprCoefficient
    |   operations+=operationExpr ('.' operations+=operationExpr)+  # operationExprCombined
    |   'do' ('(' paramList=quantumParamList ')')?
            (singleStatement=statement | statemens=statementsBlock) # operationExprAnonymous
    ;

unitaryMatrix
    :   'unitary' '(' rows+=matrixRow (',' rows+=matrixRow)* ','? ')'
    ;

matrixRow
    :   '[' elements+=complexExpr (',' elements+=complexExpr)* ','? ']'
    ;

classicalExpr
    :   ident=CamelIdent    # classicalExprUndeterminedVariable
    |   boolExpr            # classicalExprBool
    |   numericExpr         # classicalExprNumeric
    |   intExpr             # classicalExprInt
    |   floatExpr           # classicalExprFloat
    |   complexExpr         # classicalExprComplex
    |   listExpr            # classicalExprList
    ;

classicalArgList
    :   ( (args+=classicalExpr ',')* args+=classicalExpr ','? )?
    ;

qvarParam
    :   (qvar=qvarIdent | qref=qrefIdent) (':' size=qvarSize)?
    ;

qvarSize
    :   value=intExpr               # staticQvarSize
    |   sizeExpr=qvarSizeAutoExpr   # autoInferQvarSize
    ;

qvarSizeAutoExpr
    :   '?' ident=CamelIdent                                        # qvarSizeAutoExprVariable
    |   value=IntLiteral                                            # qvarSizeAutoExprValue
    |   '(' sizeExpr=qvarSizeAutoExpr ')'                           # qvarSizeAutoExprQuoted
    |   lhs=qvarSizeAutoExpr opt=('+' | '-') rhs=qvarSizeAutoExpr   # qvarSizeAutoExprAddSub
    ;

constantDef
    :   'const' ident=CamelIdent ':' type=classicalTypeIdent '=' value=classicalExpr ';'
    ;

qrefExpr
    :   qref=qrefIdent                                                      # qrefExprIdent
    |   qref=qrefIdent '[' index=intExpr ']'                                # qrefExprIndexing
    |   qref=qrefIdent '[' indexes+=intExpr (',' indexes+=intExpr)+ ']'     # qrefExprMultiIndexing
    |   qref=qrefIdent slice=slicing                                        # qrefExprSlicing
    |   '(' expr=qrefExpr ')'                                               # qrefExprQuoted
    |   lhs=qrefExpr 'and' rhs=qrefExpr                                     # qrefExprAnd
    |   lhs=qrefExpr 'or' rhs=qrefExpr                                      # qrefExprOr
    |   '!' expr=qrefExpr                                                   # qrefExprNot
    ;

qvarExpr
    :   '$'                                                                         # qvarExprDefault
    |   qvar=qvarIdent                                                              # qvarExprIdent
    |   init=qvarInit                                                               # qvarExprInit
    |   expr=qvarExpr '[' index=intExpr ']'                                         # qvarExprIndexing
    |   expr=qvarExpr '[' indexes+=intExpr (',' indexes+=intExpr)+ ']'              # qvarExprMultiIndexing
    |   expr=qvarExpr slice=slicing                                                 # qvarExprSlicing
    |   '(' expr=qvarExpr ')'                                                       # qvarExprQuoted
    |   qvarExprs+=qvarExpr ('.' qvarExprs+=qvarExpr)+                              # qvarExprConcate
    |   op=atomicOperation targets=qvarArgs                                         # qvarExprMultiTargetsOperation
    |   op=operationExpr target=qvarExpr                                            # qvarExprOperation
    // |   'if' cond=boolExpr 'then' ifBranch=qvarExpr ('else' elseBranch=qvarExpr)?   # qvarExprCif
    // |   'qif' ctrl=qrefExpr 'then' ifBranch=qvarExpr ('else' elseBranch=qvarExpr)?  # qvarExprQif
    |   target=qvarExpr '->' op=operationExpr                                       # qvarExprOperationReversed
    ;

qvarArgs
    :   '('(exprs+=qvarExpr ',')* exprs+=qvarExpr ')'
    ;

boolExpr
    :   ident=CamelIdent                            # boolExprIdent
    |   value=BoolLiteral                           # boolExprLiteral
    |   '(' expr=boolExpr ')'                       # boolExprQuoted
    |   '!' expr=boolExpr                           # boolExprNot
    |   lhs=boolExpr '&&' rhs=boolExpr              # boolExprAnd
    |   lhs=boolExpr '||' rhs=boolExpr              # boolExprOr
    |   lhs=intExpr opt=('==' | '!=' | '>' | '<' | '>=' | '<=') rhs=intExpr
                                                    # boolExprIntComparison
    ;

numericExpr
    :   ident=CamelIdent                                            # numericExprIdent
    |   value=FloatLiteral                                          # numericExprFloatLiteral
    |   value=IntLiteral                                            # numericExprIntLiteral
    |   '(' expr=numericExpr ')'                                    # numericExprQuoted
    |   '-' expr=numericExpr                                        # numericExprNegative
    |   lhs=numericExpr opt=('&' | '^' | '|') rhs=numericExpr       # numericExprBitwise
    |   base=numericExpr opt='**' exponent=numericExpr              # numericExprPow
    |   lhs=numericExpr opt=('*' | '/' | '%') rhs=numericExpr       # numericExprMulDivMod
    |   lhs=numericExpr opt=('+' | '-') rhs=numericExpr             # numericExprAddSub
    |   lhs=numericExpr opt=('<<' | '>>' | '>>>') rhs=numericExpr   # numericExprShift
    |   func=CamelIdent '(' argList=classicalArgList ')'            # numericExprFuncCall
    ;

intExpr
    :   ident=CamelIdent                                    # intExprIdent
    |   value=IntLiteral                                    # intExprLiteral
    |   '(' expr=intExpr ')'                                # intExprQuoted
    |   '-' expr=intExpr                                    # intExprNegative
    |   lhs=intExpr opt=('&' | '^' | '|') rhs=intExpr       # intExprBitwise
    |   base=intExpr opt='**' exponent=intExpr              # intExprPow
    |   lhs=intExpr opt=('*' | '/' | '%') rhs=intExpr       # intExprMulDivMod
    |   lhs=intExpr opt=('+' | '-') rhs=intExpr             # intExprAddSub
    |   lhs=intExpr opt=('<<' | '>>' | '>>>') rhs=intExpr   # intExprShift
    |   func=CamelIdent '(' argList=classicalArgList ')'    # intExprFuncCall
    ;

floatExpr
    :   ident=CamelIdent                                                        # floatExprIdent
    |   value=FloatLiteral                                                      # floatExprLiteral
    |   value=IntLiteral                                                        # floatExprIntLiteral
    |   '(' expr=floatExpr ')'                                                  # floatExprQuoted
    |   base=floatExpr opt='**' (expFloat=floatExpr | expInt=intExpr)           # floatExprPow
    |   lhs=floatExpr opt=('*' | '/') (rhsFloat=floatExpr | rhsInt=intExpr)     # floatExprMulDiv
    |   lhs=floatExpr opt=('+' | '-') (rhsFloat=floatExpr | rhsInt=intExpr)     # floatExprAddSub
    |   'Float' '(' expr=intExpr ')'                                            # floatExprIntToFloat
    |   func=CamelIdent '(' argList=classicalArgList ')'                        # floatExprFuncCall
    ;

complexExpr
    :   im=ImaginaryLiteral                                 # complexExprImLiteral
    |   intIm=intExpr '\'i'                                 # complexExprIntIm
    |   floatIm=floatExpr '\'i'                             # complexExprFloatIm
    |   intRe=intExpr                                       # complexExprIntRe
    |   floatRe=floatExpr                                   # complexExprFloatRe
    |   '(' expr=complexExpr ')'                            # complexExprQuoted
    |   lhs=complexExpr '**' rhs=complexExpr                # complexExprPower
    |   lhs=complexExpr op=('*'|'/') rhs=complexExpr        # complexExprMulDiv
    |   lhs=complexExpr op=('+'|'-') rhs=complexExpr        # complexExprAddSub
    |   func=CamelIdent '(' argList=classicalArgList ')'    # complexExprFuncCall
    ;

bitsExpr
    :   ident=CamelIdent                                    # bitsExprIdent
    |   lhs=bitsExpr '.' rhs=bitsExpr                       # bitsExprConcate
    |   lhs=bitsExpr opt=('&' | '^' | '|') rhs=bitsExpr     # bitsExprBitwise
    |   func=CamelIdent '(' argList=classicalArgList ')'    # bitsExprFuncCall
    ;

listExpr
    :   ident=CamelIdent                                                        # listExprIdent
    |   (typeIdent=classicalTypeIdent)? '['
            ( (elements+=classicalExpr ',')* elements+=classicalExpr ','? )?
        ']'                                                                     # listExprLiteral
    |   generator=classicalIntListGenerator                                     # listExprIntGenerator
    ;

iterableExpr
    :   ident=CamelIdent            # iterableExprUndeterminedVariable
    |   list=listExpr               # iterableExprList
    |   bits=bitsExpr               # iterableExprBits
    ;

classicalParamList  // without '[' and ']'
    :   (params+=classicalParam ',' )* params+=classicalParam
    ;

classicalTypeIdent
    :   classicalBasicTypeIdent     # classicalTypeIdentBasic
    |   '[' classicalTypeIdent ']'  # classicalTypeIdentList
    ;

classicalBasicTypeIdent
    :   'Int'
    |   'Float'
    |   'Complex'
    |   'Bits'
    |   'Bool'
    |   'Mat'
    ;

classicalParam
    :   ident=CamelIdent ':' type=classicalTypeIdent
    ;

classicalIntListGenerator
    :   '[' (start=intExpr)? (inclusive='..' | exclusive=':') end=intExpr ('by' step=intExpr)? ']'
    ;

PascalIdent         :   [A-Z][a-zA-Z0-9]*;
CamelIdent          :   [a-z][a-zA-Z_0-9]*;

AsciiIdentifier     :   [a-zA-Z][a-zA-Z0-9_]*;
IntLiteral          :   ('0x' | '0b')? [0-9]+;
FloatLiteral        :   [0-9]+'.'[0-9]+'f'? | [0-9]'f';
ImaginaryLiteral    :   [0-9]+('.'[0-9]+)?'i';
BoolLiteral         :   'true' | 'false';
BinaryIntLiteral    :   [01]+;

//StringLiteral       :   '"' (StringCharacter)* '"';

Whitespaces         :   [ \t\n\r] -> skip;

BlockComment
    :   '/*' .*? '*/' -> skip
    ;

LineComment
    :   '//' ~[\r\n]* -> skip
    ;

fragment
StringCharacter
    :	~["\\\r\n]
    |   EscapeSequence
    ;

fragment
EscapeSequence      :	'\\' [btnfr"'\\];
