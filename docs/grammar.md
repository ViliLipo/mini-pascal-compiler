```
<program> ::= "program"<id>";" {<procedure> | <function>}<block>"."
<procedure> ::= <id>"("<parameters>")"";"<block>";"
<var-declaration> ::= "var"<id>{"," <id>} ":" <type>
<parameters> ::= ["var"]<id> ":" <type> { "," ["var"]<id> ":" <type> } | <empty>
<type> ::= <simple type> | <array type>
<simple type> ::= <type id>
<array type> ::= "array" "["[<expr>]"]" "of" <simple type>
<block> ::= "begin" <statement> {";"<statement>} [";"] "end"
<statement> ::= <simple statement> | <structured statement> | <var-declaration>
<simple statement> ::= <assignment_or_call> | <return statement>
<assignment_or_call> ::= <id> (["["<integer expr>"]"] ":=" <expr> | "("<arguments>")")
<arguments> ::= <expr> {"," expr } | <empty>
<return statement> ::= "return" [<expr>]
<structured statement> ::= <block> | <if statement> | <whle statement>
<if statement> :== "if" <expr> "then" <statement> [ "else" <statement"]
<while statement> ::= "while" <expr> "do" <statement>
<expr> ::= <simple expr> [ <relational operator> <simple expr> ]
<simple expr> ::= [<sign>] <term> { <adding operator> <term> }
<term> ::= <factor> {<multiplying operator> <factor>}
<factor> ::= <call> | <variable_factor> | <literal> | "("<expr>")" | "not" <factor>
<variable_factor> ::= <id>["["<expr>"]" | "("<arguments>")" | ".""size"]

```
