# Lua笔记


**Lua运行结构**

{{< mermaid >}}
  graph LR
  编译器==>PROTO==>虚拟机
{{< /mermaid >}}

## 相关数据结构

### 全局状态机

> 描述Lua运行状态,同时可以产生一种面向对象的模拟

```c
struct lua_State {
  CommonHeader;
  lu_byte status;
  lu_byte allowhook;
  unsigned short nci;  /* number of items in 'ci' list */
  StkId top;  /* first free slot in the stack */
  global_State *l_G;
  CallInfo *ci;  /* call info for current function */
  StkId stack_last;  /* end of stack (last element + 1) */
  StkId stack;  /* stack base */
  UpVal *openupval;  /* list of open upvalues in this stack */
  StkId tbclist;  /* list of to-be-closed variables */
  GCObject *gclist;
  struct lua_State *twups;  /* list of threads with open upvalues */
  struct lua_longjmp *errorJmp;  /* current error recover point */
  CallInfo base_ci;  /* CallInfo for first level (C calling Lua) */
  volatile lua_Hook hook;
  ptrdiff_t errfunc;  /* current error handling function (stack index) */
  l_uint32 nCcalls;  /* number of nested (non-yieldable | C)  calls */
  int oldpc;  /* last pc traced */
  int basehookcount;
  int hookcount;
  volatile l_signalT hookmask;
};
```

![lua_state](https://raw.githubusercontent.com/mengdemao/picture/master/lua_state.png)

## Lua编译器

> lua并没有直接运行源文件,而是将源文件编译成字节码，然后运行字节码
> 其中运行编译驱动文件名`luac.c`

| 文件名              | 功能     | 入口函数                  |
| ------------------- | -------- | ------------------------- |
| `llex.c`            | 词法分析 | `luaX_next`               |
| `lparser.c`         | 语法解析 | `luaY_parser`             |
| `lcode.c`           | 代码生成 | ` luaK_finish `           |
| `ldump.c/lundump.c` | 字节码   | ` luaU_dump/luaU_undump ` |

### 词法分析

#### 外部接口
```c
// 读取下一个单词
void luaX_next (LexState *ls);

// 预读下一个单词
int luaX_lookahead (LexState *ls);
```

#### 核心函数
```c
// 词法分析状态机
typedef struct LexState {
  int current;  /* current character (charint) */
  int linenumber;  /* input line counter */
  int lastline;  /* line of last token 'consumed' */
  Token t;  /* current token */
  Token lookahead;  /* look ahead token */
  struct FuncState *fs;  /* current function (parser) */
  struct lua_State *L;
  ZIO *z;  /* input stream */
  Mbuffer *buff;  /* buffer for tokens */
  Table *h;  /* to avoid collection/reuse strings */
  struct Dyndata *dyd;  /* dynamic structures used by the parser */
  TString *source;  /* current source name */
  TString *envn;  /* environment variable name */
} LexState;

// 语义信息
typedef union {
  lua_Number r;
  lua_Integer i;
  TString *ts;
} SemInfo;

int llex (LexState *ls, SemInfo *seminfo);
```

### 语法分析

```c
typedef struct Proto {
  CommonHeader;
  lu_byte numparams;  /* number of fixed (named) parameters */
  lu_byte is_vararg;
  lu_byte maxstacksize;  /* number of registers needed by this function */
  int sizeupvalues;  /* size of 'upvalues' */
  int sizek;  /* size of 'k' */
  int sizecode;
  int sizelineinfo;
  int sizep;  /* size of 'p' */
  int sizelocvars;
  int sizeabslineinfo;  /* size of 'abslineinfo' */
  int linedefined;  /* debug information  */
  int lastlinedefined;  /* debug information  */
  TValue *k;  /* constants used by the function */
  Instruction *code;  /* opcodes */
  struct Proto **p;  /* functions defined inside the function */
  Upvaldesc *upvalues;  /* upvalue information */
  ls_byte *lineinfo;  /* information about source lines (debug information) */
  AbsLineInfo *abslineinfo;  /* idem */
  LocVar *locvars;  /* information about local variables (debug information) */
  TString  *source;  /* used for debug information */
  GCObject *gclist;
} Proto;
typedef struct LClosure {
  ClosureHeader;
  struct Proto *p;
  UpVal *upvals[1];  /* list of upvalues */
} LClosure;

LClosure *luaY_parser (
    lua_State *L,
    ZIO *z,
    Mbuffer *buff,
    Dyndata *dyd,
    const char *name,
    int firstchar);
```

![lparser](https://raw.githubusercontent.com/mengdemao/picture/master/lparser.png)

### 代码生成

```c
void luaK_finish (FuncState *fs) {
  int i;
  Proto *p = fs->f;
  for (i = 0; i < fs->pc; i++) {
    Instruction *pc = &p->code[i];
    lua_assert(i == 0 || isOT(*(pc - 1)) == isIT(*pc));
    switch (GET_OPCODE(*pc)) {
      case OP_RETURN0: case OP_RETURN1: {
        if (!(fs->needclose || p->is_vararg))
          break;  /* no extra work */
        /* else use OP_RETURN to do the extra work */
        SET_OPCODE(*pc, OP_RETURN);
      }  /* FALLTHROUGH */
      case OP_RETURN: case OP_TAILCALL: {
        if (fs->needclose)
          SETARG_k(*pc, 1);  /* signal that it needs to close */
        if (p->is_vararg)
          SETARG_C(*pc, p->numparams + 1);  /* signal that it is vararg */
        break;
      }
      case OP_JMP: {
        int target = finaltarget(p->code, i);
        fixjump(fs, i, target);
        break;
      }
      default: break;
    }
  }
}
```

### 字节码

```c
int luaU_dump(lua_State *L, const Proto *f, lua_Writer w, void *data,
              int strip) {
  DumpState D;
  D.L = L;
  D.writer = w;
  D.data = data;
  D.strip = strip;
  D.status = 0;
  dumpHeader(&D);
  dumpByte(&D, f->sizeupvalues);
  dumpFunction(&D, f, NULL);
  return D.status;
}
LClosure *luaU_undump(lua_State *L, ZIO *Z, const char *name) {
  LoadState S;
  LClosure *cl;
  if (*name == '@' || *name == '=')
    S.name = name + 1;
  else if (*name == LUA_SIGNATURE[0])
    S.name = "binary string";
  else
    S.name = name;
  S.L = L;
  S.Z = Z;
  checkHeader(&S);
  cl = luaF_newLclosure(L, loadByte(&S));
  setclLvalue2s(L, L->top, cl);
  luaD_inctop(L);
  cl->p = luaF_newproto(L);
  luaC_objbarrier(L, cl, cl->p);
  loadFunction(&S, cl->p, NULL);
  lua_assert(cl->nupvalues == cl->p->sizeupvalues);
  luai_verifycode(L, cl->p);
  return cl;
}
```

## Lua虚拟机
> 接受编译器产生的字节码并且运行,执行的入口函数是`luaV_execute`

### Lua指令分析

| `指令名`       | 参数  | 描述                                                                                            |
| -------------- | ----- | ----------------------------------------------------------------------------------------------- |
| `OP_MOVE`      | A B   | `R(A) := R(B)`                                                                                  |
| `OP_LOADK`     | A Bx  | `R(A) := Kst(Bx)`                                                                               |
| `OP_LOADBOOL`  | A B C | `R(A) := (Bool)B; if (C) pc++`                                                                  |
| `OP_LOADNIL`   | A B   | `R(A) := ... := R(B) := nil`                                                                    |
| `OP_GETUPVAL`  | A B   | `R(A) := UpValue[B]`                                                                            |
| `OP_GETGLOBAL` | A Bx  | `R(A) := Gbl[Kst(Bx)]`                                                                          |
| `OP_GETTABLE`  | A B C | `R(A) := R(B)[RK(C)]`                                                                           |
| `OP_SETGLOBAL` | A Bx  | `Gbl[Kst(Bx)] := R(A)`                                                                          |
| `OP_SETUPVAL`  | A B   | `UpValue[B] := R(A)`                                                                            |
| `OP_SETTABLE`  | A B C | `R(A)[RK(B)] := RK(C)                                                                           |
| `OP_NEWTABLE`  | A B C | `R(A) := {} (size = B,C)`                                                                       |
| `OP_SELF`      | A B C | `R(A+1) := R(B); R(A) := R(B)[RK(C)]`                                                           |
| `OP_ADD`       | A B C | `R(A) := RK(B) + RK(C)`                                                                         |
| `OP_SUB`       | A B C | `R(A) := RK(B) - RK(C)`                                                                         |
| `OP_MUL`       | A B C | `R(A) := RK(B) * RK(C)`                                                                         |
| `OP_DIV`       | A B C | `R(A) := RK(B) / RK(C)`                                                                         |
| `OP_MOD`       | A B C | `R(A) := RK(B) % RK(C)`                                                                         |
| `OP_POW`       | A B C | `R(A) := RK(B) ^ RK(C)`                                                                         |
| `OP_UNM`       | A B   | `R(A) := -R(B)`                                                                                 |
| `OP_NOT`       | A B   | `R(A) := not R(B)`                                                                              |
| `OP_LEN`       | A B   | `R(A) := length of R(B)`                                                                        |
| `OP_CONCAT`    | A B C | `R(A) := R(B).. ... ..R(C)`                                                                     |
| `OP_JMP`       | sBx   | `pc+=sBx`                                                                                       |
| `OP_EQ`        | A B C | `if ((RK(B) == RK(C)) ~= A) then pc++`                                                          |
| `OP_LT`        | A B C | `if ((RK(B) <  RK(C)) ~= A) then pc++`                                                          |
| `OP_LE`        | A B C | `if ((RK(B) <= RK(C)) ~= A) then pc++`                                                          |
| `OP_TEST`      | A C   | `if not (R(A) <=> C) then pc++`                                                                 |
| `OP_TESTSET`   | A B C | `if (R(B) <=> C) then R(A) := R(B) else pc++`                                                   |
| `OP_CALL`      | A B C | `R(A), ... ,R(A+C-2) := R(A)(R(A+1), ... ,R(A+B-1)) `                                           |
| `OP_TAILCALL`  | A B C | `return R(A)(R(A+1), ... ,R(A+B-1))`                                                            |
| `OP_RETURN`    | A B   | `return R(A), ... ,R(A+B-2)	(see note)`                                                         |
| `OP_FORLOOP`   | A sBx | `R(A)+=R(A+2); if R(A) <?= R(A+1) then { pc+=sBx; R(A+3)=R(A) }`                                |
| `OP_FORPREP`   | A sBx | `R(A)-=R(A+2); pc+=sBx`                                                                         |
| `OP_TFORLOOP`  | A C   | `R(A+3), ... ,R(A+2+C) := R(A)(R(A+1), R(A+2));  if R(A+3) ~= nil then R(A+2)=R(A+3) else pc++` |
| `OP_SETLIST`   | A B C | `R(A)[(C-1)*FPF+i] := R(A+i), 1 <= i <= B`                                                      |
| `OP_CLOSE`     | A     | `close all variables in the stack up to (>=) R(A)`                                              |
| `OP_CLOSURE`   | A Bx  | `R(A) := closure(KPROTO[Bx], R(A), ... ,R(A+n))`                                                |
| `OP_VARARG`    | A B   | `R(A), R(A+1), ..., R(A+B-1) = vararg`                                                          |

### 指令解析

```c
lua_State *L;										/* Lua状态机 */
LClosure *cl          	= &clvalue(L->ci->func)->l;	/* 当前函数执行环境 */
TValue *k    			= cl->p->k;					/* 函数环境常量数组 */
const Instruction *pc 	= L->savedpc; 				/* 当前函数指针 */
StkId base 				= L->base;;					/* 函数环境栈基地址 */
```

```c
// R(A|B|C) 寄存器索引
#define RA(i)	(base+GETARG_A(i))
#define RB(i)	check_exp(getBMode(GET_OPCODE(i)) == OpArgR, base+GETARG_B(i))
#define RC(i)	check_exp(getCMode(GET_OPCODE(i)) == OpArgR, base+GETARG_C(i))

// RKB() 寄存器索引或者常量索引
#define RKB(i)	check_exp(getBMode(GET_OPCODE(i)) == OpArgK, \
	ISK(GETARG_B(i)) ? k+INDEXK(GETARG_B(i)) : base+GETARG_B(i))
#define RKC(i)	check_exp(getCMode(GET_OPCODE(i)) == OpArgK, \
	ISK(GETARG_C(i)) ? k+INDEXK(GETARG_C(i)) : base+GETARG_C(i))

#define KBx(i)	check_exp(getBMode(GET_OPCODE(i)) == OpArgK, k+GETARG_Bx(i))
```

#### 指令分发

```c
const Instruction i = *pc++;
StkId ra = RA(i);
```

#### 钩子函数处理
```c
// 不明白在处理什么
if ((L->hookmask & (LUA_MASKLINE | LUA_MASKCOUNT)) &&
    (--L->hookcount == 0 || L->hookmask & LUA_MASKLINE)) {
    traceexec(L, pc);
    if (L->status == LUA_YIELD) {  /* did hook yield? */
    	L->savedpc = pc - 1;
        return;
    }
    base = L->base;
}
```

#### `OP_MOVE`

```c
#define setobjs2s	setobj
case OP_MOVE: {
	setobjs2s(L, ra, RB(i));
    continue;
}
```

#### `OP_LOADK`

```c
#define setobj2s	setobj
case OP_LOADK: {
	setobj2s(L, ra, KBx(i));
    continue;
}
```

统一调用此函数
```c
// 拷贝lua_TValue,也只有两个成员
void setobj(lua_State *L, const TValue *obj1, TValue *obj2)
{
    const TValue *o2=(obj2);
    	  TValue *o1=(obj1);

    o1->value = o2->value;
    o1->tt=o2->tt;

    checkliveness(G(L),o1);
}
```

#### `OP_LOADBOOL`
```c
case OP_LOADBOOL: {
	setbvalue(ra, GETARG_B(i));
	if (GETARG_C(i))
		pc++;  /* skip next instruction (if C) */
	continue;
}
```

#### `OP_LOADNIL`
```c
case OP_LOADNIL: {
	TValue *rb = RB(i);
	do {
		setnilvalue(rb--);
	} while (rb >= ra);
	continue;
}
```

#### `OP_GETUPVAL`
```c
case OP_GETUPVAL: {
	int b = GETARG_B(i);
	setobj2s(L, ra, cl->upvals[b]->v);
	continue;
}
```

```c
case OP_GETGLOBAL: {
	TValue g;
	TValue *rb = KBx(i);
	sethvalue(L, &g, cl->env);
	lua_assert(ttisstring(rb));
	Protect(luaV_gettable(L, &g, rb, ra));
	continue;
}
```

```c
case OP_GETTABLE: {
	Protect(luaV_gettable(L, RB(i), RKC(i), ra));
	continue;
}
```

```c
case OP_SETGLOBAL: {
	TValue g;
	sethvalue(L, &g, cl->env);
	lua_assert(ttisstring(KBx(i)));
	Protect(luaV_settable(L, &g, KBx(i), ra));
	continue;
}
```

```c
case OP_SETUPVAL: {
	UpVal *uv = cl->upvals[GETARG_B(i)];
	setobj(L, uv->v, ra);
	luaC_barrier(L, uv, ra);
	continue;
}
```

```c
case OP_SETTABLE: {
	Protect(luaV_settable(L, ra, RKB(i), RKC(i)));
	continue;
}
```

#### 算术指令
```c
case OP_ADD: {
	arith_op(luai_numadd, TM_ADD);
	continue;
}
case OP_SUB: {
	arith_op(luai_numsub, TM_SUB);
	continue;
}
case OP_MUL: {
	arith_op(luai_nummul, TM_MUL);
	continue;
}
case OP_DIV: {
	arith_op(luai_numdiv, TM_DIV);
	continue;
}
case OP_MOD: {
	arith_op(luai_nummod, TM_MOD);
	continue;
}
case OP_POW: {
	arith_op(luai_numpow, TM_POW);
	continue;
}
```
统一调用`arith_op`,计算数据效果

| 功能  | 第一参数                                              | 第二参数 |
| :---: | ----------------------------------------------------- | -------- |
|  加   | `#define luai_numadd(a,b)	((a)+(b))`                  | TM_ADD   |
|  减   | `#define luai_numsub(a,b)	((a)-(b))`                  | TM_SUB   |
|  乘   | `#define luai_nummul(a,b)	((a)*(b))`                  | TM_MUL   |
|  除   | `#define luai_numdiv(a,b)	((a)/(b))`                  | TM_DIV   |
|  模   | `#define luai_nummod(a,b) ((a) - floor((a)/(b))*(b))` | TM_MOD   |
|  方   | `#define luai_numpow(a,b)	(pow(a,b))`                 | TM_POW   |

```c
void arith_op(op,tm)
{
	TValue *rb = RKB(i);	// 第二操作数
	TValue *rc = RKC(i);	// 第三操作数

	if (ttisnumber(rb) && 	// 校验数据
        ttisnumber(rc)) {	// 校验数据

		lua_Number nb = nvalue(rb);	// 提取数据
        lua_Number nc = nvalue(rc); // 提取数据

		setnvalue(ra, op(nb, nc)); // 设置第一操作数
	} else {
        Protect(Arith(L, ra, rb, rc, tm)); // 如果操作数不是数据,那么强转
    }
}

#define Protect(x)	{ L->savedpc = pc; {x;}; base = L->base; }
static void Arith (lua_State *L, StkId ra, const TValue *rb,
                   const TValue *rc, TMS op) {
  TValue tempb, tempc;
  const TValue *b, *c;
  if ((b = luaV_tonumber(rb, &tempb)) != NULL &&
      (c = luaV_tonumber(rc, &tempc)) != NULL) {
    lua_Number nb = nvalue(b), nc = nvalue(c);
    switch (op) {
      case TM_ADD: setnvalue(ra, luai_numadd(nb, nc)); break;
      case TM_SUB: setnvalue(ra, luai_numsub(nb, nc)); break;
      case TM_MUL: setnvalue(ra, luai_nummul(nb, nc)); break;
      case TM_DIV: setnvalue(ra, luai_numdiv(nb, nc)); break;
      case TM_MOD: setnvalue(ra, luai_nummod(nb, nc)); break;
      case TM_POW: setnvalue(ra, luai_numpow(nb, nc)); break;
      case TM_UNM: setnvalue(ra, luai_numunm(nb)); break;
      default: lua_assert(0); break;
    }
  }
  else if (!call_binTM(L, rb, rc, ra, op))
    luaG_aritherror(L, rb, rc);
}
```

