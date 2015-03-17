/* $Id: shn.c 182 2009-03-12 08:21:53Z zagor $ */
/* Shannon: Shannon stream cipher and MAC -- reference implementation */

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

/* interface */
#include <stdlib.h>
#include <string.h>
#include "shn.h"

/*
 * FOLD is how many register cycles need to be performed after combining the
 * last byte of key and non-linear feedback, before every byte depends on every
 * byte of the key. This depends on the feedback and nonlinear functions, and
 * on where they are combined into the register. Making it same as the
 * register length is a safe and conservative choice.
 */
#define FOLD N			/* how many iterations of folding to do */
#define INITKONST 0x6996c53a	/* value of KONST to use during key loading */
#define KEYP 13			/* where to insert key/MAC/counter words */

/* some useful macros -- machine independent little-endian */
#define Byte(x,i) ((UCHAR)(((x) >> (8*(i))) & 0xFF))
#define BYTE2WORD(b) ( \
	(((WORD)(b)[3] & 0xFF)<<24) | \
	(((WORD)(b)[2] & 0xFF)<<16) | \
	(((WORD)(b)[1] & 0xFF)<<8) | \
	(((WORD)(b)[0] & 0xFF)) \
)
#define WORD2BYTE(w, b) { \
	(b)[3] = Byte(w,3); \
	(b)[2] = Byte(w,2); \
	(b)[1] = Byte(w,1); \
	(b)[0] = Byte(w,0); \
}
#define XORWORD(w, b) { \
	(b)[3] ^= Byte(w,3); \
	(b)[2] ^= Byte(w,2); \
	(b)[1] ^= Byte(w,1); \
	(b)[0] ^= Byte(w,0); \
}

/* Nonlinear transform (sbox) of a word.
 * There are two slightly different combinations.
 */
static WORD sbox1 (WORD w)
{
	w ^= ROTL (w, 5) | ROTL (w, 7);
	w ^= ROTL (w, 19) | ROTL (w, 22);
	return w;
}

static WORD sbox2 (WORD w)
{
	w ^= ROTL (w, 7) | ROTL (w, 22);
	w ^= ROTL (w, 5) | ROTL (w, 19);
	return w;
}

/* cycle the contents of the register and calculate output word in c->sbuf.
 */
static void cycle (shn_ctx * c)
{
	WORD t;
	int i;

	/* nonlinear feedback function */
	t = c->R[12] ^ c->R[13] ^ c->konst;
	t = sbox1 (t) ^ ROTL (c->R[0], 1);
	/* shift register */
	for (i = 1; i < N; ++i)
		c->R[i - 1] = c->R[i];
	c->R[N - 1] = t;
	t = sbox2 (c->R[2] ^ c->R[15]);
	c->R[0] ^= t;
	c->sbuf = t ^ c->R[8] ^ c->R[12];
}

/* The Shannon MAC function is modelled after the concepts of Phelix and SHA.
 * Basically, words to be accumulated in the MAC are incorporated in two
 * different ways:
 * 1. They are incorporated into the stream cipher register at a place
 *    where they will immediately have a nonlinear effect on the state
 * 2. They are incorporated into bit-parallel CRC-16 registers; the
 *    contents of these registers will be used in MAC finalization.
 */

/* Accumulate a CRC of input words, later to be fed into MAC.
 * This is actually 32 parallel CRC-16s, using the IBM CRC-16
 * polynomial x^16 + x^15 + x^2 + 1.
 */
static void crcfunc (shn_ctx * c, WORD i)
{
	WORD t;
	int j;

	/* Accumulate CRC of input */
	t = c->CRC[0] ^ c->CRC[2] ^ c->CRC[15] ^ i;
	for (j = 1; j < N; ++j)
		c->CRC[j - 1] = c->CRC[j];
	c->CRC[N - 1] = t;
}

/* Normal MAC word processing: do both stream register and CRC.
 */
static void macfunc (shn_ctx * c, WORD i)
{
	crcfunc (c, i);
	c->R[KEYP] ^= i;
}

/* initialise to known state
 */
static void shn_initstate (shn_ctx * c)
{
	int i;

	/* Register initialised to Fibonacci numbers; Counter zeroed. */
	c->R[0] = 1;
	c->R[1] = 1;
	for (i = 2; i < N; ++i)
		c->R[i] = c->R[i - 1] + c->R[i - 2];
	c->konst = INITKONST;
}

/* Save the current register state
 */
static void shn_savestate (shn_ctx * c)
{
	int i;

	for (i = 0; i < N; ++i)
		c->initR[i] = c->R[i];
}

/* initialise to previously saved register state
 */
static void shn_reloadstate (shn_ctx * c)
{
	int i;

	for (i = 0; i < N; ++i)
		c->R[i] = c->initR[i];
}

/* Initialise "konst"
 */
static void shn_genkonst (shn_ctx * c)
{
	c->konst = c->R[0];
}

/* Load key material into the register
 */
#define ADDKEY(k) \
	c->R[KEYP] ^= (k);

/* extra nonlinear diffusion of register for key and MAC */
static void shn_diffuse (shn_ctx * c)
{
	int i;

	for (i = 0; i < FOLD; ++i)
		cycle (c);
}

/* Common actions for loading key material
 * Allow non-word-multiple key and nonce material.
 * Note also initializes the CRC register as a side effect.
 */
static void shn_loadkey (shn_ctx * c, UCHAR key[], int keylen)
{
	int i, j;
	WORD k;
	UCHAR xtra[4];

	/* start folding in key */
	for (i = 0; i < (keylen & ~0x3); i += 4) {
		k = BYTE2WORD (&key[i]);
		ADDKEY (k);
		cycle (c);
	}

	/* if there were any extra key bytes, zero pad to a word */
	if (i < keylen) {
		for (j = 0 /* i unchanged */ ; i < keylen; ++i)
			xtra[j++] = key[i];
		for ( /* j unchanged */ ; j < 4; ++j)
			xtra[j] = 0;
		k = BYTE2WORD (xtra);
		ADDKEY (k);
		cycle (c);
	}

	/* also fold in the length of the key */
	ADDKEY (keylen);
	cycle (c);

	/* save a copy of the register */
	for (i = 0; i < N; ++i)
		c->CRC[i] = c->R[i];

	/* now diffuse */
	shn_diffuse (c);

	/* now xor the copy back -- makes key loading irreversible */
	for (i = 0; i < N; ++i)
		c->R[i] ^= c->CRC[i];
}

/* Published "key" interface
 */
void shn_key (shn_ctx * c, UCHAR key[], int keylen)
{
	shn_initstate (c);
	shn_loadkey (c, key, keylen);
	shn_genkonst (c);	/* in case we proceed to stream generation */
	shn_savestate (c);
	c->nbuf = 0;
}

/* Published "IV" interface
 */
void shn_nonce (shn_ctx * c, UCHAR nonce[], int noncelen)
{
	shn_reloadstate (c);
	c->konst = INITKONST;
	shn_loadkey (c, nonce, noncelen);
	shn_genkonst (c);
	c->nbuf = 0;
}

/* XOR pseudo-random bytes into buffer
 * Note: doesn't play well with MAC functions.
 */
void shn_stream (shn_ctx * c, UCHAR * buf, int nbytes)
{
	UCHAR *endbuf;

	/* handle any previously buffered bytes */
	while (c->nbuf != 0 && nbytes != 0) {
		*buf++ ^= c->sbuf & 0xFF;
		c->sbuf >>= 8;
		c->nbuf -= 8;
		--nbytes;
	}

	/* handle whole words */
	endbuf = &buf[nbytes & ~((WORD) 0x03)];
	while (buf < endbuf) {
		cycle (c);
		XORWORD (c->sbuf, buf);
		buf += 4;
	}

	/* handle any trailing bytes */
	nbytes &= 0x03;
	if (nbytes != 0) {
		cycle (c);
		c->nbuf = 32;
		while (c->nbuf != 0 && nbytes != 0) {
			*buf++ ^= c->sbuf & 0xFF;
			c->sbuf >>= 8;
			c->nbuf -= 8;
			--nbytes;
		}
	}
}

/* accumulate words into MAC without encryption
 * Note that plaintext is accumulated for MAC.
 */
void shn_maconly (shn_ctx * c, UCHAR * buf, int nbytes)
{
	UCHAR *endbuf;

	/* handle any previously buffered bytes */
	if (c->nbuf != 0) {
		while (c->nbuf != 0 && nbytes != 0) {
			c->mbuf ^= (*buf++) << (32 - c->nbuf);
			c->nbuf -= 8;
			--nbytes;
		}
		if (c->nbuf != 0)	/* not a whole word yet */
			return;
		/* LFSR already cycled */
		macfunc (c, c->mbuf);
	}

	/* handle whole words */
	endbuf = &buf[nbytes & ~((WORD) 0x03)];
	while (buf < endbuf) {
		cycle (c);
		macfunc (c, BYTE2WORD (buf));
		buf += 4;
	}

	/* handle any trailing bytes */
	nbytes &= 0x03;
	if (nbytes != 0) {
		cycle (c);
		c->mbuf = 0;
		c->nbuf = 32;
		while (c->nbuf != 0 && nbytes != 0) {
			c->mbuf ^= (*buf++) << (32 - c->nbuf);
			c->nbuf -= 8;
			--nbytes;
		}
	}
}

/* Combined MAC and encryption.
 * Note that plaintext is accumulated for MAC.
 */
void shn_encrypt (shn_ctx * c, UCHAR * buf, int nbytes)
{
	UCHAR *endbuf;
	WORD t = 0;

	/* handle any previously buffered bytes */
	if (c->nbuf != 0) {
		while (c->nbuf != 0 && nbytes != 0) {
			c->mbuf ^= *buf << (32 - c->nbuf);
			*buf ^= (c->sbuf >> (32 - c->nbuf)) & 0xFF;
			++buf;
			c->nbuf -= 8;
			--nbytes;
		}
		if (c->nbuf != 0)	/* not a whole word yet */
			return;
		/* LFSR already cycled */
		macfunc (c, c->mbuf);
	}

	/* handle whole words */
	endbuf = &buf[nbytes & ~((WORD) 0x03)];
	while (buf < endbuf) {
		cycle (c);
		t = BYTE2WORD (buf);
		macfunc (c, t);
		t ^= c->sbuf;
		WORD2BYTE (t, buf);
		buf += 4;
	}

	/* handle any trailing bytes */
	nbytes &= 0x03;
	if (nbytes != 0) {
		cycle (c);
		c->mbuf = 0;
		c->nbuf = 32;
		while (c->nbuf != 0 && nbytes != 0) {
			c->mbuf ^= *buf << (32 - c->nbuf);
			*buf ^= (c->sbuf >> (32 - c->nbuf)) & 0xFF;
			++buf;
			c->nbuf -= 8;
			--nbytes;
		}
	}
}

/* Combined MAC and decryption.
 * Note that plaintext is accumulated for MAC.
 */
void shn_decrypt (shn_ctx * c, UCHAR * buf, int nbytes)
{
	UCHAR *endbuf;
	WORD t = 0;

	/* handle any previously buffered bytes */
	if (c->nbuf != 0) {
		while (c->nbuf != 0 && nbytes != 0) {
			*buf ^= (c->sbuf >> (32 - c->nbuf)) & 0xFF;
			c->mbuf ^= *buf << (32 - c->nbuf);
			++buf;
			c->nbuf -= 8;
			--nbytes;
		}
		if (c->nbuf != 0)	/* not a whole word yet */
			return;
		/* LFSR already cycled */
		macfunc (c, c->mbuf);
	}

	/* handle whole words */
	endbuf = &buf[nbytes & ~((WORD) 0x03)];
	while (buf < endbuf) {
		cycle (c);
		t = BYTE2WORD (buf) ^ c->sbuf;
		macfunc (c, t);
		WORD2BYTE (t, buf);
		buf += 4;
	}

	/* handle any trailing bytes */
	nbytes &= 0x03;
	if (nbytes != 0) {
		cycle (c);
		c->mbuf = 0;
		c->nbuf = 32;
		while (c->nbuf != 0 && nbytes != 0) {
			*buf ^= (c->sbuf >> (32 - c->nbuf)) & 0xFF;
			c->mbuf ^= *buf << (32 - c->nbuf);
			++buf;
			c->nbuf -= 8;
			--nbytes;
		}
	}
}

/* Having accumulated a MAC, finish processing and return it.
 * Note that any unprocessed bytes are treated as if
 * they were encrypted zero bytes, so plaintext (zero) is accumulated.
 */
void shn_finish (shn_ctx * c, UCHAR * buf, int nbytes)
{
	int i;

	/* handle any previously buffered bytes */
	if (c->nbuf != 0) {
		/* LFSR already cycled */
		macfunc (c, c->mbuf);
	}

	/* perturb the MAC to mark end of input.
	 * Note that only the stream register is updated, not the CRC. This is an
	 * action that can't be duplicated by passing in plaintext, hence
	 * defeating any kind of extension attack.
	 */
	cycle (c);
	ADDKEY (INITKONST ^ (c->nbuf << 3));
	c->nbuf = 0;

	/* now add the CRC to the stream register and diffuse it */
	for (i = 0; i < N; ++i)
		c->R[i] ^= c->CRC[i];
	shn_diffuse (c);

	/* produce output from the stream buffer */
	while (nbytes > 0) {
		cycle (c);
		if (nbytes >= 4) {
			WORD2BYTE (c->sbuf, buf);
			nbytes -= 4;
			buf += 4;
		}
		else {
			for (i = 0; i < nbytes; ++i)
				buf[i] = Byte (c->sbuf, i);
			break;
		}
	}
}
