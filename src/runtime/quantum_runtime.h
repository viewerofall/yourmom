// quantum_runtime.h - VQ (Version 3: Quantum) runtime
// .yourmom language — the most cursed fucking language in existence
// "Yo mama so fat, she exists in multiple states simultaneously"
// "Yo mama so goddamn wide, she broke the fucking address space"

#ifndef QUANTUM_RUNTIME_H
#define QUANTUM_RUNTIME_H

#include <stdlib.h>
#include <stdio.h>
#include <time.h>
#include <signal.h>
#include <string.h>
#include <stdint.h>
#include <math.h>
#include <ctype.h>
#include <setjmp.h>

// ── Quantum integer ──────────────────────────────────────────────────────────
typedef struct {
    int* values;       // Possible values
    int count;         // Number of possibilities
    int collapsed_idx; // -1 if still in superposition, else index of collapsed value
    int collapsed_val; // The actual collapsed value (cached)
} quantum_int;

// ── Heisenberg variable (re-collapses on EVERY observation) ─────────────────
typedef struct {
    int* values;
    int count;
} quantum_heisenberg;

// ── Quaternion type ──────────────────────────────────────────────────────────
typedef struct {
    double w, x, y, z;
    int collapsed;
} quantum_quaternion;

// ── Transfinite number type ──────────────────────────────────────────────────
typedef struct {
    const char* symbol;
    int order;
    int collapsed;
} quantum_transfinite;

// ── Lazy int ─────────────────────────────────────────────────────────────────
typedef struct { int computed; int val; } lazy_int;
#define LAZY_INT_INIT { 0, 0 }
#define LAZY_GET(L, F) ((L).computed ? (L).val : ((L).computed=1, (L).val=(F)))

// ── setjmp/longjmp try-catch support ─────────────────────────────────────────
#define _DADDY_ISSUES_MAX_DEPTH 16
static jmp_buf _daddy_issues_stack[_DADDY_ISSUES_MAX_DEPTH];
static int _daddy_issues_depth = 0;

// Global RNG state
static int _quantum_rng_initialized = 0;

void _quantum_init_rng() {
    if (!_quantum_rng_initialized) {
        srand(time(NULL));
        _quantum_rng_initialized = 1;
    }
}

// ── quantum_int helpers ──────────────────────────────────────────────────────

static inline quantum_int _quantum_create(int val) {
    quantum_int q;
    q.values = malloc(sizeof(int));
    q.values[0] = val;
    q.count = 1;
    q.collapsed_idx = 0;
    q.collapsed_val = val;
    return q;
}

static inline int _quantum_eq(quantum_int* a, quantum_int* b);
static inline int _quantum_lt(quantum_int* a, quantum_int* b);
static inline int _quantum_gt(quantum_int* a, quantum_int* b);

static inline int _quantum_eq_val(quantum_int* a, int b_val) {
    quantum_int b = _quantum_create(b_val);
    int result = _quantum_eq(a, &b);
    free(b.values);
    return result;
}

static inline int _quantum_lt_val(quantum_int* a, int b_val) {
    quantum_int b = _quantum_create(b_val);
    int result = _quantum_lt(a, &b);
    free(b.values);
    return result;
}

static inline int _quantum_gt_val(quantum_int* a, int b_val) {
    quantum_int b = _quantum_create(b_val);
    int result = _quantum_gt(a, &b);
    free(b.values);
    return result;
}

quantum_int _quantum_superposition(int* vals, int count) {
    _quantum_init_rng();
    quantum_int q;
    q.values = malloc(sizeof(int) * count);
    memcpy(q.values, vals, sizeof(int) * count);
    q.count = count;
    q.collapsed_idx = -1;
    q.collapsed_val = 0;
    return q;
}

int _quantum_observe(quantum_int* q) {
    if (q->collapsed_idx == -1) {
        _quantum_init_rng();
        q->collapsed_idx = rand() % q->count;
        q->collapsed_val = q->values[q->collapsed_idx];
    }
    return q->collapsed_val;
}

int _quantum_is_collapsed(quantum_int* q) {
    return q->collapsed_idx != -1;
}

int _quantum_get(quantum_int* q) {
    return _quantum_observe(q);
}

quantum_int _quantum_add(quantum_int* a, quantum_int* b) {
    return _quantum_create(_quantum_get(a) + _quantum_get(b));
}

quantum_int _quantum_sub(quantum_int* a, quantum_int* b) {
    return _quantum_create(_quantum_get(a) - _quantum_get(b));
}

quantum_int _quantum_mul(quantum_int* a, quantum_int* b) {
    return _quantum_create(_quantum_get(a) * _quantum_get(b));
}

quantum_int _quantum_div(quantum_int* a, quantum_int* b) {
    return _quantum_create(_quantum_get(a) / _quantum_get(b));
}

static inline int _quantum_eq(quantum_int* a, quantum_int* b) {
    return _quantum_get(a) == _quantum_get(b);
}

static inline int _quantum_lt(quantum_int* a, quantum_int* b) {
    return _quantum_get(a) < _quantum_get(b);
}

static inline int _quantum_gt(quantum_int* a, quantum_int* b) {
    return _quantum_get(a) > _quantum_get(b);
}

// ── Heisenberg helpers ────────────────────────────────────────────────────────

static inline quantum_heisenberg _quantum_heisenberg_create(int* vals, int count) {
    _quantum_init_rng();
    quantum_heisenberg h;
    h.values = malloc(sizeof(int) * count);
    memcpy(h.values, vals, sizeof(int) * count);
    h.count = count;
    return h;
}

// Always re-collapses — the observer effect, bitch
static inline int _quantum_heisenberg_get(quantum_heisenberg* h) {
    _quantum_init_rng();
    return h->values[rand() % h->count];
}

#define HEISENBERG(...) ({ \
    int _hvals[] = {__VA_ARGS__}; \
    _quantum_heisenberg_create(_hvals, sizeof(_hvals)/sizeof(int)); \
})

// ── quantum_sort ─────────────────────────────────────────────────────────────

static int _int_cmp(const void* a, const void* b) {
    return (*(int*)a) - (*(int*)b);
}

quantum_int _quantum_sort_values(int* vals, int count) {
    _quantum_init_rng();
    int* sorted = malloc(sizeof(int) * count);
    memcpy(sorted, vals, sizeof(int) * count);
    qsort(sorted, count, sizeof(int), _int_cmp);
    // Print sorted array and smallest
    printf("[");
    for (int i = 0; i < count; i++) {
        printf("%d", sorted[i]);
        if (i < count - 1) printf(", ");
    }
    int smallest = sorted[0];
    free(sorted);
    quantum_int q;
    q.values = malloc(sizeof(int) * count);
    memcpy(q.values, vals, sizeof(int) * count);
    qsort(q.values, count, sizeof(int), _int_cmp);
    q.count = count;
    q.collapsed_idx = 0;
    q.collapsed_val = smallest;
    printf("] -> %d\n", smallest);
    return q;
}

#define QUANTUM_SORT(...) ({ \
    int _svals[] = {__VA_ARGS__}; \
    _quantum_sort_values(_svals, sizeof(_svals)/sizeof(int)); \
})

// ── yo_mama_so_loud (uppercase shout) ────────────────────────────────────────

static inline void _quantum_shout(const char* str) {
    printf("🔊 BITCH: ");
    for (int i = 0; str[i]; i++) {
        putchar(toupper((unsigned char)str[i]));
    }
    putchar('\n');
}

// ── quantum_quaternion helpers ────────────────────────────────────────────────

static inline quantum_quaternion _quantum_quaternion_all() {
    _quantum_init_rng();
    quantum_quaternion q;
    q.w = ((double)rand() / RAND_MAX) * 20.0 - 10.0;
    q.x = ((double)rand() / RAND_MAX) * 20.0 - 10.0;
    q.y = ((double)rand() / RAND_MAX) * 20.0 - 10.0;
    q.z = ((double)rand() / RAND_MAX) * 20.0 - 10.0;
    q.collapsed = 0;
    return q;
}

static inline void _quantum_quaternion_get(quantum_quaternion* q) {
    if (!q->collapsed) {
        *q = _quantum_quaternion_all();
        q->collapsed = 1;
    }
}

static inline void _quantum_quaternion_print(quantum_quaternion* q) {
    _quantum_quaternion_get(q);
    char xsign = q->x >= 0 ? '+' : '-';
    char ysign = q->y >= 0 ? '+' : '-';
    char zsign = q->z >= 0 ? '+' : '-';
    printf("%g%c%gi%c%gj%c%gk\n",
        q->w,
        xsign, fabs(q->x),
        ysign, fabs(q->y),
        zsign, fabs(q->z));
}

#define QUATERNION_ALL (_quantum_quaternion_all())

// ── quantum_transfinite helpers ───────────────────────────────────────────────

static inline quantum_transfinite _quantum_transfinite_all() {
    _quantum_init_rng();
    static const struct { const char* symbol; int order; } TRANSFINITES[] = {
        { "aleph_0",  0 },
        { "aleph_1",  1 },
        { "aleph_2",  2 },
        { "omega",    3 },
        { "omega+1",  4 },
        { "omega^2",  5 },
        { "omega^omega", 6 },
        { "epsilon_0", 7 },
    };
    int count = sizeof(TRANSFINITES) / sizeof(TRANSFINITES[0]);
    int idx = rand() % count;
    quantum_transfinite t;
    t.symbol = TRANSFINITES[idx].symbol;
    t.order  = TRANSFINITES[idx].order;
    t.collapsed = 0;
    return t;
}

static inline void _quantum_transfinite_get(quantum_transfinite* t) {
    if (!t->collapsed) {
        *t = _quantum_transfinite_all();
        t->collapsed = 1;
    }
}

static inline void _quantum_transfinite_print(quantum_transfinite* t) {
    _quantum_transfinite_get(t);
    printf("%s\n", t->symbol);
}

#define TRANSFINITE_ALL (_quantum_transfinite_all())

// ── self-modifying source ─────────────────────────────────────────────────────

static inline void _quantum_rewrite_source(const char* path) {
    FILE* f = fopen(path, "r");
    if (!f) {
        fprintf(stderr, "what in the actual fuck, can't open %s\n", path);
        return;
    }
    fseek(f, 0, SEEK_END);
    long sz = ftell(f);
    rewind(f);
    char* buf = malloc(sz + 1);
    fread(buf, 1, sz, f);
    buf[sz] = '\0';
    fclose(f);

    f = fopen(path, "w");
    if (!f) {
        fprintf(stderr, "holy shitballs, can't write %s\n", path);
        free(buf);
        return;
    }
    fprintf(f, "// yo mama so meta, she rewrote herself\n%s", buf);
    fclose(f);
    free(buf);
}

// ── Error handler ─────────────────────────────────────────────────────────────

static char* _error_jokes[] = {
    "Yo mama so fat, her ass caused a goddamn stack overflow",
    "Yo mama so dumb, she dereferenced a NULL pointer — what the fuck",
    "Yo mama so ugly, even the CPU refused to execute that shit",
    "Yo mama so slow, she caused heap corruption, the dumb bitch",
    "Yo mama so cheap, she freed memory twice like a damn bastard",
    "Yo mama so fat, she exceeded the fucking address space",
    "Yo mama so random, she corrupted the RNG seed — holy shit",
    "Yo mama so nasty, she segfaulted before she even started, crap",
    "Yo mama so dumb, she wrote to a read-only page like an ass",
    "Yo mama so wide, she overflowed the goddamn heap"
};

void _yo_mama_so_broken(int sig) {
    _quantum_init_rng();
    fprintf(stderr, "\n💀 WHAT IN THE ACTUAL FUCK (signal %d): %s\n",
        sig, _error_jokes[rand() % 10]);
    exit(1);
}

void _quantum_init_errors() {
    signal(SIGSEGV, _yo_mama_so_broken);
    signal(SIGABRT, _yo_mama_so_broken);
    signal(SIGFPE,  _yo_mama_so_broken);
    _quantum_init_rng();
}

// ── Macros ────────────────────────────────────────────────────────────────────

#define SUPERPOSITION(...) ({ \
    int _vals[] = {__VA_ARGS__}; \
    _quantum_superposition(_vals, sizeof(_vals)/sizeof(int)); \
})

static inline quantum_int _qint(int val) {
    return _quantum_create(val);
}

// ── INT_ALL ───────────────────────────────────────────────────────────────────

static inline quantum_int _quantum_int_all() {
    _quantum_init_rng();
    unsigned int bits = ((unsigned int)rand() << 16) ^ (unsigned int)rand();
    int val = (int)bits;
    quantum_int q;
    q.values = malloc(sizeof(int));
    q.values[0] = val;
    q.count = 1;
    q.collapsed_idx = -1;
    q.collapsed_val = val;
    return q;
}

#define INT_ALL (_quantum_int_all())

// ── quantum_float ─────────────────────────────────────────────────────────────

typedef struct {
    double val;
    int    collapsed;
} quantum_float;

static inline double _random_double() {
    _quantum_init_rng();
    int sign = rand() % 2 ? 1 : -1;
    int exp10 = (rand() % 617) - 308;
    double mantissa = 1.0 + (double)rand() / (double)RAND_MAX;
    return sign * mantissa * pow(10.0, exp10);
}

static inline quantum_float _quantum_float_all() {
    quantum_float q;
    q.val = _random_double();
    q.collapsed = 0;
    return q;
}

static inline double _quantum_float_get(quantum_float* q) {
    if (!q->collapsed) {
        q->val = _random_double();
        q->collapsed = 1;
    }
    return q->val;
}

#define REAL_ALL (_quantum_float_all())

// ── quantum_rational ──────────────────────────────────────────────────────────

typedef struct {
    int    num;
    int    den;
    int    collapsed;
} quantum_rational;

static inline int _gcd(int a, int b) {
    if (a < 0) a = -a;
    if (b < 0) b = -b;
    while (b) { int t = b; b = a % b; a = t; }
    return a ? a : 1;
}

static inline quantum_rational _quantum_rational_all() {
    _quantum_init_rng();
    quantum_rational q;
    q.num = (int)(((unsigned int)rand() << 16) ^ (unsigned int)rand());
    do { q.den = (int)(((unsigned int)rand() << 16) ^ (unsigned int)rand()); }
    while (q.den == 0);
    int g = _gcd(q.num < 0 ? -q.num : q.num, q.den < 0 ? -q.den : q.den);
    q.num /= g; q.den /= g;
    if (q.den < 0) { q.num = -q.num; q.den = -q.den; }
    q.collapsed = 0;
    return q;
}

static inline void _quantum_rational_get(quantum_rational* q) {
    if (!q->collapsed) {
        *q = _quantum_rational_all();
        q->collapsed = 1;
    }
}

static inline void _quantum_rational_print(quantum_rational* q) {
    _quantum_rational_get(q);
    if (q->den == 1)
        printf("%d\n", q->num);
    else
        printf("%d/%d\n", q->num, q->den);
}

#define RATIONAL_ALL (_quantum_rational_all())

// ── quantum_irrational ────────────────────────────────────────────────────────

typedef struct {
    double      val;
    const char* name;
    int         collapsed;
} quantum_irrational;

static inline quantum_irrational _quantum_irrational_all() {
    _quantum_init_rng();
    static const struct { double val; const char* name; } IRRATIONALS[] = {
        { 3.14159265358979323846,  "pi"      },
        { 2.71828182845904523536,  "e"       },
        { 1.41421356237309504880,  "sqrt2"   },
        { 1.73205080756887729352,  "sqrt3"   },
        { 2.23606797749978969640,  "sqrt5"   },
        { 1.61803398874989484820,  "phi"     },
        { 0.69314718055994530941,  "ln2"     },
        { 2.64575131106459059050,  "sqrt7"   },
        { 1.57079632679489661923,  "pi/2"    },
        { 0.57721566490153286060,  "gamma"   },
        { 1.20205690315959428539,  "zeta3"   },
        { 0.30102999566398119521,  "log10_2" },
        { 2.50290787509589282228,  "delta"   },
        { 3.35988566624317755317,  "cbrt36"  },
        { 1.32471795724474602596,  "rho"     },
    };
    int count = sizeof(IRRATIONALS) / sizeof(IRRATIONALS[0]);
    quantum_irrational q;
    int idx = rand() % count;
    q.val  = IRRATIONALS[idx].val;
    q.name = IRRATIONALS[idx].name;
    q.collapsed = 0;
    return q;
}

static inline void _quantum_irrational_get(quantum_irrational* q) {
    if (!q->collapsed) {
        *q = _quantum_irrational_all();
        q->collapsed = 1;
    }
}

static inline void _quantum_irrational_print(quantum_irrational* q) {
    _quantum_irrational_get(q);
    printf("%s ~= %.15g\n", q->name, q->val);
}

#define IRRATIONAL_ALL (_quantum_irrational_all())

// ── quantum_complex ───────────────────────────────────────────────────────────

typedef struct {
    double real;
    double imag;
    int    collapsed;
} quantum_complex;

static inline quantum_complex _quantum_complex_all() {
    _quantum_init_rng();
    quantum_complex q;
    q.real = _random_double();
    q.imag = _random_double();
    while (q.imag == 0.0) q.imag = _random_double();
    q.collapsed = 0;
    return q;
}

static inline void _quantum_complex_get(quantum_complex* q) {
    if (!q->collapsed) {
        *q = _quantum_complex_all();
        q->collapsed = 1;
    }
}

static inline void _quantum_complex_print(quantum_complex* q) {
    _quantum_complex_get(q);
    if (q->imag >= 0)
        printf("%g+%gi\n", q->real, q->imag);
    else
        printf("%g%gi\n",  q->real, q->imag);
}

#define IMAGINARY_ALL (_quantum_complex_all())

// ── Standard library functions (yo_mama_so_*) ────────────────────────────────

// yo_mama_so_nosy — read integer from stdin
static inline int yo_mama_so_nosy() {
    int val = 0;
    scanf("%d", &val);
    return val;
}

// yo_mama_so_quiet — read string from stdin (returns static buffer)
static inline char* yo_mama_so_quiet() {
    static char _ym_buf[4096];
    if (fgets(_ym_buf, sizeof(_ym_buf), stdin) != NULL) {
        size_t len = strlen(_ym_buf);
        if (len > 0 && _ym_buf[len - 1] == '\n') _ym_buf[len - 1] = '\0';
    }
    return _ym_buf;
}

// yo_mama_so_random — random integer
static inline int yo_mama_so_random() {
    _quantum_init_rng();
    return rand();
}

// yo_mama_so_round — round double to nearest int
static inline int yo_mama_so_round(double x) {
    return (int)round(x);
}

// yo_mama_so_square — x squared
static inline int yo_mama_so_square(int x) {
    return x * x;
}

// yo_mama_so_thicc — absolute value
static inline int yo_mama_so_thicc(int x) {
    return x < 0 ? -x : x;
}

// yo_mama_so_wide — square root (returns double)
static inline double yo_mama_so_wide(double x) {
    return sqrt(x);
}

// yo_mama_so_long — string length
static inline int yo_mama_so_long(const char* s) {
    return (int)strlen(s);
}

// yo_mama_so_cheap_copy — strcpy, returns dst
static inline char* yo_mama_so_cheap_copy(char* dst, const char* src) {
    return strcpy(dst, src);
}

// yo_mama_so_connected — strcat, returns dst
static inline char* yo_mama_so_connected(char* dst, const char* src) {
    return strcat(dst, src);
}

// yo_mama_so_fat_malloc — allocate memory (quantum-aware malloc)
static inline void* yo_mama_so_fat_malloc(int size) {
    void* p = malloc((size_t)size);
    if (!p) {
        fprintf(stderr, "💀 yo mama so broke she couldn't even malloc %d bytes\n", size);
        exit(1);
    }
    return p;
}

// yo_mama_so_cheap_free — free memory
static inline void yo_mama_so_cheap_free(void* ptr) {
    free(ptr);
}

// yo_mama_observe — force collapse of a quantum_int, returns its value
static inline int yo_mama_observe(quantum_int* q) {
    return _quantum_observe(q);
}

// yo_mama_collapsed — check if quantum_int has already collapsed (1=yes, 0=no)
static inline int yo_mama_collapsed(quantum_int* q) {
    return _quantum_is_collapsed(q);
}

// yo_mama_so_dumb — panic with a message and die
static inline void yo_mama_so_dumb(const char* msg) {
    fprintf(stderr, "💀 YO MAMA SO DUMB: %s\n", msg);
    exit(1);
}

#endif // QUANTUM_RUNTIME_H
