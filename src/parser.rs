/*

example: 
- a             → regexp(literal)
- a*            → regexp(repetition(...))
- a | b         → regexp(altanative(...))
- a | b*        → regexp(altanative(literal, repetition(literal, quantifier)))
- a* | b*
- (a)*          → regexp(quantifier(regexp(literal)))
- a(a)*
- a | (b)*
- a | (b | c)*
- a | (b* | c)*  
regexp(
    altanative(
        literal, 
        repetition(
            regexp(
                altanative(
                    regex(
                        altanative(
                            literal,
                            quantifier
                        )
                    ),
                    literal
                )
            ),
            quantifier
        )
    )
)

- (a) | (b)


regexp := literal | repetition | altanative
repetition := literal quantifier | "(" regxep ")" quantifier
altanative := regexp "|" regexp
literal := char+
char: [a-zA-z0-9.]
quantifier = * | ?

↓ Chat GPTによる最適化

<regexp> ::= <term> ("|" <term>)*
<term> ::= <factor>+
<factor> ::= <base> <quantifier>? 
<base> ::= <literal> | "(" <regexp> ")"
<literal> ::= <char>+
<char> ::= [a-zA-Z0-9.] 
<quantifier> ::= "*" | "?"

↓ アイテム集合を求めるために、基本形に直して、さらに右再帰をなくした文法

<regexp> ::= <term> <term_list>

<term_list> ::= ε
<term_list> ::= <term_list> "|" <term>
<term> ::= <factor> <factor_list>

<factor_list> ::= ε
<factor_list> ::= <factor_list> <factor>
<factor> ::= <base> <quantifier?>

<quantifier?> ::= ε
<quantifier?> ::= <quantifier>

<base> ::= <literal>
<base> ::= "(" <regexp> ")"

<literal> ::= <char> <char_list>

<char_list> ::= ε
<char_list> ::= <char_list> <char>
<char> ::= [a-zA-Z0-9.]

<quantifier> ::= "*"
<quantifier> ::= "?"

LR(0)のアイテムセットを求める
1. 
<start> ::= .<regexp>
<regexp> ::= .<term> <term_list>
<term> ::= .<factor> <factor_list>
<factor> ::= .<base> <quantifier?>
<base> ::= .<literal>
<base> ::= ."(" <regexp> ")"
<literal> ::= .<char> <char_list>
<char> ::= .[a-zA-Z0-9.]

2. from (1)
<start> ::= <regexp>.

3. from (1)
<regexp> ::= <term>.<term_list>
<term_list> ::= .ε
<term_list> ::= .<term_list> "|" <term> 

4. from (1)
<term> ::= <factor> .<factor_list>
<factor_list> ::= .ε
<factor_list> ::= .<factor_list> <factor>

5. from (1)
<factor> ::= <base> .<quantifier?>
<quantifier?> ::= .ε
<quantifier?> ::= .<quantifier>

6. from (1)
<base> ::= <literal>.
<base> ::= "(" .<regexp> ")"
<regexp> ::= .<term> <term_list>
<term> ::= .<factor> <factor_list>
<factor> ::= .<base> <quantifier?>
<base> ::= .<literal>
<base> ::= ."(" <regexp> ")"
<literal> ::= .<char> <char_list>
<char> ::= .[a-zA-Z0-9.]

7. from (1)
<literal> ::= <char> .<char_list>
<char_list> ::= .ε
<char_list> ::= .<char_list> <char>

8. from (1)
<char> ::= [a-zA-Z0-9.].

9. from (1)
<quantifier> ::= "*".

10. from (1)
<quantifier> ::= "?".
*/