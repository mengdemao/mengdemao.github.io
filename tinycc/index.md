# Tinycc


TinyCC
====

> TinyCC (aka TCC) is a small but hyper fast C compiler. 
> Unlike other C compilers, it is meant to be self-relying: you do not need an external 
> assembler or linker because TCC does that for you.

## TCC 执行概览
TCC组成代码
| 文件名                                      | 解释 |
| ------------------------------------------- | ---- |
| `tcc.c`/`libtcc.c`                          | tcc运行代码 |
| `tccpp.c`                                   |      |
| `tccelf.c` `tcccoff.c`                      |      |
| `tccasm.c`                                  |      |
| `tccrun.c`                                  |      |
| `tccgen.c`                                  |      |
| `x86_64-gen.c` `x86_64-link.c` `i386-asm.c` | 不同架构的生成器     |


![TccMainExecute](https://raw.githubusercontent.com/mengdemao/picture/master/TccMainExecute.png)

## TCC状态信息

```c
struct TCCState {
    unsigned char verbose; /* if true, display some information during compilation */
    unsigned char nostdinc; /* if true, no standard headers are added */
    unsigned char nostdlib; /* if true, no standard libraries are added */
    unsigned char nocommon; /* if true, do not use common symbols for .bss data */
    unsigned char static_link; /* if true, static linking is performed */
    unsigned char rdynamic; /* if true, all symbols are exported */
    unsigned char symbolic; /* if true, resolve symbols in the current module first */
    unsigned char filetype; /* file type for compilation (NONE,C,ASM) */
    unsigned char optimize; /* only to #define __OPTIMIZE__ */
    unsigned char option_pthread; /* -pthread option */
    unsigned char enable_new_dtags; /* -Wl,--enable-new-dtags */
    unsigned int  cversion; /* supported C ISO version, 199901 (the default), 201112, ... */

    /* C language options */
    unsigned char char_is_unsigned;
    unsigned char leading_underscore;
    unsigned char ms_extensions; /* allow nested named struct w/o identifier behave like unnamed */
    unsigned char dollars_in_identifiers;  /* allows '$' char in identifiers */
    unsigned char ms_bitfields; /* if true, emulate MS algorithm for aligning bitfields */

    /* warning switches */
    unsigned char warn_none;
    unsigned char warn_all;
    unsigned char warn_error;
    unsigned char warn_write_strings;
    unsigned char warn_unsupported;
    unsigned char warn_implicit_function_declaration;
    unsigned char warn_discarded_qualifiers;
    #define WARN_ON  1 /* warning is on (-Woption) */
    unsigned char warn_num; /* temp var for tcc_warning_c() */

    unsigned char option_r; /* option -r */
    unsigned char do_bench; /* option -bench */
    unsigned char just_deps; /* option -M  */
    unsigned char gen_deps; /* option -MD  */
    unsigned char include_sys_deps; /* option -MD  */

    /* compile with debug symbol (and use them if error during execution) */
    unsigned char do_debug;
    unsigned char do_backtrace;
#ifdef CONFIG_TCC_BCHECK
    /* compile with built-in memory and bounds checker */
    unsigned char do_bounds_check;
#endif
    unsigned char test_coverage;  /* generate test coverage code */

    /* use GNU C extensions */
    unsigned char gnu_ext;
    /* use TinyCC extensions */
    unsigned char tcc_ext;

    unsigned char dflag; /* -dX value */
    unsigned char Pflag; /* -P switch (LINE_MACRO_OUTPUT_FORMAT) */

#ifdef TCC_TARGET_X86_64
    unsigned char nosse; /* For -mno-sse support. */
#endif
#ifdef TCC_TARGET_ARM
    unsigned char float_abi; /* float ABI of the generated code*/
#endif

    unsigned char has_text_addr;
    addr_t text_addr; /* address of text section */
    unsigned section_align; /* section alignment */
#ifdef TCC_TARGET_I386
    int seg_size; /* 32. Can be 16 with i386 assembler (.code16) */
#endif

    char *tcc_lib_path; /* CONFIG_TCCDIR or -B option */
    char *soname; /* as specified on the command line (-soname) */
    char *rpath; /* as specified on the command line (-Wl,-rpath=) */

    char *init_symbol; /* symbols to call at load-time (not used currently) */
    char *fini_symbol; /* symbols to call at unload-time (not used currently) */

    /* output type, see TCC_OUTPUT_XXX */
    int output_type;
    /* output format, see TCC_OUTPUT_FORMAT_xxx */
    int output_format;
    /* nth test to run with -dt -run */
    int run_test;

    /* array of all loaded dlls (including those referenced by loaded dlls) */
    DLLReference **loaded_dlls;
    int nb_loaded_dlls;

    /* include paths */
    char **include_paths;
    int nb_include_paths;

    char **sysinclude_paths;
    int nb_sysinclude_paths;

    /* library paths */
    char **library_paths;
    int nb_library_paths;

    /* crt?.o object path */
    char **crt_paths;
    int nb_crt_paths;

    /* -D / -U options */
    CString cmdline_defs;
    /* -include options */
    CString cmdline_incl;

    /* error handling */
    void *error_opaque;
    void (*error_func)(void *opaque, const char *msg);
    int error_set_jmp_enabled;
    jmp_buf error_jmp_buf;
    int nb_errors;

    /* output file for preprocessing (-E) */
    FILE *ppfp;

    /* for -MD/-MF: collected dependencies for this compilation */
    char **target_deps;
    int nb_target_deps;

    /* compilation */
    BufferedFile *include_stack[INCLUDE_STACK_SIZE];
    BufferedFile **include_stack_ptr;

    int ifdef_stack[IFDEF_STACK_SIZE];
    int *ifdef_stack_ptr;

    /* included files enclosed with #ifndef MACRO */
    int cached_includes_hash[CACHED_INCLUDES_HASH_SIZE];
    CachedInclude **cached_includes;
    int nb_cached_includes;

    /* #pragma pack stack */
    int pack_stack[PACK_STACK_SIZE];
    int *pack_stack_ptr;
    char **pragma_libs;
    int nb_pragma_libs;

    /* inline functions are stored as token lists and compiled last
       only if referenced */
    struct InlineFunc **inline_fns;
    int nb_inline_fns;

    /* sections */
    Section **sections;
    int nb_sections; /* number of sections, including first dummy section */

    Section **priv_sections;
    int nb_priv_sections; /* number of private sections */

    /* got & plt handling */
    Section *got;
    Section *plt;

    /* predefined sections */
    Section *text_section, *data_section, *rodata_section, *bss_section;
    Section *common_section;
    Section *cur_text_section; /* current section where function code is generated */
#ifdef CONFIG_TCC_BCHECK
    /* bound check related sections */
    Section *bounds_section; /* contains global data bound description */
    Section *lbounds_section; /* contains local data bound description */
#endif
    /* test coverage */
    Section *tcov_section;
    /* symbol sections */
    Section *symtab_section;
    /* debug sections */
    Section *stab_section;
    /* Is there a new undefined sym since last new_undef_sym() */
    int new_undef_sym;

    /* temporary dynamic symbol sections (for dll loading) */
    Section *dynsymtab_section;
    /* exported dynamic symbol section */
    Section *dynsym;
    /* copy of the global symtab_section variable */
    Section *symtab;
    /* extra attributes (eg. GOT/PLT value) for symtab symbols */
    struct sym_attr *sym_attrs;
    int nb_sym_attrs;
    /* ptr to next reloc entry reused */
    ElfW_Rel *qrel;
    #define qrel s1->qrel

#ifdef TCC_TARGET_RISCV64
    struct pcrel_hi { addr_t addr, val; } last_hi;
    #define last_hi s1->last_hi
#endif

#ifdef TCC_TARGET_PE
    /* PE info */
    int pe_subsystem;
    unsigned pe_characteristics;
    unsigned pe_file_align;
    unsigned pe_stack_size;
    addr_t pe_imagebase;
# ifdef TCC_TARGET_X86_64
    Section *uw_pdata;
    int uw_sym;
    unsigned uw_offs;
# endif
#endif

#ifndef ELF_OBJ_ONLY
    int nb_sym_versions;
    struct sym_version *sym_versions;
    int nb_sym_to_version;
    int *sym_to_version;
    int dt_verneednum;
    Section *versym_section;
    Section *verneed_section;
#endif

#ifdef TCC_IS_NATIVE
    const char *runtime_main;
    void **runtime_mem;
    int nb_runtime_mem;
#endif

#ifdef CONFIG_TCC_BACKTRACE
    int rt_num_callers;
#endif

    /* benchmark info */
    int total_idents;
    int total_lines;
    int total_bytes;
    int total_output[4];

    /* option -dnum (for general development purposes) */
    int g_debug;

    /* used by tcc_load_ldscript */
    int fd, cc;

    /* for warnings/errors for object files */
    const char *current_filename;

    /* used by main and tcc_parse_args only */
    struct filespec **files; /* files seen on command line */
    int nb_files; /* number thereof */
    int nb_libraries; /* number of libs thereof */
    char *outfile; /* output filename */
    char *deps_outfile; /* option -MF */
    int argc;
    char **argv;
};
```

```c
LIBTCCAPI TCCState *tcc_new(void)
{
    TCCState *s;

    s = tcc_mallocz(sizeof(TCCState));
    if (!s)
        return NULL;
#ifdef MEM_DEBUG
    ++nb_states;
#endif

#undef gnu_ext

    s->gnu_ext = 1;
    s->tcc_ext = 1;
    s->nocommon = 1;
    s->dollars_in_identifiers = 1; /*on by default like in gcc/clang*/
    s->cversion = 199901; /* default unless -std=c11 is supplied */
    s->warn_implicit_function_declaration = 1;
    s->warn_discarded_qualifiers = 1;
    s->ms_extensions = 1;

#ifdef CHAR_IS_UNSIGNED
    s->char_is_unsigned = 1;
#endif
#ifdef TCC_TARGET_I386
    s->seg_size = 32;
#endif
    /* enable this if you want symbols with leading underscore on windows: */
#if defined TCC_TARGET_MACHO /* || defined TCC_TARGET_PE */
    s->leading_underscore = 1;
#endif
#ifdef TCC_TARGET_ARM
    s->float_abi = ARM_FLOAT_ABI;
#endif

    s->ppfp = stdout;
    /* might be used in error() before preprocess_start() */
    s->include_stack_ptr = s->include_stack;

    tccelf_new(s);

    tcc_set_lib_path(s, CONFIG_TCCDIR);
    return s;
}

LIBTCCAPI void tcc_delete(TCCState *s1)
{
    /* free sections */
    tccelf_delete(s1);

    /* free library paths */
    dynarray_reset(&s1->library_paths, &s1->nb_library_paths);
    dynarray_reset(&s1->crt_paths, &s1->nb_crt_paths);

    /* free include paths */
    dynarray_reset(&s1->include_paths, &s1->nb_include_paths);
    dynarray_reset(&s1->sysinclude_paths, &s1->nb_sysinclude_paths);

    tcc_free(s1->tcc_lib_path);
    tcc_free(s1->soname);
    tcc_free(s1->rpath);
    tcc_free(s1->init_symbol);
    tcc_free(s1->fini_symbol);
    tcc_free(s1->outfile);
    tcc_free(s1->deps_outfile);
    dynarray_reset(&s1->files, &s1->nb_files);
    dynarray_reset(&s1->target_deps, &s1->nb_target_deps);
    dynarray_reset(&s1->pragma_libs, &s1->nb_pragma_libs);
    dynarray_reset(&s1->argv, &s1->argc);
    cstr_free(&s1->cmdline_defs);
    cstr_free(&s1->cmdline_incl);
#ifdef TCC_IS_NATIVE
    /* free runtime memory */
    tcc_run_free(s1);
#endif

    tcc_free(s1);
#ifdef MEM_DEBUG
    if (0 == --nb_states)
        tcc_memcheck();
#endif
}
```

## 文件读取

### 文件描述信息

```c
typedef struct BufferedFile {
    uint8_t *buf_ptr;
    uint8_t *buf_end;
    int fd;
    struct BufferedFile *prev;
    int line_num;    /* current line number - here to simplify code */
    int line_ref;    /* tcc -E: last printed line */
    int ifndef_macro;  /* #ifndef macro / #endif search */
    int ifndef_macro_saved; /* saved ifndef_macro */
    int *ifdef_stack_ptr; /* ifdef_stack value at the start of the file */
    int include_next_index; /* next search path */
    char filename[1024];    /* filename */
    char *true_filename; /* filename not modified by # line directive */
    unsigned char unget[4];
    unsigned char buffer[1]; /* extra size for CH_EOB char */
} BufferedFile;
```

### 文件相关操作

```c
static int _tcc_open(TCCState *s1, const char *filename)
{
    int fd;
    if (strcmp(filename, "-") == 0)
        fd = 0, filename = "<stdin>";
    else
        fd = open(filename, O_RDONLY | O_BINARY);
    if ((s1->verbose == 2 && fd >= 0) || s1->verbose == 3)
        printf("%s %*s%s\n", fd < 0 ? "nf":"->",
               (int)(s1->include_stack_ptr - s1->include_stack), "", filename);
    return fd;
}

ST_FUNC int tcc_open(TCCState *s1, const char *filename)
{
    int fd = _tcc_open(s1, filename);
    if (fd < 0)
        return -1;
    tcc_open_bf(s1, filename, 0);
    file->fd = fd;
    return 0;
}

ST_FUNC void tcc_close(void)
{
    TCCState *s1 = tcc_state;
    BufferedFile *bf = file;
    if (bf->fd > 0) {
        close(bf->fd);
        total_lines += bf->line_num;
    }
    if (bf->true_filename != bf->filename)
        tcc_free(bf->true_filename);
    file = bf->prev;
    tcc_free(bf);
}
```

## 词法分析

## 语法分析

## 类型检查

## 符号表

## 段机制

## 代码生成


