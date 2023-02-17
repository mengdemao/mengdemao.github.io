# 物理页面分配器


## alloc_pages

linux常用的物理界面分配器的函数是`alloc_pages`,下面分析是如何实现的.

```c
#define alloc_pages(gfp_mask, order) alloc_pages_node(numa_node_id(), gfp_mask, order)

static inline struct page *alloc_pages_node(int nid, gfp_t gfp_mask, unsigned int order)
{
	/* Unknown node is current node */
	if (nid < 0)
		nid = numa_node_id();

	return __alloc_pages(gfp_mask, order, node_zonelist(nid, gfp_mask));
}

static inline struct page *__alloc_pages(gfp_t gfp_mask, unsigned int order, struct zonelist *zonelist)
{
	return __alloc_pages_nodemask(gfp_mask, order, zonelist, NULL);
}
```

**此时正式进入函数分析阶段**
```c
/**
 * @brief  分配物理页面
 * @param  gfp_mask         分配掩码
 * @param  order            分配阶数
 * @param  zonelist			内存区域
 * @param  nodemask			节点掩码
 * @return struct page*     分配的物理页面
 */
struct page *__alloc_pages_nodemask(gfp_t gfp_mask, unsigned int order, struct zonelist *zonelist, nodemask_t *nodemask);
```

详细解析接口实现

+ 分配掩码

```c
// 分配掩码就是一个unsigned int类型
typedef unsigned __bitwise__ gfp_t;
```

```c
#define ___GFP_DMA			0x01u
#define ___GFP_HIGHMEM		0x02u
#define ___GFP_DMA32		0x04u
#define ___GFP_MOVABLE		0x08u
#define ___GFP_WAIT			0x10u
#define ___GFP_HIGH			0x20u
#define ___GFP_IO			0x40u
#define ___GFP_FS			0x80u
#define ___GFP_COLD			0x100u
#define ___GFP_NOWARN		0x200u
#define ___GFP_REPEAT		0x400u
#define ___GFP_NOFAIL		0x800u
#define ___GFP_NORETRY		0x1000u
#define ___GFP_MEMALLOC		0x2000u
#define ___GFP_COMP			0x4000u
#define ___GFP_ZERO			0x8000u
#define ___GFP_NOMEMALLOC	0x10000u
#define ___GFP_HARDWALL		0x20000u
#define ___GFP_THISNODE		0x40000u
#define ___GFP_RECLAIMABLE	0x80000u
#define ___GFP_NOTRACK		0x200000u
#define ___GFP_NO_KSWAPD	0x400000u
#define ___GFP_OTHER_NODE	0x800000u
#define ___GFP_WRITE		0x1000000u
```

```c
#define __force	__attribute__((force))

#define __GFP_DMA				((__force gfp_t)___GFP_DMA)
#define __GFP_HIGHMEM			((__force gfp_t)___GFP_HIGHMEM)
#define __GFP_DMA32				((__force gfp_t)___GFP_DMA32)
#define __GFP_MOVABLE			((__force gfp_t)___GFP_MOVABLE)  /* Page is movable */

#define __GFP_WAIT				((__force gfp_t)___GFP_WAIT)	/* Can wait and reschedule? */
#define __GFP_HIGH				((__force gfp_t)___GFP_HIGH)	/* Should access emergency pools? */
#define __GFP_IO				((__force gfp_t)___GFP_IO)	/* Can start physical IO? */
#define __GFP_FS				((__force gfp_t)___GFP_FS)	/* Can call down to low-level FS? */
#define __GFP_COLD				((__force gfp_t)___GFP_COLD)	/* Cache-cold page required */
#define __GFP_NOWARN			((__force gfp_t)___GFP_NOWARN)	/* Suppress page allocation failure warning */
#define __GFP_REPEAT			((__force gfp_t)___GFP_REPEAT)	/* See above */
#define __GFP_NOFAIL			((__force gfp_t)___GFP_NOFAIL)	/* See above */
#define __GFP_NORETRY			((__force gfp_t)___GFP_NORETRY) /* See above */
#define __GFP_MEMALLOC			((__force gfp_t)___GFP_MEMALLOC)/* Allow access to emergency reserves */
#define __GFP_COMP				((__force gfp_t)___GFP_COMP)	/* Add compound page metadata */
#define __GFP_ZERO				((__force gfp_t)___GFP_ZERO)	/* Return zeroed page on success */
#define __GFP_NOMEMALLOC 		((__force gfp_t)___GFP_NOMEMALLOC) /* Don't use emergency reserves.
							 		* This takes precedence over the
							 		* __GFP_MEMALLOC flag if both are
							 		* set
							 		*/
#define __GFP_HARDWALL   		((__force gfp_t)___GFP_HARDWALL) /* Enforce hardwall cpuset memory allocs */
#define __GFP_THISNODE			((__force gfp_t)___GFP_THISNODE)/* No fallback, no policies */
#define __GFP_RECLAIMABLE 		((__force gfp_t)___GFP_RECLAIMABLE) /* Page is reclaimable */
#define __GFP_NOTRACK			((__force gfp_t)___GFP_NOTRACK)  /* Don't track with kmemcheck */

#define __GFP_NO_KSWAPD			((__force gfp_t)___GFP_NO_KSWAPD)
#define __GFP_OTHER_NODE 		((__force gfp_t)___GFP_OTHER_NODE) /* On behalf of other node */
#define __GFP_WRITE				((__force gfp_t)___GFP_WRITE)	/* Allocator intends to dirty page */
```

+ 分配阶数

描述分配的大小 $$ {page\_number} = 2 ^ {order} $$

+ 内存区域

`zone`令行分析

+ 节点掩码

`nodemask`令行分析

### __alloc_pages_nodemask

``` c
struct page *
__alloc_pages_nodemask(gfp_t gfp_mask, unsigned int order,
			struct zonelist *zonelist, nodemask_t *nodemask)
{
	struct zoneref *preferred_zoneref;
	struct page *page = NULL;
	unsigned int cpuset_mems_cookie;
	int alloc_flags = ALLOC_WMARK_LOW|ALLOC_CPUSET|ALLOC_FAIR;
	gfp_t alloc_mask; /* The gfp_t that was actually used for allocation */
	struct alloc_context ac = {
		.high_zoneidx = gfp_zone(gfp_mask),
		.nodemask = nodemask,
		.migratetype = gfpflags_to_migratetype(gfp_mask),
	};

	gfp_mask &= gfp_allowed_mask;

	lockdep_trace_alloc(gfp_mask);

	might_sleep_if(gfp_mask & __GFP_WAIT);

	if (should_fail_alloc_page(gfp_mask, order))
		return NULL;

	/*
	 * Check the zones suitable for the gfp_mask contain at least one
	 * valid zone. It's possible to have an empty zonelist as a result
	 * of GFP_THISNODE and a memoryless node
	 */
	if (unlikely(!zonelist->_zonerefs->zone))
		return NULL;

	if (IS_ENABLED(CONFIG_CMA) && ac.migratetype == MIGRATE_MOVABLE)
		alloc_flags |= ALLOC_CMA;

retry_cpuset:
	cpuset_mems_cookie = read_mems_allowed_begin();

	/* We set it here, as __alloc_pages_slowpath might have changed it */
	ac.zonelist = zonelist;
	/* The preferred zone is used for statistics later */
	preferred_zoneref = first_zones_zonelist(ac.zonelist, ac.high_zoneidx,
				ac.nodemask ? : &cpuset_current_mems_allowed,
				&ac.preferred_zone);
	if (!ac.preferred_zone)
		goto out;
	ac.classzone_idx = zonelist_zone_idx(preferred_zoneref);

	/* First allocation attempt */
	alloc_mask = gfp_mask|__GFP_HARDWALL;
	page = get_page_from_freelist(alloc_mask, order, alloc_flags, &ac);
	if (unlikely(!page)) {
		/*
		 * Runtime PM, block IO and its error handling path
		 * can deadlock because I/O on the device might not
		 * complete.
		 */
		alloc_mask = memalloc_noio_flags(gfp_mask);

		page = __alloc_pages_slowpath(alloc_mask, order, &ac);
	}

	if (kmemcheck_enabled && page)
		kmemcheck_pagealloc_alloc(page, order, gfp_mask);

	trace_mm_page_alloc(page, order, alloc_mask, ac.migratetype);

out:
	/*
	 * When updating a task's mems_allowed, it is possible to race with
	 * parallel threads in such a way that an allocation can fail while
	 * the mask is being updated. If a page allocation is about to fail,
	 * check if the cpuset changed during allocation and if so, retry.
	 */
	if (unlikely(!page && read_mems_allowed_retry(cpuset_mems_cookie)))
		goto retry_cpuset;

	return page;
}
```

```c
struct alloc_context {
	struct zonelist 	*zonelist;
	nodemask_t 			*nodemask;
	struct zone 		*preferred_zone;
	int 				 classzone_idx;
	int 				 migratetype;
	enum zone_type 		 high_zoneidx;
};

struct alloc_context ac = {
	.high_zoneidx = gfp_zone(gfp_mask),
	.nodemask = nodemask,
	.migratetype = gfpflags_to_migratetype(gfp_mask),
};

static inline enum zone_type gfp_zone(gfp_t flags)
{
	enum zone_type z;
	int bit = (__force int) (flags & GFP_ZONEMASK);

	z = (GFP_ZONE_TABLE >> (bit * ZONES_SHIFT)) &
					 ((1 << ZONES_SHIFT) - 1);
	VM_BUG_ON((GFP_ZONE_BAD >> bit) & 1);
	return z;
}

// GFP --> migratetype
static inline int gfpflags_to_migratetype(const gfp_t gfp_flags)
{
	WARN_ON((gfp_flags & GFP_MOVABLE_MASK) == GFP_MOVABLE_MASK);

	if (unlikely(page_group_by_mobility_disabled))
		return MIGRATE_UNMOVABLE;

	/* Group based on mobility */
	return (((gfp_flags & __GFP_MOVABLE) != 0) << 1) |
		((gfp_flags & __GFP_RECLAIMABLE) != 0);
}

```
### first_zones_zonelist

```c
static inline struct zoneref *first_zones_zonelist(struct zonelist *zonelist,
					enum zone_type highest_zoneidx,
					nodemask_t *nodes,
					struct zone **zone)
{
	struct zoneref *z = next_zones_zonelist(zonelist->_zonerefs,
							highest_zoneidx, nodes);
	*zone = zonelist_zone(z);
	return z;
}

// 提取zone的指针
static inline struct zone *zonelist_zone(struct zoneref *zoneref)
{
	return zoneref->zone;
}

struct zoneref *next_zones_zonelist(struct zoneref *z,
					enum zone_type highest_zoneidx,
					nodemask_t *nodes)
{
	/*
	 * Find the next suitable zone to use for the allocation.
	 * Only filter based on nodemask if it's set
	 */
	if (likely(nodes == NULL))
		while (zonelist_zone_idx(z) > highest_zoneidx)
			z++;
	else
		while (zonelist_zone_idx(z) > highest_zoneidx ||
				(z->zone && !zref_in_nodemask(z, nodes)))
			z++;

	return z;
}

```


## free_pages

```c
void free_pages(unsigned long addr, unsigned int order)
{
	if (addr != 0) {
		VM_BUG_ON(!virt_addr_valid((void *)addr));
		__free_pages(virt_to_page((void *)addr), order);
	}
}
```

### __free_pages

```c
void __free_pages(struct page *page, unsigned int order)
{
	if (put_page_testzero(page)) {
		if (order == 0)
			free_hot_cold_page(page, false);
		else
			__free_pages_ok(page, order);
	}
}
```

### __free_pages_ok

```c
static void __free_pages_ok(struct page *page, unsigned int order)
{
	unsigned long flags;
	int migratetype;
	unsigned long pfn = page_to_pfn(page);

	if (!free_pages_prepare(page, order))
		return;

	migratetype = get_pfnblock_migratetype(page, pfn);
	local_irq_save(flags);
	__count_vm_events(PGFREE, 1 << order);
	set_freepage_migratetype(page, migratetype);
	free_one_page(page_zone(page), page, pfn, order, migratetype);
	local_irq_restore(flags);
}
```

### free_one_page

```c
static void free_one_page(struct zone *zone,
				struct page *page, unsigned long pfn,
				unsigned int order,
				int migratetype)
{
	unsigned long nr_scanned;
	spin_lock(&zone->lock);
	nr_scanned = zone_page_state(zone, NR_PAGES_SCANNED);
	if (nr_scanned)
		__mod_zone_page_state(zone, NR_PAGES_SCANNED, -nr_scanned);

	if (unlikely(has_isolate_pageblock(zone) ||
		is_migrate_isolate(migratetype))) {
		migratetype = get_pfnblock_migratetype(page, pfn);
	}
	__free_one_page(page, pfn, zone, order, migratetype);
	spin_unlock(&zone->lock);
}
```

### __free_one_page

```c
static inline void __free_one_page(struct page *page,
		unsigned long pfn,
		struct zone *zone, unsigned int order,
		int migratetype)
{
	unsigned long page_idx;
	unsigned long combined_idx;
	unsigned long uninitialized_var(buddy_idx);
	struct page *buddy;
	int max_order = MAX_ORDER;

	VM_BUG_ON(!zone_is_initialized(zone));
	VM_BUG_ON_PAGE(page->flags & PAGE_FLAGS_CHECK_AT_PREP, page);

	VM_BUG_ON(migratetype == -1);
	if (is_migrate_isolate(migratetype)) {
		/*
		 * We restrict max order of merging to prevent merge
		 * between freepages on isolate pageblock and normal
		 * pageblock. Without this, pageblock isolation
		 * could cause incorrect freepage accounting.
		 */
		max_order = min(MAX_ORDER, pageblock_order + 1);
	} else {
		__mod_zone_freepage_state(zone, 1 << order, migratetype);
	}

	page_idx = pfn & ((1 << max_order) - 1);

	VM_BUG_ON_PAGE(page_idx & ((1 << order) - 1), page);
	VM_BUG_ON_PAGE(bad_range(zone, page), page);

	while (order < max_order - 1) {
		buddy_idx = __find_buddy_index(page_idx, order);
		buddy = page + (buddy_idx - page_idx);
		if (!page_is_buddy(page, buddy, order))
			break;
		/*
		 * Our buddy is free or it is CONFIG_DEBUG_PAGEALLOC guard page,
		 * merge with it and move up one order.
		 */
		if (page_is_guard(buddy)) {
			clear_page_guard(zone, buddy, order, migratetype);
		} else {
			list_del(&buddy->lru);
			zone->free_area[order].nr_free--;
			rmv_page_order(buddy);
		}
		combined_idx = buddy_idx & page_idx;
		page = page + (combined_idx - page_idx);
		page_idx = combined_idx;
		order++;
	}
	set_page_order(page, order);

	/*
	 * If this is not the largest possible page, check if the buddy
	 * of the next-highest order is free. If it is, it's possible
	 * that pages are being freed that will coalesce soon. In case,
	 * that is happening, add the free page to the tail of the list
	 * so it's less likely to be used soon and more likely to be merged
	 * as a higher order page
	 */
	if ((order < MAX_ORDER-2) && pfn_valid_within(page_to_pfn(buddy))) {
		struct page *higher_page, *higher_buddy;
		combined_idx = buddy_idx & page_idx;
		higher_page = page + (combined_idx - page_idx);
		buddy_idx = __find_buddy_index(combined_idx, order + 1);
		higher_buddy = higher_page + (buddy_idx - combined_idx);
		if (page_is_buddy(higher_page, higher_buddy, order + 1)) {
			list_add_tail(&page->lru,
				&zone->free_area[order].free_list[migratetype]);
			goto out;
		}
	}

	list_add(&page->lru, &zone->free_area[order].free_list[migratetype]);
out:
	zone->free_area[order].nr_free++;
}
```
