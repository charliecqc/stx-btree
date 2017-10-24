#ifndef _NV_BACKEND_H
#define _NV_BACKEND_H
#include <sys/mman.h>
#include <stdio.h>
#include <stdint.h>
#define  mb() asm volatile("mfence" ::: "memory")
#define  rmb() asm volatile("lfence" ::: "memory")
#define  wmb() asm volatile("sfence" ::: "memory")
#define _mm_clwb(addr)\
	asm volatile(".byte 0x66; xsaveopt %0" : "+m" (*(volatile char *)addr));

inline void clflush(volatile void *p) 
{
	asm volatile("clflush (%0)" ::"r"(p));
}

void *nv_malloc(size_t length) {
#ifdef USING_NVRAM
#else
	return malloc(length);
#endif
}

void nv_flush(void *m) {
	//using mb();clflush();mb();
	mb();
	clflush(m);
//	_mm_clwb(m);
#if 0
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
