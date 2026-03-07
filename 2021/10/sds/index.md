# Sds


> A C dynamic strings library
> C语言版本动态字符串库

## SDS

SDS的类型就是
```c
typedef char *sds;
```
可以明显的看到,sds就是普通的char类型

### 下面是sds的数据类型

```ascii
+--------+-------------------------------+-----------+
| Header | Binary safe C alike string... | Null term |
+--------+-------------------------------+-----------+
         |
          -> Pointer returned to the user.
```

```c
#define SDS_HDR_VAR(T,s) \
		struct sdshdr##T *sh = (void*)((s)-(sizeof(struct sdshdr##T)));
#define SDS_HDR(T,s) \
		((struct sdshdr##T *)((s)-(sizeof(struct sdshdr##T))))
#define SDS_TYPE_5_LEN(f) ((f)>>SDS_TYPE_BITS)
```

### SDS 头

根据不同的标志计算不同的头部数据
|   宏定义    |   标志   |
| :---------: | :------: |
| SDS_TYPE_5  | sdshdr5  |
| SDS_TYPE_8  | sdshdr8  |
| SDS_TYPE_16 | sdshdr16 |
| SDS_TYPE_32 | sdshdr32 |
| SDS_TYPE_64 | sdshdr64 |

flag标志:

```c
unsigned char flags = s[-1]; /* 最后一个头部数据 */
```

```c
#define SDS_TYPE_5  0
#define SDS_TYPE_8  1
#define SDS_TYPE_16 2
#define SDS_TYPE_32 3
#define SDS_TYPE_64 4
```

```c
/* Note: sdshdr5 is never used, we just access the flags byte directly.
 * However is here to document the layout of type 5 SDS strings. */
struct __attribute__ ((__packed__)) sdshdr5 {
    unsigned char flags; /* 3 lsb of type, and 5 msb of string length */
    char buf[];
};
struct __attribute__ ((__packed__)) sdshdr8 {
    uint8_t len; /* used */
    uint8_t alloc; /* excluding the header and null terminator */
    unsigned char flags; /* 3 lsb of type, 5 unused bits */
    char buf[];
};
struct __attribute__ ((__packed__)) sdshdr16 {
    uint16_t len; /* used */
    uint16_t alloc; /* excluding the header and null terminator */
    unsigned char flags; /* 3 lsb of type, 5 unused bits */
    char buf[];
};
struct __attribute__ ((__packed__)) sdshdr32 {
    uint32_t len; /* used */
    uint32_t alloc; /* excluding the header and null terminator */
    unsigned char flags; /* 3 lsb of type, 5 unused bits */
    char buf[];
};
struct __attribute__ ((__packed__)) sdshdr64 {
    uint64_t len; /* used */
    uint64_t alloc; /* excluding the header and null terminator */
    unsigned char flags; /* 3 lsb of type, 5 unused bits */
    char buf[];
};
```

```c
#define SDS_TYPE_MASK 7
#define SDS_TYPE_BITS 3
```

## 创建SDS

函数原型
```c
sds sdsnewlen(const void *init, size_t initlen);
```

<img src="https://raw.githubusercontent.com/mengdemao/picture/master/SDS%E5%88%9D%E5%A7%8B%E5%8C%96.svg" alt="SDS初始化" style="zoom:50%;" />

## 扩张字符串缓存区
```c
sds sdsMakeRoomFor(sds s, size_t addlen)
{
    void *sh;
    void *newsh;
    size_t avail = sdsavail(s);			/* 计算剩余的可以使用的大小 */
    size_t len;
    size_t newlen;
    char type, oldtype = s[-1] & SDS_TYPE_MASK;
    int hdrlen;

    if (avail >= addlen) { /* 如果剩余的存储空间超过添加大小,那么就可以直接返回 */
        return s;
    }
    len = sdslen(s);	  /* 计算字符串大小 */
    sh = (char*)s - sdsHdrSize(oldtype); /* 缓冲区地址 */

    /* 计算得到新的长度 */
    newlen = (len+addlen);
    if (newlen < SDS_MAX_PREALLOC)
        newlen *= 2;
    else
        newlen += SDS_MAX_PREALLOC;
	/* 重新生成类型 */
    type = sdsReqType(newlen);

    /* Don't use type 5: the user is appending to the string and type 5 is
     * not able to remember empty space, so sdsMakeRoomFor() must be called
     * at every appending operation. */
    if (type == SDS_TYPE_5) {
        type = SDS_TYPE_8;
	}

    /* 计算头部大小 */
    hdrlen = sdsHdrSize(type);

    if (oldtype == type) {
        newsh = s_realloc(sh, hdrlen + newlen + 1);
        if (newsh == NULL) {
            return NULL;
        }
        s = (char*)newsh + hdrlen;
    } else {
        /* Since the header size changes, need to move the string forward,
         * and can't use realloc */
        newsh = s_malloc(hdrlen+newlen+1);
        if (newsh == NULL) {
        	return NULL;
        }
        memcpy((char*)newsh+hdrlen, s, len+1);
        s_free(sh);

        s = (char*)newsh + hdrlen;
        s[-1] = type;

        sdssetlen(s, len);
    }

    sdssetalloc(s, newlen);
    return s;
}
```

## 追加字符串
```c
sds sdscatlen(sds s, const void *t, size_t len)
{
    size_t curlen = sdslen(s);		/* 计算字符串的长度 */

    s = sdsMakeRoomFor(s,len);		/* 扩展字符串缓冲区长度 */
    if (s == NULL) {
         return NULL;
    }
    memcpy(s+curlen, t, len);		/* 添加字符串 */
    sdssetlen(s, curlen+len);		/* 设置长度标志 */
    s[curlen+len] = '\0';			/* 补全结束符 */
    return s;
}
```

