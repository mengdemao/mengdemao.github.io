# antlr笔记


## 简介

> ANTLR是一款强大的语法分析器生成工具,用于读取、处理、执行和翻译结构化的文本或二进制文件.

类似于*flex/bison*,根据描述文件,自动生成词法语法分析器;
解析规则文件,生成解析源文件,与SDK组合编译生成可执行文件;

生成器可以支持的语言,但是下面我只会其中的几个语言:
因此暂时只进行下面几种语言的开发,下面描述开发的情况和进度

+ [ ] Python3
+ [ ] JavaScript
+ [ ] Go
+ [x] C++

## 安装说明

1. 下载[antlr](https://www.antlr.org/download/antlr-4.9.2-complete.jar)
2. 设置path和classpath
3. 编写相关脚本

## 语法设计

## 错误处理

## 解析器

## 测试程序

### antlr4编译器

```sh
#!/bin/sh
antlr4 Expr.g4
```

### 编译生成的java文件

```bash
$ javac *.java
```

### 运行编译的结果

```bash
$ grun Expr prog -tree
```

```txt
(prog (stat (expr (expr (expr 1) + (expr 2)) + (expr 3)) \r\n))
```

```shell
grun Expr prog -gui			
```

```sh
grun Expr prog -tokens
```

```txt
[@0,0:0='1',<INT>,1:0]
[@1,1:1='+',<'+'>,1:1]
[@2,2:2='2',<INT>,1:2]
[@3,3:3='+',<'+'>,1:3]
[@4,4:4='3',<INT>,1:4]
[@5,5:6='\r\n',<NEWLINE>,1:5]
[@6,7:6='<EOF>',<EOF>,2:0]
```

## antlr语法详解
### Hello
```antlr
// antlr4 Hello.g4
// javac *.java
// grun Hello r -gui
grammar Hello;              // 定义一个Hello的grammer
r  : 'hello' ID ;           // 开头是hello后面接着一个ID
ID : [a-z]+ ;               // ID由小写字母组成
WS : [ \t\r\n]+ -> skip ;   // 控制符清除
```
### ArrayInit
```antlr
// antlr4 ArrayInit.g4
// javac *.java
// grun ArrayInit init -gui
grammar ArrayInit;						// 定义一个ArrayInit的grammer
init    : '{' value (',' value)* '}' ; 	// 
value   : init							// 嵌套定义
        | INT							// 定义整数
        ;
INT     :   [0-9]+ ;
WS      :   [ \t\r\n]+ -> skip ;
```

()* --> 相当于扩展

### Expr
```antlr
// antlr4 Expr.g4
// javac *.java
// grun Expr prog -gui
grammar Expr;

prog : stat+;

stat: expr NEWLINE          # printExpr
    | ID '=' expr NEWLINE   # assign
    | NEWLINE               # blank
    ;

expr: expr op=('*'|'/') expr        # MulDiv
    | expr op=('+'|'-') expr        # AddSub
    | INT                           # int
    | ID                            # id
    | '('expr')'                  # parens
    ;

MUL     : '*' ; // assigns token name to '*' used above in grammar
DIV     : '/' ;
ADD     : '+' ;
SUB     : '-' ;
ID      : [a-zA-Z]+ ;
INT     : [0-9]+ ;
NEWLINE :'\r'? '\n' ;
WS      : [ \t]+ -> skip;
```
### json
> 在词法规则中那些不会被语法规则直接调用的词法规则可以用一个fragment关键字来标识，
> fragment标识的规则只能为其它词法规则提供基础
```antlr4
grammar JSON;	// 声明一个grammar

json
   : value		// 一个value候选
   ;

obj				// 对象类型
   : '{' pair (',' pair)* '}'
   | '{' '}'
   ;

pair
   : STRING ':' value
   ;

arr
   : '[' value (',' value)* ']'
   | '[' ']'
   ;

value
   : STRING
   | NUMBER
   | obj
   | arr
   | 'true'
   | 'false'
   | 'null'
   ;


STRING
   : '"' (ESC | SAFECODEPOINT)* '"'
   ;


fragment ESC
   : '\\' (["\\/bfnrt] | UNICODE)
   ;


fragment UNICODE
   : 'u' HEX HEX HEX HEX
   ;


fragment HEX
   : [0-9a-fA-F]
   ;


fragment SAFECODEPOINT
   : ~ ["\\\u0000-\u001F]
   ;


NUMBER
   : '-'? INT ('.' [0-9] +)? EXP?
   ;


fragment INT
   : '0' | [1-9] [0-9]*
   ;

// no leading zeros

fragment EXP
   : [Ee] [+\-]? INT
   ;

// \- since - means "range" inside [...]

WS
   : [ \t\n\r] + -> skip
   ;
```

测试例子
```json
{
    "glossary": {
        "title": "example glossary",
		"GlossDiv": {
            "title": "S",
			"GlossList": {
                "GlossEntry": {
                    "ID": "SGML",
					"SortAs": "SGML",
					"GlossTerm": "Standard Generalized Markup Language",
					"Acronym": "SGML",
					"Abbrev": "ISO 8879:1986",
					"GlossDef": {
                        "para": "A meta-markup language",
						"GlossSeeAlso": ["GML", "XML"]
                    },
					"GlossSee": "markup"
                }
            }
        }
    }
}
```
显示结果：
### XML
> 孤岛语法:

### dot
```antlr
grammar DOT;

graph
   : STRICT? ( GRAPH | DIGRAPH ) id_? '{' stmt_list '}'
   ;

stmt_list
   : ( stmt ';'? )*
   ;

stmt
   : node_stmt | edge_stmt | attr_stmt | id_ '=' id_ | subgraph
   ;

attr_stmt
   : ( GRAPH | NODE | EDGE ) attr_list
   ;

attr_list
   : ( '[' a_list? ']' )+
   ;

a_list
   : ( id_ ( '=' id_ )? ','? )+
   ;

edge_stmt
   : ( node_id | subgraph ) edgeRHS attr_list?
   ;

edgeRHS
   : ( edgeop ( node_id | subgraph ) )+
   ;

edgeop
   : '->' | '--'
   ;

node_stmt
   : node_id attr_list?
   ;

node_id
   : id_ port?
   ;

port
   : ':' id_ ( ':' id_ )?
   ;

subgraph
   : ( SUBGRAPH id_? )? '{' stmt_list '}'
   ;

id_
   : ID | STRING | HTML_STRING | NUMBER
   ;

// "The keywords node, edge, graph, digraph, subgraph, and strict are
// case-independent"

STRICT
   : [Ss] [Tt] [Rr] [Ii] [Cc] [Tt]
   ;


GRAPH
   : [Gg] [Rr] [Aa] [Pp] [Hh]
   ;


DIGRAPH
   : [Dd] [Ii] [Gg] [Rr] [Aa] [Pp] [Hh]
   ;


NODE
   : [Nn] [Oo] [Dd] [Ee]
   ;


EDGE
   : [Ee] [Dd] [Gg] [Ee]
   ;


SUBGRAPH
   : [Ss] [Uu] [Bb] [Gg] [Rr] [Aa] [Pp] [Hh]
   ;


/** "a numeral [-]?(.[0-9]+ | [0-9]+(.[0-9]*)? )" */ NUMBER
   : '-'? ( '.' DIGIT+ | DIGIT+ ( '.' DIGIT* )? )
   ;


fragment DIGIT
   : [0-9]
   ;


/** "any double-quoted string ("...") possibly containing escaped quotes" */ STRING
   : '"' ( '\\"' | . )*? '"'
   ;


/** "Any string of alphabetic ([a-zA-Z\200-\377]) characters, underscores
 *  ('_') or digits ([0-9]), not beginning with a digit"
 */ ID
   : LETTER ( LETTER | DIGIT )*
   ;


fragment LETTER
   : [a-zA-Z\u0080-\u00FF_]
   ;


/** "HTML strings, angle brackets must occur in matched pairs, and
 *  unescaped newlines are allowed."
 */ HTML_STRING
   : '<' ( TAG | ~ [<>] )* '>'
   ;


fragment TAG
   : '<' .*? '>'
   ;


COMMENT
   : '/*' .*? '*/' -> skip
   ;


LINE_COMMENT
   : '//' .*? '\r'? '\n' -> skip
   ;


/** "a '#' character is considered a line output from a C preprocessor (e.g.,
 *  # 34 to indicate line 34 ) and discarded"
 */ PREPROC
   : '#' ~[\r\n]* -> skip
   ;


WS
   : [ \t\n\r]+ -> skip
   ;
```

