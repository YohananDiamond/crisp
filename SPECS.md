# Language Specs

```
expression: '(' ( ( command args* ) | expression ) ')'
command: expression | identifier | literal
args: expression | identifier | literal
identifier: [aA-zZ][aA-zZ0-9]*
literal: int_literal | float_literal | string_literal
int_literal: [0-9]+
float_literal: [0-9].[0-9]+
string_literal: '"' [^"] '"'
```
