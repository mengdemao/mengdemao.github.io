# FreeRTOS


## 基本概念

### 任务

> 一个死循环的函数运行(在RTOS中)

### 函数运行环境(context)
  + 寄存器
  + 堆栈信息

### 函数执行原理

  + pc/lr
  + push/pop

下面写一个简单的函数,测试一下
函数是如何尽心调用的;

```c
int test(int a, int b)
{
    return 0;
}

void main(void)
{
    test(1, 2);
}
```

main --> test

下面是程序编译后得到的汇编源码

```asm
test:
    push    {r7}                @ r7 --> stack
    sub     sp, sp, #12         @ sp = sp + 12  // 三个局部变量
    add     r7, sp, #0          @ r7 = sp
    str     r0, [r7, #4]        @ 保存第一个变量
    str     r1, [r7]            @ 保存第二个变量
    movs    r3, #0              @ r3 = 0
    mov     r0, r3              @ r0 = r3
    adds    r7, r7, #12         @ r7 = r7 + 12
    mov     sp, r7              @ sp = r7 (销毁堆栈)
    ldr     r7, [sp], #4        @ 恢复r7
    bx      lr                  @ 执行返回

main:
    push    {r7, lr}    @ 保存r7, lr
    add     r7, sp, #0  @ r7 = sp
    movs    r1, #2      @ r1 = 2
    movs    r0, #1      @ r0 = 1
    bl      test        @ call test
    nop                 @ nop
    pop     {r7, pc}    @ lr_r = pc_v
                        @ r7_r = r7_v
```

### 总结

那么,多任务就可以任务是从一个函数切换到另外一个函数; </br>
但是此种切换对于任务来说是不可见的;

+ 任务切换需要特权(定时器中断)
+ 任务切换需要时停(开关中断)

## 任务

### 任务创建

+ 分配TCB_t结构体
+ 初始化参数(prvInitialiseNewTask)
+ 添加到就绪列表(prvAddNewTaskToReadyList)

#### 分配TCB结构体

1. 申请TCB内存
2. 申请栈内存

TCB结构体类型

```c
typedef tskTCB TCB_t;
typedef struct tskTaskControlBlock       /* The old naming convention is used to prevent breaking kernel aware debuggers. */
{
    volatile StackType_t * pxTopOfStack; /*< Points to the location of the last item placed on the tasks stack.  THIS MUST BE THE FIRST MEMBER OF THE TCB STRUCT. */

    #if ( portUSING_MPU_WRAPPERS == 1 )
        xMPU_SETTINGS xMPUSettings; /*< The MPU settings are defined as part of the port layer.  THIS MUST BE THE SECOND MEMBER OF THE TCB STRUCT. */
    #endif

    ListItem_t xStateListItem;                  /*< The list that the state list item of a task is reference from denotes the state of that task (Ready, Blocked, Suspended ). */
    ListItem_t xEventListItem;                  /*< Used to reference a task from an event list. */
    UBaseType_t uxPriority;                     /*< The priority of the task.  0 is the lowest priority. */
    StackType_t * pxStack;                      /*< Points to the start of the stack. */
    char pcTaskName[ configMAX_TASK_NAME_LEN ]; /*< Descriptive name given to the task when created.  Facilitates debugging only. */ /*lint !e971 Unqualified char types are allowed for strings and single characters only. */

    #if ( ( portSTACK_GROWTH > 0 ) || ( configRECORD_STACK_HIGH_ADDRESS == 1 ) )
        StackType_t * pxEndOfStack; /*< Points to the highest valid address for the stack. */
    #endif

    #if ( portCRITICAL_NESTING_IN_TCB == 1 )
        UBaseType_t uxCriticalNesting; /*< Holds the critical section nesting depth for ports that do not maintain their own count in the port layer. */
    #endif

    #if ( configUSE_TRACE_FACILITY == 1 )
        UBaseType_t uxTCBNumber;  /*< Stores a number that increments each time a TCB is created.  It allows debuggers to determine when a task has been deleted and then recreated. */
        UBaseType_t uxTaskNumber; /*< Stores a number specifically for use by third party trace code. */
    #endif

    #if ( configUSE_MUTEXES == 1 )
        UBaseType_t uxBasePriority; /*< The priority last assigned to the task - used by the priority inheritance mechanism. */
        UBaseType_t uxMutexesHeld;
    #endif

    #if ( configUSE_APPLICATION_TASK_TAG == 1 )
        TaskHookFunction_t pxTaskTag;
    #endif

    #if ( configNUM_THREAD_LOCAL_STORAGE_POINTERS > 0 )
        void * pvThreadLocalStoragePointers[ configNUM_THREAD_LOCAL_STORAGE_POINTERS ];
    #endif

    #if ( configGENERATE_RUN_TIME_STATS == 1 )
        configRUN_TIME_COUNTER_TYPE ulRunTimeCounter; /*< Stores the amount of time the task has spent in the Running state. */
    #endif

    #if ( configUSE_NEWLIB_REENTRANT == 1 )

        /* Allocate a Newlib reent structure that is specific to this task.
         * Note Newlib support has been included by popular demand, but is not
         * used by the FreeRTOS maintainers themselves.  FreeRTOS is not
         * responsible for resulting newlib operation.  User must be familiar with
         * newlib and must provide system-wide implementations of the necessary
         * stubs. Be warned that (at the time of writing) the current newlib design
         * implements a system-wide malloc() that must be provided with locks.
         *
         * See the third party link http://www.nadler.com/embedded/newlibAndFreeRTOS.html
         * for additional information. */
        struct  _reent xNewLib_reent;
    #endif

    #if ( configUSE_TASK_NOTIFICATIONS == 1 )
        volatile uint32_t ulNotifiedValue[ configTASK_NOTIFICATION_ARRAY_ENTRIES ];
        volatile uint8_t ucNotifyState[ configTASK_NOTIFICATION_ARRAY_ENTRIES ];
    #endif

    /* See the comments in FreeRTOS.h with the definition of
     * tskSTATIC_AND_DYNAMIC_ALLOCATION_POSSIBLE. */
    #if ( tskSTATIC_AND_DYNAMIC_ALLOCATION_POSSIBLE != 0 ) /*lint !e731 !e9029 Macro has been consolidated for readability reasons. */
        uint8_t ucStaticallyAllocated;                     /*< Set to pdTRUE if the task is a statically allocated to ensure no attempt is made to free the memory. */
    #endif

    #if ( INCLUDE_xTaskAbortDelay == 1 )
        uint8_t ucDelayAborted;
    #endif

    #if ( configUSE_POSIX_ERRNO == 1 )
        int iTaskErrno;
    #endif
} tskTCB;
```

实现的地方

```c
StackType_t * pxStack;

/* Allocate space for the stack used by the task being created. */
pxStack = pvPortMallocStack( ( ( ( size_t ) usStackDepth ) * sizeof( StackType_t ) ) ); /*lint !e9079 All values returned by pvPortMalloc() have at least the alignment required by the MCU's stack and this allocation is the stack. */

if( pxStack != NULL )
{
    /* Allocate space for the TCB. */
    pxNewTCB = ( TCB_t * ) pvPortMalloc( sizeof( TCB_t ) ); /*lint !e9087 !e9079 All values returned by pvPortMalloc() have at least the alignment required by the MCU's stack, and the first member of TCB_t is always a pointer to the task's stack. */

    if( pxNewTCB != NULL )
    {
        memset( ( void * ) pxNewTCB, 0x00, sizeof( TCB_t ) );

        /* Store the stack location in the TCB. */
        pxNewTCB->pxStack = pxStack;
    }
    else
    {
        /* The stack cannot be used as the TCB was not created.  Free
          * it again. */
        vPortFreeStack( pxStack );
    }
}
else
{
    pxNewTCB = NULL;
}
```

实现还是较为简单的

#### 初始化TCB成员变量

> 初始化一些成员数据

#### 加入就绪列表

prvAddNewTaskToReadyList-->prvAddTaskToReadyList

```c
static void prvAddNewTaskToReadyList( TCB_t * pxNewTCB )
{
    /* Ensure interrupts don't access the task lists while the lists are being
     * updated. */
    taskENTER_CRITICAL();
    {
        uxCurrentNumberOfTasks++;

        if( pxCurrentTCB == NULL )
        {
            /* There are no other tasks, or all the other tasks are in
             * the suspended state - make this the current task. */
            pxCurrentTCB = pxNewTCB;

            if( uxCurrentNumberOfTasks == ( UBaseType_t ) 1 )
            {
                /* This is the first task to be created so do the preliminary
                 * initialisation required.  We will not recover if this call
                 * fails, but we will report the failure. */
                prvInitialiseTaskLists();
            }
            else
            {
                mtCOVERAGE_TEST_MARKER();
            }
        }
        else
        {
            /* If the scheduler is not already running, make this task the
             * current task if it is the highest priority task to be created
             * so far. */
            if( xSchedulerRunning == pdFALSE )
            {
                if( pxCurrentTCB->uxPriority <= pxNewTCB->uxPriority )
                {
                    pxCurrentTCB = pxNewTCB;
                }
                else
                {
                    mtCOVERAGE_TEST_MARKER();
                }
            }
            else
            {
                mtCOVERAGE_TEST_MARKER();
            }
        }

        uxTaskNumber++;

        #if ( configUSE_TRACE_FACILITY == 1 )
        {
            /* Add a counter into the TCB for tracing only. */
            pxNewTCB->uxTCBNumber = uxTaskNumber;
        }
        #endif /* configUSE_TRACE_FACILITY */
        traceTASK_CREATE( pxNewTCB );

        prvAddTaskToReadyList( pxNewTCB );

        portSETUP_TCB( pxNewTCB );
    }
    taskEXIT_CRITICAL();

    if( xSchedulerRunning != pdFALSE )
    {
        /* If the created task is of a higher priority than the current task
         * then it should run now. */
        if( pxCurrentTCB->uxPriority < pxNewTCB->uxPriority )
        {
            taskYIELD_IF_USING_PREEMPTION();
        }
        else
        {
            mtCOVERAGE_TEST_MARKER();
        }
    }
    else
    {
        mtCOVERAGE_TEST_MARKER();
    }
}
```

```c
#define prvAddTaskToReadyList( pxTCB )                                                                 \
    traceMOVED_TASK_TO_READY_STATE( pxTCB );                                                           \
    taskRECORD_READY_PRIORITY( ( pxTCB )->uxPriority );                                                \
    listINSERT_END( &( pxReadyTasksLists[ ( pxTCB )->uxPriority ] ), &( ( pxTCB )->xStateListItem ) ); \
    tracePOST_MOVED_TASK_TO_READY_STATE( pxTCB )
```

此时任务就加入了就绪列表,可以被调度器进行调度了

### 任务调度

> 任务调度的基础

+ 可调用中断(可选)
+ 定时器中断


#### 调度器核心

```c
void vTaskSwitchContext(void)
{
    if( uxSchedulerSuspended != ( UBaseType_t ) pdFALSE )
    {
        /* The scheduler is currently suspended - do not allow a context
         * switch. */
        xYieldPending = pdTRUE;
    }
    else
    {
        xYieldPending = pdFALSE;
        traceTASK_SWITCHED_OUT();

        #if ( configGENERATE_RUN_TIME_STATS == 1 )
        {
            #ifdef portALT_GET_RUN_TIME_COUNTER_VALUE
                portALT_GET_RUN_TIME_COUNTER_VALUE( ulTotalRunTime );
            #else
                ulTotalRunTime = portGET_RUN_TIME_COUNTER_VALUE();
            #endif

            /* Add the amount of time the task has been running to the
             * accumulated time so far.  The time the task started running was
             * stored in ulTaskSwitchedInTime.  Note that there is no overflow
             * protection here so count values are only valid until the timer
             * overflows.  The guard against negative values is to protect
             * against suspect run time stat counter implementations - which
             * are provided by the application, not the kernel. */
            if( ulTotalRunTime > ulTaskSwitchedInTime )
            {
                pxCurrentTCB->ulRunTimeCounter += ( ulTotalRunTime - ulTaskSwitchedInTime );
            }
            else
            {
                mtCOVERAGE_TEST_MARKER();
            }

            ulTaskSwitchedInTime = ulTotalRunTime;
        }
        #endif /* configGENERATE_RUN_TIME_STATS */

        /* Check for stack overflow, if configured. */
        taskCHECK_FOR_STACK_OVERFLOW();

        /* Before the currently running task is switched out, save its errno. */
        #if ( configUSE_POSIX_ERRNO == 1 )
        {
            pxCurrentTCB->iTaskErrno = FreeRTOS_errno;
        }
        #endif

        /* Select a new task to run using either the generic C or port
         * optimised asm code. */
        taskSELECT_HIGHEST_PRIORITY_TASK(); /*lint !e9079 void * is used as this macro is used with timers and co-routines too.  Alignment is known to be fine as the type of the pointer stored and retrieved is the same. */
        traceTASK_SWITCHED_IN();

        /* After the new task is switched in, update the global errno. */
        #if ( configUSE_POSIX_ERRNO == 1 )
        {
            FreeRTOS_errno = pxCurrentTCB->iTaskErrno;
        }
        #endif

        #if ( ( configUSE_NEWLIB_REENTRANT == 1 ) || ( configUSE_C_RUNTIME_TLS_SUPPORT == 1 ) )
        {
            /* Switch C-Runtime's TLS Block to point to the TLS
             * Block specific to this task. */
            configSET_TLS_BLOCK( pxCurrentTCB->xTLSBlock );
        }
        #endif
    }
}
```

分析M3的调度实现

```c
void xPortPendSVHandler( void )
{
    /* This is a naked function. */

    __asm volatile
    (
        "	mrs r0, psp							\n"
        "	isb									\n"
        "										\n"
        "	ldr	r3, pxCurrentTCBConst			\n"/* Get the location of the current TCB. */
        "	ldr	r2, [r3]						\n"
        "										\n"
        "	stmdb r0!, {r4-r11}					\n"/* Save the remaining registers. */
        "	str r0, [r2]						\n"/* Save the new top of stack into the first member of the TCB. */
        "										\n"
        "	stmdb sp!, {r3, r14}				\n"
        "	mov r0, %0							\n"
        "	msr basepri, r0						\n"
        "	bl vTaskSwitchContext				\n"
        "	mov r0, #0							\n"
        "	msr basepri, r0						\n"
        "	ldmia sp!, {r3, r14}				\n"
        "										\n"/* Restore the context, including the critical nesting count. */
        "	ldr r1, [r3]						\n"
        "	ldr r0, [r1]						\n"/* The first item in pxCurrentTCB is the task top of stack. */
        "	ldmia r0!, {r4-r11}					\n"/* Pop the registers. */
        "	msr psp, r0							\n"
        "	isb									\n"
        "	bx r14								\n"
        "										\n"
        "	.align 4							\n"
        "pxCurrentTCBConst: .word pxCurrentTCB	\n"
        ::"i" ( configMAX_SYSCALL_INTERRUPT_PRIORITY )
    );
}
```

分析A9实现

```c

vTaskSwitchContextConst: .word vTaskSwitchContext

.macro portSAVE_CONTEXT

	/* Save the LR and SPSR onto the system mode stack before switching to
	system mode to save the remaining system mode registers. */
	SRSDB	sp!, #SYS_MODE
	CPS		#SYS_MODE
	PUSH	{R0-R12, R14}

	/* Push the critical nesting count. */
	LDR		R2, ulCriticalNestingConst
	LDR		R1, [R2]
	PUSH	{R1}

	/* Does the task have a floating point context that needs saving?  If
	ulPortTaskHasFPUContext is 0 then no. */
	LDR		R2, ulPortTaskHasFPUContextConst
	LDR		R3, [R2]
	CMP		R3, #0

	/* Save the floating point context, if any. */
	FMRXNE  R1,  FPSCR
	VPUSHNE {D0-D15}
	VPUSHNE	{D16-D31}
	PUSHNE	{R1}

	/* Save ulPortTaskHasFPUContext itself. */
	PUSH	{R3}

	/* Save the stack pointer in the TCB. */
	LDR		R0, pxCurrentTCBConst
	LDR		R1, [R0]
	STR		SP, [R1]

	.endm

; /**********************************************************************/

.macro portRESTORE_CONTEXT

	/* Set the SP to point to the stack of the task being restored. */
	LDR		R0, pxCurrentTCBConst
	LDR		R1, [R0]
	LDR		SP, [R1]

	/* Is there a floating point context to restore?  If the restored
	ulPortTaskHasFPUContext is zero then no. */
	LDR		R0, ulPortTaskHasFPUContextConst
	POP		{R1}
	STR		R1, [R0]
	CMP		R1, #0

	/* Restore the floating point context, if any. */
	POPNE 	{R0}
	VPOPNE	{D16-D31}
	VPOPNE	{D0-D15}
	VMSRNE  FPSCR, R0

	/* Restore the critical section nesting depth. */
	LDR		R0, ulCriticalNestingConst
	POP		{R1}
	STR		R1, [R0]

	/* Ensure the priority mask is correct for the critical nesting depth. */
	LDR		R2, ulICCPMRConst
	LDR		R2, [R2]
	CMP		R1, #0
	MOVEQ	R4, #255
	LDRNE	R4, ulMaxAPIPriorityMaskConst
	LDRNE	R4, [R4]
	STR		R4, [R2]

	/* Restore all system mode registers other than the SP (which is already
	being used). */
	POP		{R0-R12, R14}

	/* Return to the task code, loading CPSR on the way. */
	RFEIA	sp!

	.endm

.align 4
.type FreeRTOS_SWI_Handler, %function
FreeRTOS_SWI_Handler:
	/* Save the context of the current task and select a new task to run. */
	portSAVE_CONTEXT
	LDR R0, vTaskSwitchContextConst
	BLX	R0
	portRESTORE_CONTEXT

```

中断实现

```asm

.extern FreeRTOS_IRQ_Handler
.extern FreeRTOS_SWI_Handler

.section .freertos_vectors
_freertos_vector_table:
	B	  _boot
	B	  FreeRTOS_Undefined
	ldr   pc, _swi
	B	  FreeRTOS_PrefetchAbortHandler
	B	  FreeRTOS_DataAbortHandler
	NOP	  /* Placeholder for address exception vector*/
	LDR   PC, _irq
	B	  FreeRTOS_FIQHandler

_irq:   .word FreeRTOS_IRQ_Handler
_swi:   .word FreeRTOS_SWI_Handler

```

主动放弃运行,强制开启调度

```c
#define taskYIELD()                        portYIELD()

// M3实现
#define portYIELD()                                 \
{                                                   \
    /* Set a PendSV to request a context switch. */ \
    portNVIC_INT_CTRL_REG = portNVIC_PENDSVSET_BIT; \
                                                    \
    /* Barriers are normally not required but do ensure the code is completely \
      * within the specified behaviour for the architecture. */ \
    __asm volatile ( "dsb" ::: "memory" );                     \
    __asm volatile ( "isb" );                                  \
}

// A9实现
#define portYIELD() __asm volatile ( "SWI 0" ::: "memory" );
```

### 任务状态

> 心跳实现

M3实现

```c
void xPortPendSVHandler( void )
{
    /* This is a naked function. */

    __asm volatile
    (
        "	mrs r0, psp							\n"
        "	isb									\n"
        "										\n"
        "	ldr	r3, pxCurrentTCBConst			\n"/* Get the location of the current TCB. */
        "	ldr	r2, [r3]						\n"
        "										\n"
        "	stmdb r0!, {r4-r11}					\n"/* Save the remaining registers. */
        "	str r0, [r2]						\n"/* Save the new top of stack into the first member of the TCB. */
        "										\n"
        "	stmdb sp!, {r3, r14}				\n"
        "	mov r0, %0							\n"
        "	msr basepri, r0						\n"
        "	bl vTaskSwitchContext				\n"
        "	mov r0, #0							\n"
        "	msr basepri, r0						\n"
        "	ldmia sp!, {r3, r14}				\n"
        "										\n"/* Restore the context, including the critical nesting count. */
        "	ldr r1, [r3]						\n"
        "	ldr r0, [r1]						\n"/* The first item in pxCurrentTCB is the task top of stack. */
        "	ldmia r0!, {r4-r11}					\n"/* Pop the registers. */
        "	msr psp, r0							\n"
        "	isb									\n"
        "	bx r14								\n"
        "										\n"
        "	.align 4							\n"
        "pxCurrentTCBConst: .word pxCurrentTCB	\n"
        ::"i" ( configMAX_SYSCALL_INTERRUPT_PRIORITY )
    );
}
```

A9实现

```asm
.align 4
.type FreeRTOS_IRQ_Handler, %function
FreeRTOS_IRQ_Handler:
	/* Return to the interrupted instruction. */
	SUB		lr, lr, #4

	/* Push the return address and SPSR. */
	PUSH	{lr}
	MRS		lr, SPSR
	PUSH	{lr}

	/* Change to supervisor mode to allow reentry. */
	CPS		#SVC_MODE

	/* Push used registers. */
	PUSH	{r0-r4, r12}

	/* Increment nesting count.  r3 holds the address of ulPortInterruptNesting
	for future use.  r1 holds the original ulPortInterruptNesting value for
	future use. */
	LDR		r3, ulPortInterruptNestingConst
	LDR		r1, [r3]
	ADD		r4, r1, #1
	STR		r4, [r3]

	/* Read value from the interrupt acknowledge register, which is stored in r0
	for future parameter and interrupt clearing use. */
	LDR 	r2, ulICCIARConst
	LDR		r2, [r2]
	LDR		r0, [r2]

	/* Ensure bit 2 of the stack pointer is clear.  r2 holds the bit 2 value for
	future use.  _RB_ Does this ever actually need to be done provided the start
	of the stack is 8-byte aligned? */
	MOV		r2, sp
	AND		r2, r2, #4
	SUB		sp, sp, r2

	/* Call the interrupt handler.  r4 pushed to maintain alignment. */
	PUSH	{r0-r4, lr}
	LDR		r1, vApplicationIRQHandlerConst
	BLX		r1
	POP		{r0-r4, lr}
	ADD		sp, sp, r2

	CPSID	i
	DSB
	ISB

	/* Write the value read from ICCIAR to ICCEOIR. */
	LDR 	r4, ulICCEOIRConst
	LDR		r4, [r4]
	STR		r0, [r4]

	/* Restore the old nesting count. */
	STR		r1, [r3]

	/* A context switch is never performed if the nesting count is not 0. */
	CMP		r1, #0
	BNE		exit_without_switch

	/* Did the interrupt request a context switch?  r1 holds the address of
	ulPortYieldRequired and r0 the value of ulPortYieldRequired for future
	use. */
	LDR		r1, =ulPortYieldRequired
	LDR		r0, [r1]
	CMP		r0, #0
	BNE		switch_before_exit

exit_without_switch:
	/* No context switch.  Restore used registers, LR_irq and SPSR before
	returning. */
	POP		{r0-r4, r12}
	CPS		#IRQ_MODE
	POP		{LR}
	MSR		SPSR_cxsf, LR
	POP		{LR}
	MOVS	PC, LR

switch_before_exit:
	/* A context swtich is to be performed.  Clear the context switch pending
	flag. */
	MOV		r0, #0
	STR		r0, [r1]

	/* Restore used registers, LR-irq and SPSR before saving the context
	to the task stack. */
	POP		{r0-r4, r12}
	CPS		#IRQ_MODE
	POP		{LR}
	MSR		SPSR_cxsf, LR
	POP		{LR}
	portSAVE_CONTEXT

	/* Call the function that selects the new task to execute.
	vTaskSwitchContext() if vTaskSwitchContext() uses LDRD or STRD
	instructions, or 8 byte aligned stack allocated data.  LR does not need
	saving as a new LR will be loaded by portRESTORE_CONTEXT anyway. */
	LDR		R0, vTaskSwitchContextConst
	BLX		R0

	/* Restore the context of, and branch to, the task selected to execute
	next. */
	portRESTORE_CONTEXT
```
