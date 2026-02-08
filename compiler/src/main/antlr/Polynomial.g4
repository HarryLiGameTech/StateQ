grammar Polynomial;

polynomial
    :   (terms+=monomial '+')* terms+=monomial
    ;

monomial
    :   coeff=IntLiteral '*' (terms+=monomialTerm '*')* terms+=monomialTerm     # monomialWithCoeff
    |   (negative='-')? (terms+=monomialTerm '*')* terms+=monomialTerm          # monomialWithNoCoeff
    |   value=IntLiteral                                                        # constant
    ;

monomialTerm
    :   indeterminate=Indeterminate ('^' exponent=IntLiteral)?
    ;

IntLiteral          :   '-'?[0-9]+;
Indeterminate       :   [a-zA-Z][a-zA-Z0-9]*;
Whitespaces         :   ' ' -> skip;
