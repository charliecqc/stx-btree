#ifndef _NV_BACKEND_H
#define _NV_BACKEND_H
#include <sys/mman.h>
#define  mb() asm volatile("mfence" ::: "memory")
#define  rmb() asm volatile("lfence" ::: "memory")
#define  wmb() asm volatile("sfence" ::: "memory")

void *nv_malloc(size_t length) {
#ifdef USING_NVRAM
#else
	return malloc(length);
#endif
}

void nv_flush(void *m, size_t length) {
#ifdef USING_NVRAM
	//using mb();clflush();mb();
	mb();
	_mm_clflush(m);
#else
	mb();
	msync(m, length, MS_SYNC);
	mb();
#endif
}

void *nv_memcpy(void *dest, const void *src, size_t num)
{
#ifdef USING_NVRAM

#else
	return memcpy(dest, src, num);
#endif
}

void nv_free(void *m)
{
#ifdef USING_PMEM
#else
	free(m);
#endif
}

#endif
