/* $Id: shn.h 182 2009-03-12 08:21:53Z zagor $ */
/* Shannon: Shannon stream cipher and MAC header files */

/*
THIS SOFTWARE IS PROVIDED ``AS IS'' AND ANY EXPRESS OR IMPLIED
WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE AND AGAINST
INFRINGEMENT ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR OR
CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

#ifndef _SHN_DEFINED
#define _SHN_DEFINED 1

#include <limits.h>

#if __STDC_VERSION__ >= 199901
#include <stdint.h>
#endif

#define N 16
#define WORDSIZE 32
#define UCHAR unsigned char

#if __STDC_VERSION__ >= 199901
#define WORD uint32_t
#define WORD_MAX UINT32_MAX
#elif UINT_MAX >= 0xffffffff
#define WORD unsigned int
#define WORD_MAX UINT_MAX
#else
#define WORD unsigned long
#define WORD_MAX ULONG_MAX
#endif

#if WORD_MAX == 0xffffffff
#define ROTL(w,x) (((w) << (x))|((w) >> (32 - (x))))
#define ROTR(w,x) (((w) >> (x))|((w) << (32 - (x))))
#else
#define ROTL(w,x) (((w) << (x))|(((w) & 0xffffffff) >> (32 - (x))))
#define ROTR(w,x) ((((w) & 0xffffffff) >> (x))|((w) << (32 - (x))))
#endif

typedef struct
{
	WORD R[N];		/* Working storage for the shift register */
	WORD CRC[N];		/* Working storage for CRC accumulation */
	WORD initR[N];		/* saved register contents */
	WORD konst;		/* key dependent semi-constant */
	WORD sbuf;		/* encryption buffer */
	WORD mbuf;		/* partial word MAC buffer */
	int nbuf;		/* number of part-word stream bits buffered */
} shn_ctx;

/* interface definitions */
void shn_key (shn_ctx * c, UCHAR key[], int keylen);	/* set key */
void shn_nonce (shn_ctx * c, UCHAR nonce[], int nlen);	/* set Init Vector */
void shn_stream (shn_ctx * c, UCHAR * buf, int nbytes);	/* stream cipher */
void shn_maconly (shn_ctx * c, UCHAR * buf, int nbytes);	/* accumulate MAC */
void shn_encrypt (shn_ctx * c, UCHAR * buf, int nbytes);	/* encrypt + MAC */
void shn_decrypt (shn_ctx * c, UCHAR * buf, int nbytes);	/* decrypt + MAC */
void shn_finish (shn_ctx * c, UCHAR * buf, int nbytes);	/* finalise MAC */
#endif /* _SHN_DEFINED */
