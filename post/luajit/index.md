
LuaJIT
====
## Lua语法
### 基本语法

``` lua
	print("Hello World")
```
### 表(table)

## LuaJIT分析
### LuaJIT主函数
``` c
int main(int argc, char **argv)
{
	int status; 				/* 返回值 */
	lua_State *L = lua_open();  /* 创建LUA状态机 */
	if (L == NULL) {
		l_message(argv[0], "cannot create state: not enough memory");
		return EXIT_FAILURE;
	}
	
	/* smain只存在三个参数,主要作用是向pmain传递数据 */
	smain.argc = argc;
	smain.argv = argv;
	
	status = lua_cpcall(L, pmain, NULL);	/* 启动函数调用 */
	
	report(L, status); /* 提取报错参数 */
	
	lua_close(L);	/* 销毁状态机 */
	
	return (status || smain.status > 0) ? EXIT_FAILURE : EXIT_SUCCESS;
}
```

### Lua状态机
``` c
struct lua_State {
	GCObject*next;
    
    lu_byte tt;
    lu_byte marked;
	lu_byte status;
	
    StkId top;
	StkId base;
	
    global_State *l_G;	/* 全局状态信息 */
	
    CallInfo*ci;
	
    const Instruction*savedpc;
	StkId stack_last;
	StkId stack;
	
    CallInfo*end_ci;
	CallInfo*base_ci;
	
    int stacksize;
	int size_ci;
	unsigned short nCcalls;
	unsigned short baseCcalls;
	
    lu_byte hookmask;
	lu_byte allowhook;
	
    int basehookcount;
	int hookcount;
	
    lua_Hook hook;
	
    TValue l_gt;
	TValue env;
	
    GCObject*openupval;
	GCObject*gclist;
	
    struct lua_longjmp*errorJmp;
	
    ptrdiff_t errfunc;
};
```

### 创建状态
``` c
/* 此函数实际不存在,程序内部使用的是宏定义 */
void lua_open(void);

/* 实际调用位置 */
LUALIB_API lua_State *luaL_newstate(void);

/* 根据编译期64位信息选择调用 */
#if LJ_64 && !LJ_GC64 && !(defined(LUAJIT_USE_VALGRIND) && defined(LUAJIT_USE_SYSMALLOC))
lua_State *lj_state_newstate(lua_Alloc allocf, void *allocd);
#else
LUA_API lua_State *lua_newstate(lua_Alloc allocf, void *allocd);
#endif
```


### 函数调用

```c
LUA_API int lua_cpcall(lua_State *L, lua_CFunction func, void *ud);
LUA_API int lua_pcall(lua_State *L, int nargs, int nresults, int errfunc);
LUA_API void lua_call(lua_State *L, int nargs, int nresults);
```
lua_cpcall函数调用

![lua_cpcall执行截图](lua_cpcall执行截图.png)

## 执行原理


## FFI分析

