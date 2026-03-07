# Rbtree


<!--more-->
红黑树（Red Black Tree）是一种自平衡二叉查找树，在计算机科学中被广泛用于实现关联数组和其他需要高效插入、查找和删除操作的数据结构
<!--more-->

## 简介

> 红黑树是一种特化的AVL树（平衡二叉树），它通过特定的规则和操作保持树的平衡。

### 特性

红黑树的每个节点都包含一个颜色属性，可以是红色或黑色。红黑树需要满足以下五个关键性质：
+ 节点颜色：每个节点是红色或黑色。
+ 根节点：根节点是黑色的。
+ 叶子节点：所有叶子节点（NIL节点，空节点）都是黑色的。
+ 红色节点的子节点：每个红色节点的两个子节点都是黑色的（即红色节点不能相邻）。
+ 路径黑色节点数：从任一节点到其每个叶子的所有路径都包含相同数目的黑色节点。
+ 操作

### 操作
红黑树的基本操作包括查找、插入和删除。这些操作的时间复杂度在最坏情况下都是O(log n)，其中n是树中元素的数目。

+ 查找操作：红黑树的查找操作与普通二叉查找树类似，根据节点的键值进行查找。
+ 插入操作：在插入新节点时，新节点通常被着色为红色，然后通过一系列旋转和重新着色操作来维持树的平衡。
+ 删除操作：删除节点后，可能需要进行一系列的调整，包括重新着色和旋转，以保持树的平衡。

### 平衡性
红黑树的平衡性是通过树的旋转和重新着色操作来维持的。当插入或删除节点后，可能会违背红黑树的性质，此时需要进行调整。调整操作主要包括左旋和右旋，通过旋转和变色来保持树的平衡。

## 源码解析

```c
struct rb_node {
	unsigned long  __rb_parent_color;
	struct rb_node *rb_right;
	struct rb_node *rb_left;
} __attribute__((aligned(sizeof(long))));

struct rb_root {
	struct rb_node *rb_node;
};
```

```c
#define	RB_RED				0
#define	RB_BLACK			1

/* 最低两位为空 */
#define __rb_parent(pc)    	((struct rb_node *)(pc & ~3))

/* 最低位表示color */
#define __rb_color(pc)     	((pc) & 1)

#define __rb_is_black(pc)  	__rb_color(pc)
#define __rb_is_red(pc)    	(!__rb_color(pc))

#define rb_color(rb)       	__rb_color((rb)->__rb_parent_color)
#define rb_is_red(rb)      	__rb_is_red((rb)->__rb_parent_color)
#define rb_is_black(rb)    	__rb_is_black((rb)->__rb_parent_color)

static inline void rb_set_parent(struct rb_node *rb, struct rb_node *p)
{
	rb->__rb_parent_color = rb_color(rb) | (unsigned long)p;
}

static inline void rb_set_parent_color(struct rb_node *rb,
				       struct rb_node *p, int color)
{
	rb->__rb_parent_color = (unsigned long)p | color;
}
```

插入节点
```c
static inline void dummy_rotate(struct rb_node *old, struct rb_node *new) {}

void rb_insert_color(struct rb_node *node, struct rb_root *root)
{
	__rb_insert(node, root, dummy_rotate);
}

```

删除节点
```c
static inline void dummy_propagate(struct rb_node *node, struct rb_node *stop) {}
static inline void dummy_copy(struct rb_node *old, struct rb_node *new) {}
static inline void dummy_rotate(struct rb_node *old, struct rb_node *new) {}

static const struct rb_augment_callbacks dummy_callbacks = {
	dummy_propagate, dummy_copy, dummy_rotate
};

void rb_erase(struct rb_node *node, struct rb_root *root)
{
	struct rb_node *rebalance;
	rebalance = __rb_erase_augmented(node, root, &dummy_callbacks);
	if (rebalance)
		____rb_erase_color(rebalance, root, dummy_rotate);
}
```
